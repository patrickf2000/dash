
use std::collections::HashMap;

use crate::ast::*;
use crate::ltac;
use crate::ltac::*;

#[derive(PartialEq, Clone)]
enum DataType {
    Int,
}

#[derive(Clone)]
struct Var {
    pos : i32,
    data_type : DataType,
}

pub struct LtacBuilder {
    file : LtacFile,
    str_pos : i32,
    vars : HashMap<String, Var>,
    stack_pos : i32,
}

pub fn new_ltac_builder(name : String) -> LtacBuilder {
    LtacBuilder {
        file : LtacFile {
            name : name,
            data : Vec::new(),
            code : Vec::new(),
        },
        str_pos : 0,
        vars : HashMap::new(),
        stack_pos : 0,
    }
}

// The LTAC builder
impl LtacBuilder {

    // Builds the main LTAC file
    pub fn build_ltac(&mut self, tree : &AstTree) -> LtacFile {
        // Build functions
        self.build_functions(tree);
        
        self.file.clone()
    }

    // Converts AST functions to LTAC functions
    fn build_functions(&mut self, tree : &AstTree) {
        for func in tree.functions.iter() {
            if func.is_extern {
                let mut fc = ltac::create_instr(LtacType::Extern);
                fc.name = func.name.clone();
                self.file.code.push(fc);
            } else {
                let mut fc = ltac::create_instr(LtacType::Func);
                fc.name = func.name.clone();
                fc.arg1_val = 0;
                
                let pos = self.file.code.len();
                
                self.build_block(&func.statements);
                
                if self.vars.len() > 0 {
                    let mut stack_size = 0;
                    while stack_size < (self.stack_pos + 1) {
                        stack_size = stack_size + 16;
                    }
                    
                    fc.arg1_val = stack_size;
                }
                
                self.file.code.insert(pos, fc);
            }
        }
    }

    // Builds function body
    fn build_block(&mut self, statements : &Vec<AstStmt>) {
        for line in statements {
            match &line.stmt_type {
                AstStmtType::VarDec => self.build_var_dec(&line),
                AstStmtType::FuncCall => self.build_func_call(&line),
                AstStmtType::End => self.build_end(),
            }
        }
    }
    
    // Builds an LTAC variable declaration
    fn build_var_dec(&mut self, line : &AstStmt) {
        let name = line.name.clone();
        let data_type = &line.modifiers[0];
        
        match &data_type.mod_type {
            AstModType::Int => self.stack_pos += 4,
        }
        
        let v = Var {
            pos : self.stack_pos,
            data_type : DataType::Int,
        };
        
        self.vars.insert(name, v);
        self.build_var_assign(line);
    }
    
    // Builds an LTAC variable assignment
    fn build_var_assign(&mut self, line : &AstStmt) {
        let var : Var;
        match self.vars.get(&line.name) {
            Some(v) => var = v.clone(),
            None => return,
        }
        
        if var.data_type == DataType::Int {
            if line.args.len() == 1 {
                self.build_i32var_single_assign(&line.args, &var);
            } else {
                self.build_i32var_math(&line.args, &var);
            }
        }
    }
    
    // Builds a single int32 variable assignment
    fn build_i32var_single_assign(&mut self, args : &Vec<AstArg>, var : &Var) {
        let arg = &args[0];
        
        let mut instr = ltac::create_instr(LtacType::Mov);
        instr.arg1_type = LtacArg::Mem;
        instr.arg1_val = var.pos;
        
        match &arg.arg_type {
            AstArgType::IntL => {
                instr.arg2_type = LtacArg::I32;
                instr.arg2_val = arg.i32_val;
            },
            
            AstArgType::Id => {},
            _ => { /* TODO ERROR */ },
        }
        
        self.file.code.push(instr);
    }
    
    // Builds an int32 math assignment
    fn build_i32var_math(&mut self, args : &Vec<AstArg>, var : &Var) {
        let mut instr = ltac::create_instr(LtacType::Mov);
        instr.arg1_type = LtacArg::Reg;
        instr.arg1_val = 1;
        
        for arg in args.iter() {
            match &arg.arg_type {
                AstArgType::IntL => {
                    instr.arg2_type = LtacArg::I32;
                    instr.arg2_val = arg.i32_val;
                    self.file.code.push(instr.clone());
                },
                
                AstArgType::StringL => {},
                
                AstArgType::Id => {
                    match self.vars.get(&arg.str_val) {
                        Some(v) => instr.arg2_val = v.pos,
                        None => instr.arg2_val = 0,
                    }
                    
                    instr.arg2_type = LtacArg::Mem;
                    self.file.code.push(instr.clone());
                },
                
                AstArgType::OpAdd => {
                    instr = ltac::create_instr(LtacType::I32Add);
                    instr.arg1_type = LtacArg::Reg;
                    instr.arg1_val = 1;
                },
                
                AstArgType::OpMul => {
                    instr = ltac::create_instr(LtacType::I32Mul);
                    instr.arg1_type = LtacArg::Reg;
                    instr.arg1_val = 1;
                },
            }
        }
        
        //Store the result back
        instr = ltac::create_instr(LtacType::Mov);
        instr.arg1_type = LtacArg::Mem;
        instr.arg1_val = var.pos;
        instr.arg2_type = LtacArg::Reg;
        instr.arg2_val = 1;
        self.file.code.push(instr);
    }

    // Builds an LTAC function call
    fn build_func_call(&mut self, line : &AstStmt) {
        // Build the arguments
        for arg in line.args.iter() {
            match &arg.arg_type {
                AstArgType::IntL => {},
                
                AstArgType::StringL => {
                    let name = self.build_string(arg.str_val.clone());
                    
                    let mut push = ltac::create_instr(LtacType::PushArg);
                    push.arg1_type = LtacArg::Ptr;
                    push.arg1_sval = name;
                    self.file.code.push(push);
                },
                
                AstArgType::Id => {
                    let mut push = ltac::create_instr(LtacType::PushArg);
                    push.arg1_type = LtacArg::Mem;
                    
                    match &self.vars.get(&arg.str_val) {
                        Some(v) => push.arg1_val = v.pos,
                        None => push.arg1_val = 0,
                    }
                    
                    self.file.code.push(push);
                },
                
                _ => {},
            }
        }
        
        // Build the call
        let mut fc = ltac::create_instr(LtacType::Call);
        fc.name = line.name.clone();
        self.file.code.push(fc);
    }
    
    // Builds a void return
    // TODO: We will eventually need better handling of this
    fn build_end(&mut self) {
        let ret = ltac::create_instr(LtacType::Ret);
        self.file.code.push(ret);
    }

    // Builds a string and adds it to the data section
    fn build_string(&mut self, val : String) -> String {
        // Create the string name
        let spos = self.str_pos.to_string();
        self.str_pos = self.str_pos + 1;
        
        let mut name = "STR".to_string();
        name.push_str(&spos);
        
        // Create the data
        let string = LtacData {
            data_type : LtacDataType::StringL,
            name : name.clone(),
            val : val.clone(),
        };
        
        self.file.data.push(string);
        
        name
    }

}
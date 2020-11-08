
use std::collections::HashMap;
use std::mem;

use crate::ast::*;
use crate::ltac;
use crate::ltac::*;
use crate::syntax::*;

use crate::ltac_array::*;
use crate::ltac_flow::*;
use crate::ltac_func::*;
use crate::ltac_var::*;

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    Void,
    Byte,
    ByteDynArray,
    UByte,
    UByteDynArray,
    Short,
    UShort,
    ShortDynArray,
    UShortDynArray,
    Int,
    UInt,
    IntDynArray,
    UIntDynArray,
    Int64,
    UInt64,
    I64DynArray,
    U64DynArray,
    Float,
    Double,
    FloatDynArray,
    DoubleDynArray,
    Char,
    Str,
}

#[derive(Clone)]
pub struct Var {
    pub pos : i32,
    pub data_type : DataType,
    pub is_param : bool,
}

#[derive(Clone)]
pub struct LtacBuilder {
    pub file : LtacFile,
    pub syntax : ErrorManager,
    
    pub str_pos : i32,
    pub flt_pos : i32,
    
    // Function-related values
    pub functions : HashMap<String, DataType>,
    pub current_func : String,
    pub current_type : DataType,
    
    // Constants
    pub global_consts : HashMap<String, LtacArg>,
    
    // Variable-related values
    pub vars : HashMap<String, Var>,
    pub stack_pos : i32,
    
    // For labels and blocks
    pub block_layer : i32,
    pub label_stack : Vec<String>,
    pub top_label_stack : Vec<String>,
    pub code_stack : Vec<Vec<LtacInstr>>,
    
    //For loops
    pub loop_layer : i32,
    pub loop_labels : Vec<String>,      // Needed for continue
    pub end_labels : Vec<String>,       // Needed for break
}

pub fn new_ltac_builder(name : String, syntax : &mut ErrorManager) -> LtacBuilder {
    LtacBuilder {
        file : LtacFile {
            name : name,
            data : Vec::new(),
            code : Vec::new(),
        },
        syntax : syntax.clone(),
        str_pos : 0,
        flt_pos : 0,
        functions : HashMap::new(),
        current_func : String::new(),
        current_type : DataType::Void,
        global_consts : HashMap::new(),
        vars : HashMap::new(),
        stack_pos : 0,
        block_layer : 0,
        label_stack : Vec::new(),
        top_label_stack : Vec::new(),
        code_stack : Vec::new(),
        loop_layer : 0,
        loop_labels : Vec::new(),
        end_labels : Vec::new(),
    }
}

// The LTAC builder
impl LtacBuilder {

    // Builds the main LTAC file
    pub fn build_ltac(&mut self, tree : &AstTree) -> Result<LtacFile, ()> {
        // Cache the constants
        if !self.build_global_constants(tree) {
            self.syntax.print_errors();
            return Err(());
        }
        
        // Build functions
        if !self.build_functions(tree) {
            self.syntax.print_errors();
            return Err(());
        }
        
        Ok(self.file.clone())
    }
    
    // Builds the constant table
    fn build_global_constants(&mut self, tree : &AstTree) -> bool {
        for c in tree.constants.iter() {
            let data_type = ast_to_datatype(&c.data_type);
            let val = &c.value;
            let arg : LtacArg;
            
            match data_type {
                DataType::Byte => {
                    match &val.arg_type {
                        AstArgType::ByteL => arg = LtacArg::Byte(val.u8_val as i8),
                        AstArgType::IntL => arg = LtacArg::Byte(val.u64_val as i8),
                        
                        _ => return false,
                    }
                },
                
                DataType::UByte => {
                    match &val.arg_type {
                        AstArgType::ByteL => arg = LtacArg::UByte(val.u8_val),
                        AstArgType::IntL => arg = LtacArg::UByte(val.u64_val as u8),
                        
                        _ => return false,
                    }
                },
                
                DataType::Short => {
                    match &val.arg_type {
                        AstArgType::ShortL => arg = LtacArg::I16(val.u16_val as i16),
                        AstArgType::IntL => arg = LtacArg::I16(val.u64_val as i16),
                        
                        _ => return false,
                    }
                },
                
                DataType::UShort => {
                    match &val.arg_type {
                        AstArgType::ShortL => arg = LtacArg::U16(val.u16_val as u16),
                        AstArgType::IntL => arg = LtacArg::U16(val.u64_val as u16),
                        
                        _ => return false,
                    }
                },
                
                DataType::Int => {
                    if val.arg_type == AstArgType::IntL {
                        arg = LtacArg::I32(val.u64_val as i32);
                    } else {
                        return false;
                    }
                },
                
                DataType::UInt => {
                    if val.arg_type == AstArgType::IntL {
                        arg = LtacArg::U32(val.u64_val as u32);
                    } else {
                        return false;
                    }
                },
                
                DataType::Int64 => {
                    if val.arg_type == AstArgType::IntL {
                        arg = LtacArg::I64(val.u64_val as i64);
                    } else {
                        return false;
                    }
                },
                
                DataType::UInt64 => {
                    if val.arg_type == AstArgType::IntL {
                        arg = LtacArg::U64(val.u64_val);
                    } else {
                        return false;
                    }
                },
                
                /*DataType::Float => {},
                
                DataType::Double => {},
                
                DataType::Char => {},
                DataType::Str => {},*/
                
                _ => {
                    return false;
                },
            }
            
            self.global_consts.insert(c.name.clone(), arg);
        }
        
        true
    }

    // Converts AST functions to LTAC functions
    // Make two passes; the first collects information, and the second does construction
    fn build_functions(&mut self, tree : &AstTree) -> bool {
        // Collect information- for now, only names
        for func in tree.functions.iter() {
            let name = func.name.clone();
            let mut func_type = DataType::Void;
            
            if func.modifiers.len() > 0 {
                let func_mod = func.modifiers.first().unwrap();
                func_type = ast_to_datatype(&func_mod);
            }
        
            self.functions.insert(name, func_type);
        }
        
        // Build everything
        for func in tree.functions.iter() {
            if func.is_extern {
                let mut fc = ltac::create_instr(LtacType::Extern);
                fc.name = func.name.clone();
                self.file.code.push(fc);
            } else {
                // Set the current function and type
                self.current_func = func.name.clone();
                
                match self.functions.get(&self.current_func) {
                    Some(t) => self.current_type = t.clone(),
                    None => self.current_type = DataType::Void,
                };
            
                // Create the function and load the arguments
                let mut fc = ltac::create_instr(LtacType::Func);
                fc.name = func.name.clone();
                fc.arg1_val = 0;
                
                let pos = self.file.code.len();        // The position of the code before we add anything
                let mut arg_pos = 1;                   // Needed for function arguments
                let mut flt_arg_pos = 1;               // Needed for floating-point function arguments
                
                for arg in func.args.iter() {
                    let ret = build_var_dec(self, &arg, arg_pos, flt_arg_pos);
                    arg_pos = ret.1;
                    flt_arg_pos = ret.2;
                }
                
                // Build the body and calculate the stack size
                if !self.build_block(&func.statements) {
                    return false;
                }
                
                if self.vars.len() > 0 {
                    let mut stack_size = 0;
                    while stack_size < (self.stack_pos + 1) {
                        stack_size = stack_size + 16;
                    }
                    
                    fc.arg1_val = stack_size;
                    fc.arg2_val = self.stack_pos;    // At this point, only needed by Arm
                }
                
                self.file.code.insert(pos, fc);
                self.stack_pos = 0;
                self.vars.clear();
            }
        }
        
        true
    }

    // Builds function body
    fn build_block(&mut self, statements : &Vec<AstStmt>) -> bool {
        let mut code = true;
    
        for line in statements {
            match &line.stmt_type {
                AstStmtType::VarDec => code = build_var_dec(self, &line, 0, 0).0,
                AstStmtType::VarAssign => code = build_var_assign(self, &line),
                AstStmtType::ArrayAssign => code = build_array_assign(self, &line),
                AstStmtType::If => build_cond(self, &line),
                AstStmtType::Elif => build_cond(self, &line),
                AstStmtType::Else => build_cond(self, &line),
                AstStmtType::While => build_while(self, &line),
                AstStmtType::Break => build_break(self),
                AstStmtType::Continue => build_continue(self),
                AstStmtType::FuncCall => code = build_func_call(self, &line),
                AstStmtType::Return => code = build_return(self, &line),
                AstStmtType::Exit => code = build_exit(self, &line),
                AstStmtType::End => code = build_end(self, &line),
            }
            
            if !code {
                break;
            }
        }
        
        code
    }

    // Builds a string and adds it to the data section
    pub fn build_string(&mut self, val : String) -> String {
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
    
    // Builds a float literal and adds it to the data section
    // https://stackoverflow.com/questions/40030551/how-to-decode-and-encode-a-float-in-rust
    pub fn build_float(&mut self, v : f64, is_double : bool, negate_next : bool) -> String {
        // Create the float name
        let fpos = self.flt_pos.to_string();
        self.flt_pos = self.flt_pos + 1;
        
        let mut name = "FLT".to_string();
        name.push_str(&fpos);
        
        let value : String;
        let data_type : LtacDataType;
        
        let mut val = v;
        if negate_next {
            val = -v;
        }
        
        if is_double {
            data_type = LtacDataType::DoubleL;
            let as_int : u64 = unsafe { mem::transmute(val) };
            value = as_int.to_string();
        } else {
            data_type = LtacDataType::FloatL;
            let as_int: u32 = unsafe { mem::transmute(val as f32) };
            value = as_int.to_string();
        }
        
        // Create the data
        let flt = LtacData {
            data_type : data_type,
            name : name.clone(),
            val : value,
        };
        
        self.file.data.push(flt);
        
        name
    }

}

pub fn ast_to_datatype(ast_mod : &AstMod) -> DataType {
    match &ast_mod.mod_type {
        AstModType::Byte => return DataType::Byte,
        AstModType::ByteDynArray => return DataType::ByteDynArray,
        AstModType::UByte => return DataType::UByte,
        AstModType::UByteDynArray => return DataType::UByteDynArray,
        AstModType::Short => return DataType::Short,
        AstModType::UShort => return DataType::UShort,
        AstModType::ShortDynArray => return DataType::ShortDynArray,
        AstModType::UShortDynArray => return DataType::UShortDynArray,
        AstModType::Int => return DataType::Int,
        AstModType::UInt => return DataType::UInt,
        AstModType::IntDynArray => return DataType::IntDynArray,
        AstModType::UIntDynArray => return DataType::UIntDynArray,
        AstModType::Int64 => return DataType::Int64,
        AstModType::UInt64 => return DataType::UInt64,
        AstModType::I64DynArray => return DataType::I64DynArray,
        AstModType::U64DynArray => return DataType::U64DynArray,
        AstModType::Float => return DataType::Float,
        AstModType::Double => return DataType::Double,
        AstModType::FloatDynArray => return DataType::FloatDynArray,
        AstModType::DoubleDynArray => return DataType::DoubleDynArray,
        AstModType::Char => return DataType::Char,
        AstModType::Str => return DataType::Str,
        
        // Do we need an error here? Really, it should never get to this pointer
        AstModType::None => return DataType::Void,
    }
}

// Returns a move statement for a given type
pub fn mov_for_type(data_type :& DataType) -> LtacInstr {
    let mut instr = ltac::create_instr(LtacType::Mov);
    
    match data_type {
        DataType::Byte | DataType::ByteDynArray => instr = ltac::create_instr(LtacType::MovB),
        DataType::UByte | DataType::UByteDynArray => instr = ltac::create_instr(LtacType::MovUB),
        
        DataType::Short | DataType::ShortDynArray => instr = ltac::create_instr(LtacType::MovW),
        DataType::UShort | DataType::UShortDynArray => instr = ltac::create_instr(LtacType::MovUW),
        
        DataType::Int | DataType::IntDynArray => instr = ltac::create_instr(LtacType::Mov),
        DataType::UInt | DataType::UIntDynArray => instr = ltac::create_instr(LtacType::MovU),
        
        DataType::Int64 | DataType::I64DynArray => instr = ltac::create_instr(LtacType::MovQ),
        DataType::UInt64 | DataType::U64DynArray => instr = ltac::create_instr(LtacType::MovUQ),
        
        DataType::Float | DataType::FloatDynArray => instr = ltac::create_instr(LtacType::MovF32),
        DataType::Double | DataType::DoubleDynArray => instr = ltac::create_instr(LtacType::MovF64),
        
        DataType::Char | DataType::Str => instr = ltac::create_instr(LtacType::MovB),
        
        _ => {},
    }
    
    instr
}

// Returns a register for a given type
pub fn reg_for_type(data_type : &DataType, reg_no : i32) -> LtacArg {
    let mut arg = LtacArg::Reg32(reg_no);
    
    match data_type {
        DataType::Byte | DataType::ByteDynArray => arg = LtacArg::Reg8(reg_no),
        DataType::UByte | DataType::UByteDynArray => arg = LtacArg::Reg8(reg_no),
        
        DataType::Short | DataType::ShortDynArray => arg = LtacArg::Reg16(reg_no),
        DataType::UShort | DataType::UShortDynArray => arg = LtacArg::Reg16(reg_no),
        
        DataType::Int | DataType::IntDynArray => arg = LtacArg::Reg32(reg_no),
        DataType::UInt | DataType::UIntDynArray => arg = LtacArg::Reg32(reg_no),
        
        DataType::Int64 | DataType::I64DynArray => arg = LtacArg::Reg64(reg_no),
        DataType::UInt64 | DataType::U64DynArray => arg = LtacArg::Reg64(reg_no),
        
        DataType::Float | DataType::FloatDynArray => arg = LtacArg::FltReg(reg_no),
        DataType::Double | DataType::DoubleDynArray => arg = LtacArg::FltReg64(reg_no),
        
        DataType::Char | DataType::Str => arg = LtacArg::Reg8(reg_no),
        
        _ => {},
    }
    
    arg
}

// Returns a ldarg statement for a given type
pub fn ldarg_for_type(data_type : &DataType, dest : LtacArg, pos : i32) -> LtacInstr {
    let mut arg = ltac::create_instr(LtacType::None);
    
    match data_type {
        DataType::Byte => arg = ltac::create_instr(LtacType::LdArgI8),
        DataType::UByte => arg = ltac::create_instr(LtacType::LdArgU8),
        
        DataType::Short => arg = ltac::create_instr(LtacType::LdArgI16),
        DataType::UShort => arg = ltac::create_instr(LtacType::LdArgU16),
        
        DataType::Int => arg = ltac::create_instr(LtacType::LdArgI32),
        DataType::UInt => arg = ltac::create_instr(LtacType::LdArgU32),
        
        DataType::Int64 => arg = ltac::create_instr(LtacType::LdArgI64),
        DataType::UInt64 => arg = ltac::create_instr(LtacType::LdArgU64),
        
        DataType::Float => arg = ltac::create_instr(LtacType::LdArgF32),
        DataType::Double => arg = ltac::create_instr(LtacType::LdArgF64),
        
        DataType::Str | DataType::IntDynArray => arg = ltac::create_instr(LtacType::LdArgPtr),
        
        _ => return arg,
    }
    
    arg.arg1 = dest;
    arg.arg2_val = pos;
    
    arg
}


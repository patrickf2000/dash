
// This file is part of the Lila compiler
// Copyright (C) 2020 Patrick Flynn
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; version 2.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use std::collections::HashMap;

use crate::lex::*;

// Represents AST statement types
#[derive(PartialEq, Clone)]
pub enum AstStmtType {
    VarDec,
    VarAssign,
    ArrayAssign,
    If,
    Elif,
    Else,
    While,
    Break,
    Continue,
    FuncCall,
    Return,
    Exit,
    End,
}

// Represents AST argument types
#[derive(PartialEq, Clone)]
pub enum AstArgType {
    None,
    ByteL,
    ShortL,
    IntL,
    FloatL,
    CharL,
    StringL,
    Id,
    LdArg,
    Array,
    OpRParen,
    OpLParen,
    OpNeg,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpEq,
    OpNeq,
    OpLt,
    OpLte,
    OpGt,
    OpGte,
    OpNot,
    OpAnd,
    OpOr,
    OpXor,
    OpLeftShift,
    OpRightShift,
}

// Represents modifiers
#[derive(PartialEq, Clone)]
pub enum AstModType {
    None,
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
    Enum(String),
}

// Represents the top of an AST tree
pub struct AstTree {
    pub file_name : String,
    pub module : String,
    pub functions : Vec<AstFunc>,
    pub constants : Vec<AstConst>,
}

// Represents a function in a tree
pub struct AstFunc {
    pub name : String,
    pub is_extern : bool,
    pub statements : Vec<AstStmt>,
    pub args : Vec<AstStmt>,
    pub modifiers : Vec<AstMod>,
    pub enums : Vec<AstEnum>,
    pub line : String,
}

// Represents a constant
pub struct AstConst {
    pub name : String,
    pub data_type : AstMod,
    pub value : AstArg,
    
    pub line : String,
    pub line_no : i32,
}

// Represents an enumeration
pub struct AstEnum {
    pub name : String,
    pub data_type : AstMod,
    pub values : HashMap<String, i32>,
}

// Represents a statement
#[derive(Clone)]
pub struct AstStmt {
    pub stmt_type : AstStmtType,
    pub name : String,
    
    pub sub_args : Vec<AstArg>,
    pub args : Vec<AstArg>,
    pub modifiers : Vec<AstMod>,
    
    pub line : String,
    pub line_no : i32,
}

// Represents an argument
// Arguments are constants, variables, operators, etc
#[derive(Clone)]
pub struct AstArg {
    pub arg_type : AstArgType,
    pub str_val : String,
    pub char_val : char,
    pub u8_val : u8,
    pub u16_val : u16,
    pub u64_val : u64,
    pub f64_val : f64,
    
    pub sub_args : Vec<AstArg>,
    pub sub_modifiers : Vec<AstMod>,
}

// Represents an statement modifier
#[derive(Clone)]
pub struct AstMod {
    pub mod_type : AstModType,
}

// Tree implementation
impl AstTree {
    pub fn print(&self) {
        print!("Tree: ");
        if self.module.len() > 0 {
            print!("{}.", self.module);
        }
        
        println!("{}", self.file_name);
        
        for constant in self.constants.iter() {
            constant.print(false);
        }
    
        for func in self.functions.iter() {
            func.print();
        }
    }
}

// Constant implementation
impl AstConst {
    pub fn print(&self, is_global : bool) {
        if is_global {
            print!("CONST ");
        } else {
            print!("  CONST ");
        }
        
        print!("{} ", self.name);
        
        match &self.data_type.mod_type {
            AstModType::Byte => print!("Byte "),
            AstModType::UByte => print!("UByte "),
            AstModType::Short => print!("Short "),
            AstModType::UShort => print!("UShort "),
            AstModType::Int => print!("Int "),
            AstModType::UInt => print!("UInt "),
            AstModType::Int64 => print!("Int64 "),
            AstModType::UInt64 => print!("UInt64 "),
            AstModType::Float => print!("Float "),
            AstModType::Double => print!("Double "),
            AstModType::Char => print!("Char "),
            AstModType::Str => print!("Str "),
            
            _ => print!("NONE"),
        }
        
        self.value.print();
        
        println!("");
    }
}

// Enum implementation
impl AstEnum {
    pub fn print(&self) {
        print!("    ");
        print!("ENUM {} ", self.name);
        
        for (name,val) in self.values.iter() {
            print!("{}({}) ", name, val);
        }
        
        println!("");
    }
}

// Function implementation
impl AstFunc {
    pub fn print(&self) {
        print!("  ");
        if self.is_extern {
            print!("EXTERN ");
        }
        
        print!("FUNC {}", self.name);
        
        for m in self.modifiers.iter() {
            m.print(true);
        }
        
        println!("");
        
        for arg in self.args.iter() {
            arg.print(true);
        }
        
        for e in self.enums.iter() {
            e.print();
        }
        
        for stmt in self.statements.iter() {
            stmt.print(false);
        }
    }
}


// Statement implementation
impl AstStmt {
    pub fn print(&self, is_arg : bool) {
        print!("    ");
        
        if is_arg {
            print!("FUNC_ARG ");
        }
        
        match &self.stmt_type {
            AstStmtType::VarDec => println!("VAR DEC {}", self.name),
            AstStmtType::VarAssign => println!("VAR ASSIGN {}", self.name),
            AstStmtType::ArrayAssign => println!("ARRAY ASSIGN {}", self.name),
            AstStmtType::If => println!("IF"),
            AstStmtType::Elif => println!("ELIF"),
            AstStmtType::Else => println!("ELSE"),
            AstStmtType::While => println!("WHILE"),
            AstStmtType::Break => println!("BREAK"),
            AstStmtType::Continue => println!("CONTINUE"),
            AstStmtType::FuncCall => println!("FUNC CALL {}", self.name),
            AstStmtType::Return => println!("RETURN"),
            AstStmtType::Exit => println!("EXIT"),
            AstStmtType::End => println!("END"),
        }
        
        for m in self.modifiers.iter() {
            m.print(false);
        }
        
        if self.args.len() > 0 {
            print!("        ARG ");
            
            for arg in self.args.iter() {
                arg.print();
            }
            
            println!("");
        }
        
        if self.sub_args.len() > 0 {
            print!("        SUB_ARG ");
            
            for arg in self.sub_args.iter() {
                arg.print();
            }
            
            println!("");
        }
    }
}

// Argument implementation
impl AstArg {
    pub fn print(&self) {
        match &self.arg_type {
            AstArgType::None => print!("?? "),
            AstArgType::ByteL => print!("{} ", self.u8_val),
            AstArgType::ShortL => print!("{} ", self.u16_val),
            AstArgType::IntL => print!("{} ", self.u64_val),
            AstArgType::FloatL => print!("{} ", self.f64_val),
            AstArgType::CharL => print!("\'{}\' ", self.char_val),
            AstArgType::StringL => print!("\"{}\" ", self.str_val),
            AstArgType::Id => print!("{} ", self.str_val),
            AstArgType::LdArg => print!("LDARG "),
            AstArgType::Array => print!("ARRAY "),
            AstArgType::OpLParen => print!("("),
            AstArgType::OpRParen => print!(")"),
            AstArgType::OpNeg => print!("-"),
            AstArgType::OpAdd => print!("+ "),
            AstArgType::OpSub => print!("- "),
            AstArgType::OpMul => print!("* "),
            AstArgType::OpDiv => print!("/ "),
            AstArgType::OpMod => print!("% "),
            AstArgType::OpEq => print!("== "),
            AstArgType::OpNeq => print!("!= "),
            AstArgType::OpLt => print!("< "),
            AstArgType::OpLte => print!("<= "),
            AstArgType::OpGt => print!("> "),
            AstArgType::OpGte => print!(">= "),
            AstArgType::OpNot => print!("! "),
            AstArgType::OpAnd => print!("& "),
            AstArgType::OpOr => print!("| "),
            AstArgType::OpXor => print!("^ "),
            AstArgType::OpLeftShift => print!("<< "),
            AstArgType::OpRightShift => print!(">> "),
        }
        
        if self.sub_args.len() > 0 {
            print!("(");
            for arg in self.sub_args.iter() {
                arg.print();
            }
            print!(") ");
        }
        
        if self.sub_modifiers.len() > 0 {
            print!("[MOD ");
            for arg in self.sub_modifiers.iter() {
                arg.print(true);
            }
            print!("] ");
        }
    }
}

// Modifier implementation
impl AstMod {
    pub fn print(&self, is_func : bool) {
        if is_func {
            print!(" (");
        } else {
            print!("        MOD ");
        }
        
        match &self.mod_type {
            AstModType::None => print!("NONE"),
            AstModType::Byte => print!("Byte"),
            AstModType::ByteDynArray => print!("ByteDynArr"),
            AstModType::UByte => print!("UByte"),
            AstModType::UByteDynArray => print!("UByteDynArr"),
            AstModType::Short => print!("Short"),
            AstModType::UShort => print!("UShort"),
            AstModType::ShortDynArray => print!("ShortDynArr"),
            AstModType::UShortDynArray => print!("UShortDynArr"),
            AstModType::Int => print!("Int"),
            AstModType::UInt => print!("UInt"),
            AstModType::IntDynArray => print!("IntDynArr"),
            AstModType::UIntDynArray => print!("UIntDynArr"),
            AstModType::Int64 => print!("Int64"),
            AstModType::UInt64 => print!("UInt64"),
            AstModType::I64DynArray => print!("I64DynArray"),
            AstModType::U64DynArray => print!("U64DynArray"),
            AstModType::Float => print!("Float"),
            AstModType::Double => print!("Double"),
            AstModType::FloatDynArray => print!("FloatDynArray"),
            AstModType::DoubleDynArray => print!("DoubleDynArray"),
            AstModType::Char => print!("Char"),
            AstModType::Str => print!("Str"),
            AstModType::Enum(ref val) => print!("Enum({})", val),
        }
        
        if is_func {
            print!(")");
        } else {
            println!("");
        }
    }
}

// Helper functions
pub fn create_extern_func(name : String) -> AstFunc {
    AstFunc {
        name : name,
        is_extern : true,
        statements : Vec::new(),
        args : Vec::new(),
        modifiers : Vec::new(),
        enums : Vec::new(),
        line : String::new(),
    }
}

pub fn create_func(name : String) -> AstFunc {
    AstFunc {
        name : name,
        is_extern : false,
        statements : Vec::new(),
        args : Vec::new(),
        modifiers : Vec::new(),
        enums : Vec::new(),
        line : String::new(),
    }
}

pub fn create_stmt(stmt_type : AstStmtType, scanner : &mut Lex) -> AstStmt {
    AstStmt {
        stmt_type : stmt_type,
        name : String::new(),
        
        sub_args : Vec::new(),
        args : Vec::new(),
        modifiers : Vec::new(),
        
        line_no : scanner.get_line_no(),
        line : scanner.get_current_line(),
    }
}

// This should only be used by the LTAC layer; do not use unless absolutely necessary
pub fn create_orphan_stmt(stmt_type : AstStmtType) -> AstStmt {
    AstStmt {
        stmt_type : stmt_type,
        name : String::new(),
        
        sub_args : Vec::new(),
        args : Vec::new(),
        modifiers : Vec::new(),
        
        line_no : 0,
        line : String::new(),
    }
}

pub fn add_stmt(tree : &mut AstTree, stmt : AstStmt) {
    let top_func_pos = tree.functions.len() - 1;
    let top_func = &mut tree.functions[top_func_pos];
    &top_func.statements.push(stmt);
}

pub fn add_func_enum(tree : &mut AstTree, new_enum : AstEnum) {
    let top_func_pos = tree.functions.len() - 1;
    let top_func = &mut tree.functions[top_func_pos];
    &top_func.enums.push(new_enum);
}

pub fn enum_exists(tree : &mut AstTree, to_find : String) -> bool {
    let top_func_pos = tree.functions.len() - 1;
    let top_func = &mut tree.functions[top_func_pos];
    
    for e in top_func.enums.iter() {
        if e.name == to_find {
            return true;
        }
    }
    
    false
}

pub fn create_byte(val : u8) -> AstArg {
    AstArg {
        arg_type : AstArgType::ByteL,
        str_val : String::new(),
        char_val : '\0',
        u8_val : val,
        u16_val : 0,
        u64_val : 0,
        f64_val : 0.0,
        sub_args : Vec::new(),
        sub_modifiers : Vec::new(),
    }
}

pub fn create_short(val : u16) -> AstArg {
    AstArg {
        arg_type : AstArgType::ShortL,
        str_val : String::new(),
        char_val : '\0',
        u8_val : 0,
        u16_val : val,
        u64_val : 0,
        f64_val : 0.0,
        sub_args : Vec::new(),
        sub_modifiers : Vec::new(),
    }
}

pub fn create_int(val : u64) -> AstArg {
    AstArg {
        arg_type : AstArgType::IntL,
        str_val : String::new(),
        char_val : '\0',
        u8_val : 0,
        u16_val : 0,
        u64_val : val,
        f64_val : 0.0,
        sub_args : Vec::new(),
        sub_modifiers : Vec::new(),
    }
}

pub fn create_float(val : f64) -> AstArg {
    AstArg {
        arg_type : AstArgType::FloatL,
        str_val : String::new(),
        char_val : '\0',
        u8_val : 0,
        u16_val : 0,
        u64_val : 0,
        f64_val : val,
        sub_args : Vec::new(),
        sub_modifiers : Vec::new(),
    }
}

pub fn create_char(val : char) -> AstArg {
    AstArg {
        arg_type : AstArgType::CharL,
        str_val : String::new(),
        char_val : val,
        u8_val : 0,
        u16_val : 0,
        u64_val : 0,
        f64_val : 0.0,
        sub_args : Vec::new(),
        sub_modifiers : Vec::new(),
    }
}

pub fn create_string(val : String) -> AstArg {
    AstArg {
        arg_type : AstArgType::StringL,
        str_val : val,
        char_val : '\0',
        u8_val : 0,
        u16_val : 0,
        u64_val : 0,
        f64_val : 0.0,
        sub_args : Vec::new(),
        sub_modifiers : Vec::new(),
    }
}

pub fn create_arg(arg_type : AstArgType) -> AstArg {
    AstArg {
        arg_type : arg_type,
        str_val : String::new(),
        char_val : '\0',
        u8_val : 0,
        u16_val : 0,
        u64_val : 0,
        f64_val : 0.0,
        sub_args : Vec::new(),
        sub_modifiers : Vec::new(),
    }
}


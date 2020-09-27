
// Import what we need
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::ast;
use crate::ast::*;
use crate::lex::{Token, Lex, create_lex};

use crate::ast_func::*;
use crate::ast_utils::*;

// The AST building function
// This function opens the file and reads a line; 
// the line is then passed to another function which lexically analyzes
// it and builds an AST node.
//
// In Quik, each line is a self-contained expression; as a result, we read a line
// and then lexically analyze and build an AST node from it
//
pub fn build_ast(path : String, name : String) -> AstTree {   
    let mut tree = AstTree {
        file_name : name,
        functions : Vec::new(),
    };
    
    // Open the file
    let file = File::open(&path)
        .expect("Error: Unable to open input file.");
    let reader = BufReader::new(file);
    
    // Read the thing line by line
    for line in reader.lines() {
        let mut current = line.unwrap();
        current = current.trim().to_string();
        
        if current.len() == 0 {
            continue;
        }
        
        build_line(current, &mut tree);
    }
    
    tree
}

// Converts a line to an AST node
fn build_line(line : String, tree : &mut AstTree) {
    let mut analyzer = create_lex(line);
    analyzer.tokenize();
    
    // Get the first token
    let token = analyzer.get_token();
    
    match token {
        Token::Extern => build_extern(&mut analyzer, tree),
        Token::Func => build_func(&mut analyzer, tree),
        Token::Return => build_return(&mut analyzer, tree),
        Token::End => build_end(tree),
        Token::Int => build_i32var_dec(&mut analyzer, tree),
        Token::TStr => println!("TStr: {:?}", token),
        Token::Id(ref val) => build_id(&mut analyzer, tree, val.to_string()),
        Token::If => build_cond(&mut analyzer, tree, Token::If),
        Token::Elif => build_cond(&mut analyzer, tree, Token::Elif),
        Token::Else => build_cond(&mut analyzer, tree, Token::Else),
        _ => println!("Error: {:?}", token),
    }
}

// Builds an integer variable declaration
fn build_i32var_dec(scanner : &mut Lex, tree : &mut AstTree) {
    let mut var_dec = ast::create_stmt(AstStmtType::VarDec);
        
    let data_type = AstMod {
        mod_type : AstModType::Int,
    };
    var_dec.modifiers.push(data_type);
    
    // Gather information
    // The first token should be the name
    let mut token = scanner.get_token();
    
    // TODO: Better syntax error
    match token {
        Token::Id(ref val) => var_dec.name = val.to_string(),
        _ => println!("Error: Invalid variable name-> {:?}", token),
    }
    
    // The next token should be the assign operator
    token = scanner.get_token();
    
    // TODO: Better syntax error
    match token {
        Token::Assign => {},
        _ => println!("Error: Missing assignment"),
    }
    
    // Build the remaining arguments
    build_args(scanner, &mut var_dec, Token::Eof);

    // Add the declaration
    ast::add_stmt(tree, var_dec);
}

// Handles cases when an identifier is the first token
fn build_id(scanner : &mut Lex, tree : &mut AstTree, id_val : String) {
    // If the next token is an assignment, we have a variable assignment
    // If the next token is a parantheses, we have a function call
    let token = scanner.get_token();
    
    // TODO: Better assignment
    match token {
        Token::Assign => {},
        Token::LParen => build_func_call(scanner, tree, id_val),
        _ => println!("Invalid declaration or assignment"),
    }
}

// Builds conditional statements
fn build_cond(scanner : &mut Lex, tree : &mut AstTree, cond_type : Token) {
    let mut ast_cond_type : AstStmtType = AstStmtType::If;
    match cond_type {
        Token::If => ast_cond_type = AstStmtType::If,
        Token::Elif => ast_cond_type = AstStmtType::Elif,
        Token::Else => ast_cond_type = AstStmtType::Else,
        _ => {},
    }
    
    let mut cond = ast::create_stmt(ast_cond_type);
    
    // Build arguments
    if cond_type != Token::Else {
        build_args(scanner, &mut cond, Token::Eof);
    }
    
    // Add the conditional
    ast::add_stmt(tree, cond);
}


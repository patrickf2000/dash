use std::fs::File;
use std::io::{Write, BufWriter};

use parser::ltac::LtacInstr;

// Builds an extern declaration
pub fn aarch64_build_extern(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(".extern ");
    line.push_str(&code.name);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[ARCH64_build_extern] Write failed.");
}

// Builds a function declaration
pub fn aarch64_build_func(writer : &mut BufWriter<File>, code : &LtacInstr) -> i32 {
    let name = &code.name;
    
    let mut stack_size = code.arg1_val;
    while (stack_size - code.arg2_val) < 24 {
        stack_size += 16;
    }
    
    let mut line = "\n.global ".to_string();
    line.push_str(name);
    line.push_str("\n");
    line.push_str(name);
    line.push_str(":\n");
    
    // Set up the stack
    line.push_str("  stp x29, x30, [sp, -");
    line.push_str(&stack_size.to_string());
    line.push_str("]!\n");
    line.push_str("  mov x29, sp\n\n");
    
    writer.write(&line.into_bytes())
        .expect("[ARCH64_build_func] Write failed.");
        
    stack_size
}

// Builds a function return
pub fn aarch64_build_ret(writer : &mut BufWriter<File>, stack_size : i32) {
    let mut line = "\n  ".to_string();
    line.push_str("ldp x29, x30, [sp], ");
    line.push_str(&stack_size.to_string());
    line.push_str("\n");
    line.push_str("  ret\n");
    
    writer.write(&line.into_bytes())
        .expect("[AARCH64_build_ret] Write failed.");
}
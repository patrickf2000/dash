use std::io::{BufWriter, Write};
use std::fs::File;

use parser::ltac::{LtacInstr};

// Builds an extern declaration
pub fn aarch64_build_extern(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(".extern ");
    line.push_str(&code.name);
    line.push_str("\n");
    
    writer.write(&line.into_bytes())
        .expect("[AArch64_build_extern] Write failed.");
}

// Builds a label
pub fn aarch64_build_label(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(&code.name);
    line.push_str(":\n");
    
    writer.write(&line.into_bytes())
        .expect("[AArch64_build_label] Write failed.");
}

// Builds a function
pub fn aarch64_build_func(writer : &mut BufWriter<File>, code : &LtacInstr) {
    let mut line = String::new();
    line.push_str(".global ");
    line.push_str(&code.name);
    line.push_str("\n");
    
    line.push_str(&code.name);
    line.push_str(":\n");
    
    line.push_str("  stp x29, x30, [sp, -");
    line.push_str(&code.arg1_val.to_string());
    line.push_str("]!\n");

    line.push_str("  mov x29, sp\n\n");

    writer.write(&line.into_bytes())
        .expect("[AArch64_build_func] Write failed.");
}

// Builds a return statement
pub fn aarch64_build_ret(writer : &mut BufWriter<File>, stack_size : i32) {
    let mut line = String::new();
    line.push_str("  ldp x29, x30, [sp], ");
    line.push_str(&stack_size.to_string());
    line.push_str("\n");

    line.push_str("  ret\n");

    writer.write(&line.into_bytes())
        .expect("[AArch64_build_ret] Write failed.");
}

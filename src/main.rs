mod qbe;
use std::{
    ffi::{c_char, CString},
    process,
};

use qbe::{codegen, fclose, fopen};

//// We need this wrapper to avoid unsafe code in main func and to keep codegen step clean.
#[allow(dead_code, non_camel_case_types)]
enum QBE_TARGETS {
    AMD64_SYSV,
    AMD64_APPLE,
    ARM64,
    ARM64_APPPLE,
    RISV64,
}

impl Into<qbe::Target> for QBE_TARGETS {
    fn into(self) -> qbe::Target {
        unsafe {
            match self {
                QBE_TARGETS::AMD64_SYSV => qbe::T_amd64_sysv,
                QBE_TARGETS::AMD64_APPLE => qbe::T_amd64_apple,
                QBE_TARGETS::ARM64 => qbe::T_arm64,
                QBE_TARGETS::ARM64_APPPLE => qbe::T_arm64_apple,
                QBE_TARGETS::RISV64 => qbe::T_rv64,
            }
        }
    }
}
////

fn invoke_assembler(assembly_file_path: &str, object_file_path: &str) -> () {
    match process::Command::new("as")
        .arg("-c")
        .arg(assembly_file_path)
        .arg("-o")
        .arg(object_file_path)
        .spawn()
    {
        Err(e) => panic!("Failed to run assembler: {:#?}", e),
        Ok(_) => (),
    };
}

fn run_qbe_codegen(input_file_name: &str, output_file_name: &str, target: QBE_TARGETS) {
    let input_name_c = CString::new(input_file_name).unwrap();
    let output_name_c = CString::new(output_file_name).unwrap();

    let read_mode = CString::new("r").unwrap();
    let write_mode = CString::new("w").unwrap();

    unsafe {
        let input_stream = fopen(input_name_c.as_ptr(), read_mode.as_ptr());
        let output_stream = fopen(output_name_c.as_ptr(), write_mode.as_ptr());

        codegen(input_stream, input_name_c.as_ptr() as *mut c_char, output_stream, target.into());

        fclose(output_stream);
        fclose(input_stream);
    }
}

fn main() -> () {
    let input_name = "/tmp/output.ssa";
    let output_name = "/tmp/out.s";
    let object_name = "/tmp/out.o";

    run_qbe_codegen(input_name, output_name, QBE_TARGETS::AMD64_SYSV);
    println!("Codegen complete!");

    invoke_assembler(output_name, object_name);
    println!("Object file assembly completed!");
}

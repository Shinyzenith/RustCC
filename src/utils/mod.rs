use crate::qbe;

use std::{ffi::CString, fs, path::Path, process};
use uuid::Uuid;

//// We need this wrapper to avoid unsafe code in main func and to keep codegen step clean.
#[allow(dead_code, non_camel_case_types)]
pub enum QBE_TARGETS {
    AMD64_SYSV,
    AMD64_APPLE,
    ARM64,
    ARM64_APPPLE,
    RISV64,
}

impl From<QBE_TARGETS> for qbe::Target {
    fn from(val: QBE_TARGETS) -> Self {
        unsafe {
            match val {
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

pub fn invoke_assembler(assembly_file_path: &str, object_file_path: &str) {
    if let Err(e) = process::Command::new("as")
        .arg("-c")
        .arg(assembly_file_path)
        .arg("-o")
        .arg(object_file_path)
        .spawn()
    {
        panic!("Failed to run assembler: {:#?}", e);
    };
}

pub fn run_qbe_codegen(input_file_name: &str, output_file_name: &str, target: QBE_TARGETS) {
    let input_name_c = CString::new(input_file_name).unwrap();
    let output_name_c = CString::new(output_file_name.clone()).unwrap();

    let read_mode = CString::new("r").unwrap();
    let write_mode = CString::new("w").unwrap();

    unsafe {
        let input_stream = qbe::fopen(input_name_c.as_ptr(), read_mode.as_ptr());
        let output_stream = qbe::fopen(output_name_c.as_ptr(), write_mode.as_ptr());

        qbe::codegen(input_stream, output_stream, target.into());

        qbe::fclose(output_stream);
        qbe::fclose(input_stream);
    }
}

pub fn init_build_directory() -> String {
    let mut build_dir = format!("/tmp/rustcc-build-{}/", Uuid::new_v4());

    while Path::new(build_dir.as_str()).exists() {
        println!("Build Directory: {} already exists. Retrying....", build_dir);
        build_dir = format!("/tmp/rustcc-build-{}/", Uuid::new_v4());
    }
    if let Err(e) = fs::create_dir(build_dir.clone()) {
        panic!("Failed to create build_directory: {:#?}", e);
    }

    build_dir
}

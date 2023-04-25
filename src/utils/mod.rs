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

pub fn invoke_linker(object_file: &str, output_file: &str, linker_is_cc: bool) {
    // For now we only support x86_64 elf emulations!
    // TODO: If linker is cc, then invoke ld on failure.
    // TODO: If linker is ld, then invoke cc on failure.
    let mut ld_cmd = process::Command::new("sh");
    ld_cmd.arg("-c");
    ld_cmd.arg(format!(
        "ld											\

			-m elf_x86_64								\
			/usr/lib/x86_64-linux-gnu/crti.o			\
			/usr/lib/x86_64-linux-gnu/crt1.o			\
			/usr/lib/gcc/x86_64-linux-gnu/*/crtbegin.o	\
			-l /usr/lib/gcc/x86_64-linux-gnu			\
			-l/usr/lib/x86_64-linux-gnu					\
			-l/usr/lib64								\
			-l/lib64									\
			-l/usr/lib/x86_64-linux-gnu					\
			-l/usr/lib									\
			-l/lib										\
			-dynamic-linker								\
			/lib64/ld-linux-x86-64.so.2					\
			--start-group								\
			-lc											\
			--end-group									\
			/usr/lib/gcc/x86_64-linux-gnu/11/crtends.o	\
			/usr/lib/x86_64-linux-gnu/crtn.o			\
			-o {} {}",
        output_file, object_file
    ));

    let mut cc_cmd = process::Command::new("cc");
    cc_cmd.arg("-o").arg(output_file).arg(object_file);

    if linker_is_cc {
        if let Err(e) = cc_cmd.spawn() {
            println!("Failed to invoke $CC: {:#?}", e);
            println!("Attempting to invoke system $LD");
            if let Err(e) = ld_cmd.spawn() {
                panic!("Failed to run linker: {:#?}", e);
            }
        }
        return;
    }

    if let Err(e) = ld_cmd.spawn() {
        println!("Failed to invoke $LD: {:#?}", e);
        println!("Attempting to invoke system $CC");
        if let Err(e) = cc_cmd.spawn() {
            panic!("Failed to run linker: {:#?}", e);
        }
    }
}

pub fn run_qbe_codegen(input_file_name: &str, target: QBE_TARGETS) -> String {
    let build_dir: String = format!("/tmp/rustcc-build-{}/", Uuid::new_v4());
    let build_dir_path = Path::new(build_dir.as_str());
    if !build_dir_path.exists() {
        if let Err(e) = fs::create_dir(build_dir.clone()) {
            panic!("Failed to create build directory: {:#?}", e);
        }
    }
    let file_name = Path::new(input_file_name).file_stem().unwrap().to_str().unwrap();
    let output_file_name = format!("/{}/{}.s", build_dir, file_name);

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
    output_file_name
}

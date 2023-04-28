use crate::qbe;

use std::{
    ffi::CString,
    fs::{self, File},
    io::Write,
    path::Path,
    process,
};
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
    let output_name_c = CString::new(output_file_name).unwrap();

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

pub fn invoke_linker(linker: &str, output_file: &str, lib_files: Vec<String>) {
    let mut command = process::Command::new(linker);

    command.arg("-o");
    command.arg(output_file);

    for lib_file in lib_files {
        command.arg(lib_file);
    }

    command.spawn().unwrap().wait().unwrap();
}

pub fn generate_libc_files(build_dir: String, use_musl: bool, object_name: String) -> Vec<String> {
    let mut lib_files: Vec<String> = vec![object_name.clone()];
    if use_musl {
        // This mess can be cleaned up with some comptime evaluation. Wish I was using zig rn..
        lib_files.append(&mut vec![
            format!("{}crt1.o", build_dir),
            format!("{}crtn.o", build_dir),
            format!("{}libcmusl.a", build_dir),
            format!("{}libcrypt.a", build_dir),
            format!("{}libdl.a", build_dir),
            format!("{}libm.a", build_dir),
            format!("{}libpthread.a", build_dir),
            format!("{}libresolv.a", build_dir),
            format!("{}librt.a", build_dir),
            format!("{}libutil.a", build_dir),
            format!("{}libxnet.a", build_dir),
        ]);

        File::create(lib_files[1].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/musl-libc/crt1.o"))
            .unwrap();
        File::create(lib_files[2].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/musl-libc/crtn.o"))
            .unwrap();
        File::create(lib_files[3].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/musl-libc/libcmusl.a"))
            .unwrap();
        File::create(lib_files[4].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/musl-libc/libcrypt.a"))
            .unwrap();
        File::create(lib_files[5].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/musl-libc/libdl.a"))
            .unwrap();
        File::create(lib_files[6].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/musl-libc/libm.a"))
            .unwrap();
        File::create(lib_files[7].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/musl-libc/libpthread.a"))
            .unwrap();
        File::create(lib_files[8].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/musl-libc/libresolv.a"))
            .unwrap();
        File::create(lib_files[9].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/musl-libc/librt.a"))
            .unwrap();
        File::create(lib_files[10].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/musl-libc/libutil.a"))
            .unwrap();
        File::create(lib_files[11].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/musl-libc/libxnet.a"))
            .unwrap();
    } else {
        lib_files.append(&mut vec![
            format!("{}libstart.a", build_dir),
            format!("{}libcguana.a", build_dir),
        ]);

        File::create(lib_files[1].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/ziglibc/libstart.a"))
            .unwrap();

        File::create(lib_files[2].clone())
            .unwrap()
            .write_all(include_bytes!("../../target/lib/ziglibc/libcguana.a"))
            .unwrap();
    }

    lib_files
}

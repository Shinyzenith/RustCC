extern crate bindgen;
extern crate cc;

use std::env;
use std::fs::{remove_file, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    let include_dir = "./deps/qbe";
    let qbe_backend_path = "./src/qbe/qbe.c";
    let qbe_backend_header_path = "./src/qbe/qbe.h";
    let qbe_backend_mod_path = "./src/qbe/mod.rs";

    println!("cargo:rerun-if-changed={}", qbe_backend_path);
    format!("cargo:rerun-if-changed={}", qbe_backend_header_path);

    bindgen_qbe(include_dir, qbe_backend_header_path, qbe_backend_mod_path);
    compile_c_files(include_dir);
}

fn bindgen_qbe(include_dir: &str, qbe_backend_header_path: &str, qbe_backend_mod_path: &str) {
    let bindings = bindgen::Builder::default()
        .header(qbe_backend_header_path)
        .clang_arg(format!("-I{}", include_dir))
        .generate()
        .unwrap();

    // Removing useless bindgen errors!
    // Wish there was an easier way to prepend to files through seeking!
    let qbe_backend_mod_file =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(qbe_backend_mod_path);

    _ = remove_file(qbe_backend_mod_file.clone());

    writeln!(OpenOptions::new().write(true).create(true).open(qbe_backend_mod_file.clone()).unwrap(),
    "#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code, improper_ctypes, clippy::upper_case_acronyms, clippy::useless_transmute)]").unwrap();

    let tmp = PathBuf::from(env::var("OUT_DIR").unwrap()).join("tmp");
    bindings.write_to_file(tmp.clone()).unwrap();

    io::copy(
        &mut OpenOptions::new().read(true).open(tmp).unwrap(),
        &mut OpenOptions::new().append(true).open(qbe_backend_mod_file).unwrap(),
    )
    .unwrap();
}

fn compile_c_files(include_dir: &str) {
    let c_files = vec![
        "./deps/qbe/util.c",
        "./deps/qbe/parse.c",
        "./deps/qbe/abi.c",
        "./deps/qbe/cfg.c",
        "./deps/qbe/mem.c",
        "./deps/qbe/ssa.c",
        "./deps/qbe/alias.c",
        "./deps/qbe/load.c",
        "./deps/qbe/copy.c",
        "./deps/qbe/fold.c",
        "./deps/qbe/simpl.c",
        "./deps/qbe/live.c",
        "./deps/qbe/spill.c",
        "./deps/qbe/rega.c",
        "./deps/qbe/emit.c",
        "./deps/qbe/amd64/targ.c",
        "./deps/qbe/amd64/sysv.c",
        "./deps/qbe/amd64/isel.c",
        "./deps/qbe/amd64/emit.c",
        "./deps/qbe/arm64/targ.c",
        "./deps/qbe/arm64/abi.c",
        "./deps/qbe/arm64/isel.c",
        "./deps/qbe/arm64/emit.c",
        "./deps/qbe/rv64/targ.c",
        "./deps/qbe/rv64/abi.c",
        "./deps/qbe/rv64/isel.c",
        "./deps/qbe/rv64/emit.c",
        "./src/qbe/qbe.c",
    ];

    let mut builder = cc::Build::new();
    for c_file in c_files {
        builder.file(c_file).include(include_dir);
    }
    builder.flag("-std=c99").flag("-Wall").flag("-Wextra").flag("-Wpedantic").compile("qbe");
}

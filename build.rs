extern crate bindgen;
extern crate cc;

use std::{
    env,
    fs::{self, remove_file, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process,
};

fn main() {
    let include_dir = "./deps/qbe";
    let qbe_backend_path = "./src/qbe/qbe.c";
    let qbe_backend_header_path = "./src/qbe/qbe.h";
    let qbe_backend_mod_path = "./src/qbe/mod.rs";

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = Path::new(&manifest_dir).join("target/lib");
    if !lib_dir.exists() {
        fs::create_dir(lib_dir).unwrap();
    }

    println!("cargo:rerun-if-changed={}", qbe_backend_path);
    format!("cargo:rerun-if-changed={}", qbe_backend_header_path);

    bindgen_qbe(manifest_dir.clone(), include_dir, qbe_backend_header_path, qbe_backend_mod_path);
    compile_c_files(include_dir);

    build_libcguana(manifest_dir.clone(), "0.11.0-dev.2157+f56f3c582");
    build_musl_libc(manifest_dir);
}

fn build_musl_libc(manifest_dir: String) {
    let musl_src_dir = format!("{}/deps/musl-libc", manifest_dir);

    let musl_lib_dir = Path::new(&manifest_dir).join("target/lib/musl-libc");
    if !musl_lib_dir.exists() {
        fs::create_dir(musl_lib_dir.clone()).unwrap();
    }

    process::Command::new("./configure")
        .arg("--prefix=build")
        .arg("--disable-shared")
        .current_dir(musl_src_dir.clone())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    process::Command::new("make")
        .arg("install-libs")
        .current_dir(musl_src_dir.clone())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let lib_files = vec![
        "libc.a",
        "crt1.o",
        "crtn.o",
        "libc.a",
        "libcrypt.a",
        "libdl.a",
        "libm.a",
        "libpthread.a",
        "libresolv.a",
        "librt.a",
        "libutil.a",
        "libxnet.a",
    ];

    for lib_file in lib_files {
        println!("{}", lib_file);
        fs::copy(
            format!("{}/build/lib/{}", musl_src_dir, lib_file),
            format!("{}/{}", musl_lib_dir.clone().to_str().unwrap(), lib_file),
        )
        .unwrap();
    }

    fs::rename(
        format!("{}/libc.a", musl_lib_dir.to_str().unwrap()),
        format!("{}/libcmusl.a", musl_lib_dir.to_str().unwrap()),
    )
    .unwrap();
}

fn build_libcguana(manifest_dir: String, zig_version: &str) {
    let zig_libc_dir = format!("{}/deps/ziglibc", manifest_dir);

    let zig_lib_dir = Path::new(&manifest_dir).join("target/lib/ziglibc");
    if !zig_lib_dir.exists() {
        fs::create_dir(zig_lib_dir).unwrap();
    }

    process::Command::new("curl")
        .arg("--output")
        .arg(format!("zig-linux-x86_64-{}.tar.xz", zig_version))
        .arg(format!("https://ziglang.org/builds/zig-linux-x86_64-{}.tar.xz", zig_version))
        .current_dir(zig_libc_dir.clone())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    process::Command::new("tar")
        .arg("xf")
        .arg(format!("zig-linux-x86_64-{}.tar.xz", zig_version))
        .current_dir(zig_libc_dir.clone())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    process::Command::new("sh")
        .arg("-c")
        .current_dir(zig_libc_dir.clone())
        .arg("./zig-linux-x86_64-0.11.0-dev.2157+f56f3c582/zig build")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    fs::copy(
        format!("{}/zig-out/lib/libcguana.a", zig_libc_dir),
        format!("{}/target/lib/ziglibc/libcguana.a", manifest_dir),
    )
    .unwrap();

    fs::copy(
        format!("{}/zig-out/lib/libstart.a", zig_libc_dir),
        format!("{}/target/lib/ziglibc/libstart.a", manifest_dir),
    )
    .unwrap();
}

fn bindgen_qbe(
    manifest_dir: String,
    include_dir: &str,
    qbe_backend_header_path: &str,
    qbe_backend_mod_path: &str,
) {
    let bindings = bindgen::Builder::default()
        .header(qbe_backend_header_path)
        .clang_arg(format!("-I{}", include_dir))
        .generate()
        .unwrap();

    // Removing useless bindgen errors!
    // Wish there was an easier way to prepend to files through seeking!
    let qbe_backend_mod_file = PathBuf::from(format!("{}/{}", manifest_dir, qbe_backend_mod_path));

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

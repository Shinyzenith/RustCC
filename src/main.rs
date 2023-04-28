mod qbe;
mod utils;

fn main() {
    let build_dir = utils::init_build_directory();

    let bin_executable = "a.out";
    let linker = "ld.lld";
    let use_musl: bool = false;

    let input_name = String::from("./output.ssa");
    let output_name = format!("{}{}", build_dir, input_name.replace(".ssa", ".s"));
    let object_name = format!("{}{}", build_dir, input_name.replace(".ssa", ".o"));

    let lib_files = utils::generate_libc_files(build_dir, use_musl, object_name.clone());

    utils::run_qbe_codegen(&input_name, &output_name, utils::QBE_TARGETS::AMD64_SYSV);
    println!("QBE x86_64 codegen complete!");

    utils::invoke_assembler(&output_name, &object_name);
    println!("Object file assembly completed!");

    utils::invoke_linker(linker, bin_executable, lib_files);
    println!("Linking complete!");
}

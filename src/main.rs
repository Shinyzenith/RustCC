mod qbe;
mod utils;

fn main() {
    let input_name = "./output.ssa";
    let object_name = input_name.replace(".ssa", ".o");

    // For now we only support x86_64
    let output_name = utils::run_qbe_codegen(input_name, utils::QBE_TARGETS::AMD64_SYSV);
    println!("QBE x86_64 codegen complete!");

    utils::invoke_assembler(&output_name, &object_name);
    println!("Object file assembly completed!");

    utils::invoke_linker(&object_name, "a.out", true);
    println!("Linking complete!");
}

mod qbe;
mod utils;
use std::{fs::File, io::Write};

fn main() {
    let build_dir = utils::init_build_directory();

    let input_name = String::from("./output.ssa");
    let output_name = format!("{}{}", build_dir, input_name.replace(".ssa", ".s"));
    let object_name = format!("{}{}", build_dir, input_name.replace(".ssa", ".o"));

    let lib_start = format!("{}libstart.a", build_dir);
    let lib_cguana = format!("{}libcguana.a", build_dir);

    File::create(lib_start.clone())
        .unwrap()
        .write_all(include_bytes!("../target/lib/libstart.a"))
        .unwrap();
    File::create(lib_cguana.clone())
        .unwrap()
        .write_all(include_bytes!("../target/lib/libcguana.a"))
        .unwrap();

    utils::run_qbe_codegen(&input_name, &output_name, utils::QBE_TARGETS::AMD64_SYSV);
    println!("QBE x86_64 codegen complete!");

    utils::invoke_assembler(&output_name, &object_name);
    println!("Object file assembly completed!");

    invoke_linker!("a.out", &object_name, lib_start, lib_cguana);
    println!("Linking complete!");
}

#[macro_export]
macro_rules! invoke_linker {
    ($output_file:expr, $( $object_file:expr ),* ) => {
		let mut command = std::process::Command::new("ld.lld");
		command.arg("-m");
		command.arg("elf_x86_64");
		command.arg("-o");
		command.arg($output_file);
		command.arg("--as-needed");
		$(
			command.arg($object_file);
		)*
		if let Err(e) = command.spawn() {
			panic!("Failed to invoke linker: {:#?}", e);
		}
	};
}

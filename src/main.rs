mod qbe;
use std::ffi::{c_char, CString};

fn main() {
    let input_path = CString::new("/tmp/output.ssa").unwrap();
    let output_path = CString::new("/tmp/out.s").unwrap();

    let read_mode = CString::new("r").unwrap();
    let write_mode = CString::new("w").unwrap();

    unsafe {
        let input_stream = qbe::fopen(input_path.as_ptr(), read_mode.as_ptr());
        let output_stream = qbe::fopen(output_path.as_ptr(), write_mode.as_ptr());

        qbe::codegen(
            input_stream,
            input_path.as_ptr() as *mut c_char,
            output_stream,
            qbe::T_amd64_sysv,
        );

        qbe::fclose(output_stream);
        qbe::fclose(input_stream);
    }

    println!("Codegen complete!");
}

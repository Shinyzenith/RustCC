mod qbe;
use std::ffi::{c_char, CString};

fn main() {
    let input_path = CString::new("/tmp/output.ssa").unwrap();
    let output_path = CString::new("/tmp/out.s").unwrap();

    unsafe {
        qbe::codegen(
            input_path.as_ptr() as *mut c_char,
            output_path.as_ptr() as *mut c_char,
            qbe::T_amd64_sysv,
        );
    }

    println!("Codegen complete!");
}

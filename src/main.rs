#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::env;
use std::ffi::CString;
use std::ptr::null;
use std::process::exit;
use std::os::raw::c_char;
use std::option::Option;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		eprintln!("Invalid number of arguments, call like this: {} yang_filename.", args[0]);
		exit(1);
	}

	let f = CString::new(args[1].clone()).unwrap();
	unsafe {
		let ctx = ly_ctx_new(null(), 0);
		let filename_c_ptr : *const c_char = f.as_ptr();
		let destructor: Option<unsafe extern "C" fn(*const lys_node, *mut std::ffi::c_void)> = None;
		lys_parse_path(ctx, filename_c_ptr, LYS_INFORMAT_LYS_IN_YANG);
		ly_ctx_clean(ctx, destructor);
		ly_ctx_destroy(ctx, destructor);
	}
}
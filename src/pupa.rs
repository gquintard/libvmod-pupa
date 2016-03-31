extern crate libc;
use libc::c_int;
use libc::c_uint;
use libc::c_char;
use libc::c_void;
use libc::c_double;

use std::io::prelude::*;
use std::ptr;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::boxed::Box;
use std::collections::HashMap;

#[repr(C)]
pub struct VrtCtx<'a> {
	magic: c_uint,
	method: c_uint,
	handling: *const c_uint,

	vsb: *const c_void,
	vsb_log: *const c_void,
	vcl: *const c_void,
	ws: &'a ws,

	req: *const c_void,
	http_req: *const c_void,
	http_req_top: *const c_void,
	http_resp: *const c_void,

	bo: *const c_void,
	http_bereq: *const c_void,
	http_beresp: *const c_void,

	now: c_double,

	specific: *const c_void
}

#[repr(C)]
struct ws {                                                                                                                                                                                                        
	magic: c_uint,
	id: [c_char; 4], //id[4], actually
	s: *const c_char,
	f: *mut c_char,
	r: *const c_char,
	e: *const c_char,
}

extern {
	fn WS_Reserve(ws: *const ws, bytes: c_uint) -> c_uint;
	fn WS_Release(ws: *const ws, bytes: c_uint);
}

#[repr(C)]
pub enum VclEvent {
	Load = 0,
	Warm,
	Use,
	Cold,
	Discard,
}

pub struct vmod_priv {                                                                                                                                                                                                 
	prv	: *const Mutex<HashMap<String, c_int>>,
	len	: c_int,
	free	: *const c_void
}

fn conv(input: *const c_char) -> String {
	unsafe { 
		CString::new(String::new() +
			     CStr::from_ptr(input).to_str().unwrap())
	}.unwrap()
	.into_string()
	.unwrap()
}

#[no_mangle]
pub extern fn init_function(_	: *const VrtCtx,
			    prv	: &mut vmod_priv,
			    ev	: VclEvent ) -> c_int{
	match ev {
		VclEvent::Warm => {
			let hash = Mutex::new(HashMap::new());
			prv.prv = Box::into_raw( Box::new(hash));
		},
		VclEvent::Cold => {
			unsafe { Box::from_raw(prv);}
		},
		_ => ()
	}
	0
}

#[no_mangle]
pub unsafe extern fn vmod_push(ctx	: &VrtCtx,
				prv	: &vmod_priv,
				input	: *const c_char) -> c_int {
	
	let mut hash = unsafe { &*prv.prv }
			.lock()
			.unwrap();
	
	let key = conv(input);

	let c = hash.entry(key).or_insert(0);
	*c += 1;
	*c
}

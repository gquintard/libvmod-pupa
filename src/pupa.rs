extern crate libc;
use libc::c_int;
use libc::c_char;
use libc::c_void;

use std::ffi::CStr;
use std::ffi::CString;
use std::sync::Mutex;
use std::boxed::Box;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

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

#[no_mangle]
pub extern fn init_function(_	: *const c_void,
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

fn conv(input: *const c_char) -> String {
	unsafe {
		CString::new(String::new() +
			     CStr::from_ptr(input).to_str().unwrap())
	}.unwrap()
	.into_string()
	.unwrap()
}

#[no_mangle]
pub unsafe extern fn vmod_push(_	: *const c_void,
				prv	: &vmod_priv,
				input	: *const c_char) -> c_int {
	
	let mut hash = (&*prv.prv).lock().unwrap();

	let key = conv(input);

	let c = hash.entry(key).or_insert(0);
	*c += 1;
	*c
}

#[no_mangle]
pub unsafe extern fn vmod_pull(_	: *const c_void,
				prv	: &vmod_priv,
				input	: *const c_char) -> c_int {

	let mut hash = (&*prv.prv).lock().unwrap();

	let key = conv(input);

	match hash.entry(key) {
		Occupied(mut entry) => {
			if *entry.get() == 0 { 0 }
			else {
				*entry.get_mut() -= 1;
				*entry.get()
			}
		},
		Vacant(_) => 0
	}
}

#[no_mangle]
pub unsafe extern fn vmod_peek(_	: *const c_void,
				prv	: &vmod_priv,
				input	: *const c_char) -> c_int {

	let mut hash = (&*prv.prv).lock().unwrap();

	let key = conv(input);

	match hash.entry(key) {
		Occupied(entry) => *entry.get(),
		Vacant(_) => 0
	}
}

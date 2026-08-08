use crate::graph::G;
use crate::ipc::Ipc;
use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn silt_add(v: f64) -> usize { G.write().add(v) }

#[no_mangle]
pub extern "C" fn silt_link(src: usize, dst: usize) { G.write().link(src, dst); }

#[no_mangle]
pub extern "C" fn silt_set(id: usize, v: f64) { G.write().set(id, v); }

#[no_mangle]
pub extern "C" fn silt_get(id: usize) -> f64 { G.read().get(id) }

#[no_mangle]
pub extern "C" fn silt_ipc_open(name: *const libc::c_char) -> *mut Ipc {
    let s = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy() };
    Box::into_raw(Box::new(Ipc::open(&s).unwrap()))
}

#[no_mangle]
pub extern "C" fn silt_ipc_write(ptr: *mut Ipc, k: usize, v: f64) { unsafe { (*ptr).write(k, v) } }

#[no_mangle]
pub extern "C" fn silt_ipc_read(ptr: *mut Ipc, k: usize) -> f64 { unsafe { (*ptr).read(k) } }
use crate::graph::G;
use crate::ipc::Ipc;

#[no_mangle]
pub extern "C" fn silt_add(v: f64) -> usize { G.write().add(v) }

#[no_mangle]
pub extern "C" fn silt_link(src: usize, dst: usize) { G.write().link(src, dst); }

#[no_mangle]
pub extern "C" fn silt_set(id: usize, v: f64) { G.write().set(id, v); }

#[no_mangle]
pub extern "C" fn silt_get(id: usize) -> f64 {
    G.write().get(id).unwrap_or(0.0)
}

#[no_mangle]
pub extern "C" fn silt_ipc_open(name: *const libc::c_char) -> *mut Ipc {
    let s = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy() };
    match Ipc::open(&s) {
        Some(ipc) => Box::into_raw(Box::new(ipc)),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn silt_ipc_write(ptr: *mut Ipc, k: usize, v: f64) {
    if !ptr.is_null() { unsafe { (*ptr).write(k, v) } }
}

#[no_mangle]
pub extern "C" fn silt_ipc_read(ptr: *mut Ipc, k: usize) -> f64 {
    if !ptr.is_null() { unsafe { (*ptr).read(k) } } else { 0.0 }
}
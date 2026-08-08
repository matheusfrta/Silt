use crate::primitives::{Signal, Effect};
use crate::ipc::IpcBridge;
use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn silt_sig_new_f64(v: f64) -> *mut Signal<f64> {
    Box::into_raw(Box::new(Signal::new(v)))
}

#[no_mangle]
pub extern "C" fn silt_sig_get_f64(ptr: *mut Signal<f64>) -> f64 {
    if ptr.is_null() { return 0.0; }
    unsafe { (*ptr).get() }
}

#[no_mangle]
pub extern "C" fn silt_sig_set_f64(ptr: *mut Signal<f64>, v: f64) {
    if !ptr.is_null() { unsafe { (*ptr).set(v) }; }
}

#[no_mangle]
pub extern "C" fn silt_ipc_open(name: *const libc::c_char) -> *mut IpcBridge {
    let s = unsafe { std::ffi::CStr::from_ptr(name).to_string_lossy() };
    match IpcBridge::new(&s) {
        Some(bridge) => Box::into_raw(Box::new(bridge)),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn silt_ipc_get(ptr: *mut IpcBridge, idx: usize) -> f64 {
    if ptr.is_null() { return 0.0; }
    unsafe { (*ptr).get(idx) }
}

#[no_mangle]
pub extern "C" fn silt_ipc_set(ptr: *mut IpcBridge, idx: usize, v: f64) {
    if !ptr.is_null() { unsafe { (*ptr).set(idx, v) }; }
}
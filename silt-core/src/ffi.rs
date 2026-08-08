use crate::primitives::Signal;

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
    if !ptr.is_null() {
        unsafe { (*ptr).set(v) };
    }
}

#[no_mangle]
pub extern "C" fn silt_sig_free_f64(ptr: *mut Signal<f64>) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr)) };
    }
}
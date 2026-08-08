import ctypes
import os

try:
    lib = ctypes.CDLL(os.path.abspath("libsilt.so"))
    lib.silt_sig_new_f64.restype = ctypes.c_void_p
    lib.silt_sig_get_f64.argtypes = [ctypes.c_void_p]
    lib.silt_sig_get_f64.restype = ctypes.c_double
    lib.silt_sig_set_f64.argtypes = [ctypes.c_void_p, ctypes.c_double]
except OSError:
    lib = None

class FFISignal:
    def __init__(self, val: float):
        if not lib:
            raise RuntimeError("core lib err")
        self.ptr = lib.silt_sig_new_f64(ctypes.c_double(val))
    
    def get(self) -> float:
        return lib.silt_sig_get_f64(self.ptr)
    
    def set(self, val: float):
        lib.silt_sig_set_f64(self.ptr, ctypes.c_double(val))
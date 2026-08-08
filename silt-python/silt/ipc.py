import ctypes
import os

try:
    lib = ctypes.CDLL(os.path.abspath("libsilt_core.so"))
    lib.silt_ipc_open.argtypes = [ctypes.c_char_p]
    lib.silt_ipc_open.restype = ctypes.c_void_p
    lib.silt_ipc_get.argtypes = [ctypes.c_void_p, ctypes.c_size_t]
    lib.silt_ipc_get.restype = ctypes.c_double
    lib.silt_ipc_set.argtypes = [ctypes.c_void_p, ctypes.c_size_t, ctypes.c_double]
except OSError:
    lib = None

class SharedState:
    def __init__(self, name: str):
        if not lib:
            raise RuntimeError("core lib err")
        self.ptr = lib.silt_ipc_open(name.encode('utf-8'))
        
    def get(self, idx: int) -> float:
        return lib.silt_ipc_get(self.ptr, idx)
        
    def set(self, idx: int, val: float):
        lib.silt_ipc_set(self.ptr, idx, ctypes.c_double(val))
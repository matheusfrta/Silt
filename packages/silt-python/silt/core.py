import ctypes
import os

try:
    lib = ctypes.CDLL(os.path.abspath("libsilt_core.so"))
    lib.silt_add.restype = ctypes.c_size_t
    lib.silt_get.restype = ctypes.c_double
    lib.silt_ipc_open.restype = ctypes.c_void_p
    lib.silt_ipc_read.restype = ctypes.c_double
except OSError:
    lib = None

class Graph:
    @staticmethod
    def add(v: float) -> int: return lib.silt_add(ctypes.c_double(v))
    @staticmethod
    def set(id: int, v: float): lib.silt_set(id, ctypes.c_double(v))
    @staticmethod
    def get(id: int) -> float: return lib.silt_get(id)

class IpcBridge:
    def __init__(self, name: str):
        self.ptr = lib.silt_ipc_open(name.encode())
    def write(self, k: int, v: float):
        lib.silt_ipc_write(self.ptr, k, ctypes.c_double(v))
    def read(self, k: int) -> float:
        return lib.silt_ipc_read(self.ptr, k)
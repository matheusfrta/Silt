use libc;
use std::ffi::CString;
use std::ptr;

#[repr(C)]
pub struct ShmSeg {
    pub magic: u32,
    pub locks: [u8; 4096],
    pub ver: [u64; 4096],
    pub vals: [f64; 4096],
}

pub struct Ipc {
    pub ptr: *mut ShmSeg,
    fd: libc::c_int,
}

impl Ipc {
    pub fn open(name: &str) -> Option<Self> {
        let cn = CString::new(name).ok()?;
        let sz = std::mem::size_of::<ShmSeg>();
        unsafe {
            let fd = libc::shm_open(cn.as_ptr(), libc::O_CREAT | libc::O_RDWR, 0o666);
            if fd < 0 { return None; }
            libc::ftruncate(fd, sz as libc::off_t);
            let ptr = libc::mmap(ptr::null_mut(), sz, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0);
            if ptr == libc::MAP_FAILED { return None; }
            
            let seg = &mut *(ptr as *mut ShmSeg);
            if seg.magic != 0x5117 {
                std::ptr::write_bytes(ptr, 0, sz);
                seg.magic = 0x5117;
            }
            Some(Self { ptr: ptr as *mut ShmSeg, fd })
        }
    }

    pub fn write(&self, k: usize, v: f64) {
        if k < 4096 { unsafe { (*self.ptr).vals[k] = v; (*self.ptr).ver[k] += 1; } }
    }
    
    pub fn read(&self, k: usize) -> f64 {
        if k < 4096 { unsafe { (*self.ptr).vals[k] } } else { 0.0 }
    }
}
use libc;
use std::ffi::CString;
use std::ptr;

// fast shared memory synchronization between node, py, rust
#[repr(C)]
pub struct MmapSegment {
    pub magic: u32,
    pub locks: [u8; 1024],
    pub versions: [u64; 1024],
    pub values: [f64; 1024],
}

pub struct IpcBridge {
    pub ptr: *mut MmapSegment,
    pub size: usize,
    fd: libc::c_int,
}

impl IpcBridge {
    pub fn new(name: &str) -> Option<Self> {
        let cname = CString::new(name).ok()?;
        let size = std::mem::size_of::<MmapSegment>();
        unsafe {
            let fd = libc::shm_open(cname.as_ptr(), libc::O_CREAT | libc::O_RDWR, 0o666);
            if fd < 0 { return None; }
            if libc::ftruncate(fd, size as libc::off_t) < 0 { return None; }
            let ptr = libc::mmap(
                ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                0,
            );
            if ptr == libc::MAP_FAILED { return None; }
            
            let seg = &mut *(ptr as *mut MmapSegment);
            if seg.magic != 0x5117 {
                std::ptr::write_bytes(ptr, 0, size);
                seg.magic = 0x5117;
            }
            Some(Self { ptr: ptr as *mut MmapSegment, size, fd })
        }
    }

    pub fn get(&self, idx: usize) -> f64 {
        if idx >= 1024 { return 0.0; }
        unsafe { (*self.ptr).values[idx] }
    }

    pub fn set(&self, idx: usize, v: f64) {
        if idx >= 1024 { return; }
        unsafe { 
            (*self.ptr).values[idx] = v;
            (*self.ptr).versions[idx] += 1;
        }
    }
}
#[desc = "Rust mmap wrapper"]
#[license = "MIT"]

use std::libc;
use std::os;
use std::fmt;
use std::vec::raw;

struct MappedRegion {
    addr : *libc::c_void,
    len : libc::size_t
}

impl fmt::Default for MappedRegion {
    fn fmt(value: &MappedRegion, f: &mut std::fmt::Formatter) {
        write!(f.buf, "MappedRegion\\{{}, {}\\}", value.addr, value.len);
    }
}

impl Drop for MappedRegion {
    #[fixed_stack_segment]
    fn drop(&mut self) {
        debug!(format!("munmapping {}", self.addr));
        unsafe {
            if libc::munmap(self.addr, self.len) < 0 {
                fail!(format!("munmap({}, {}): {}", self.addr, self.len, os::last_os_error()));
            } else {
                self.addr = 0 as *libc::c_void;
                self.len = 0;
            }
        }
    }
}

#[fixed_stack_segment]
fn mmap(size: libc::size_t) -> Result<MappedRegion, ~str> {
    unsafe {
        let ptr = libc::mmap(0 as *libc::c_void, size, libc::PROT_READ, libc::MAP_ANON | libc::MAP_PRIVATE, 0, 0) as *libc::c_void;
        return match ptr {
            x if x == libc::MAP_FAILED => { Err(os::last_os_error()) }
            _ => { Ok(MappedRegion{addr : ptr, len : size}) }
        }
    }
}

pub fn mmap_as_slice<T, U>(size: libc::size_t, f: &fn(v: &[T]) -> U) -> U {
    match mmap(size) {
        Ok(r) => { unsafe { raw::buf_as_slice(r.addr as *T, r.len as uint, f) } }
        Err(s) => { fail!(s) }
    }
}

#[test]
fn test_all() {
    match mmap(4096) {
        Ok(r) => { println!("ok: {}", r); }
        Err(s) => { println!("err: {}", s); }
    }

    do mmap_as_slice(4096) |v : &[u8]| {
        println!("v[0] = {}", v[4095]);
    }
    println("exiting main");
}

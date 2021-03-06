#[desc = "Rust mmap wrapper"]
#[license = "MIT"]

extern crate libc;
use std::os;
use std::fmt;
use std::vec::raw;

struct MappedRegion {
    addr : *libc::c_void,
    len : libc::size_t,
    prot: libc::c_int,
}

impl MappedRegion {
    fn protect(&mut self, prot: uint) -> Option<MappedRegion> {
        if libc::mprotect(self.addr, self.len, prot) < 0 {
            None
        } else {
            self.prot = prot;
            Some(self)
        }
    }
}

impl fmt::Show for MappedRegion {
    fn show(value: &MappedRegion, f: &mut std::fmt::Formatter) {
        write!(f.buf, "MappedRegion\\{{}, {}\\}", value.addr, value.len);
    }
}

impl Drop for MappedRegion {
    #[fixed_stack_segment]
    fn drop(&mut self) {
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
        return if ptr == libc::MAP_FAILED {
            Err(os::last_os_error())
        } else {
            Ok(MappedRegion{addr : ptr, len : size})
        }
    }
}

pub fn mmap_as_slice<T, U>(size: libc::size_t, f: &fn(v: &[T]) -> U) -> U {
    match mmap(size) {
        Ok(r) => { unsafe { raw::from_buf(r.addr as *T, r.len as uint, f) } }
        Err(s) => { fail!(s) }
    }
}

#[test]
fn test_all() {
    match mmap(4096) {
        Ok(r) => {
            println!("ok: {}", r);
            match r.protect(libc::PROT_READ | libc::PROT_EXEC) {
                Some(r) => { println!("Protected!"); }
                None => { println!("Mapping failed!"); }
            }
        }
        Err(s) => { println!("err: {}", s); }
    }

     mmap_as_slice(4096, |v : &[u8]| {
        println!("v[0] = {}", v[0]);
    });

    println("exiting");
}

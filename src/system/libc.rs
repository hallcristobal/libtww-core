use Addr;
use addrs::system::{FREE, MEMALIGN};
use core::fmt;
use core::mem::transmute;
use core::ptr::null_mut;

extern "C" {
    #[link_name = "cMl::memalignB(i32, u32)"]
    fn game_memalign(align: isize, size: usize) -> *mut u8;
    #[link_name = "cMl::free(void*)"]
    fn game_free(ptr: *mut u8);
    #[link_name = "strlen"]
    fn game_strlen(string: *const u8) -> usize;
}

#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    let memalign =
        unsafe { transmute::<Addr, extern "C" fn(size_t, size_t) -> *mut c_void>(MEMALIGN) };
    memalign(0xFFFFFFFC, size)
}

#[no_mangle]
pub extern "C" fn posix_memalign(
    memptr: *mut *mut c_void,
    alignment: size_t,
    size: size_t,
) -> c_int {
    let memalign =
        unsafe { transmute::<Addr, extern "C" fn(size_t, size_t) -> *mut c_void>(MEMALIGN) };
    unsafe {
        *memptr = memalign(alignment, size);
    }
    0
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    let free = unsafe { transmute::<Addr, extern "C" fn(*mut c_void)>(FREE) };
    free(ptr);
}

#[no_mangle]
pub extern "C" fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    let new_data = malloc(size);

    if ptr != null_mut() {
        let mut dst = new_data as *mut u8;
        let mut src = ptr as *mut u8;

        for _ in 0..size {
            unsafe {
                *dst = *src;
                dst = dst.offset(1);
                src = src.offset(1);
            }
        }

        free(ptr);
    }

    new_data
}

#[no_mangle]
pub extern "C" fn posix_memalign(
    memptr: *mut *mut u8,
    alignment: usize,
    size: usize,
) -> i32 {
    unsafe {
        *memptr = game_memalign(alignment as isize, size);
    }
    0
}


#[no_mangle]
pub extern "C" fn write(_file: i32, _buffer: *const u8, _count: usize) -> i32 {
    unimplemented!()
}

pub fn strlen(string: *const u8) -> usize {
    unsafe { game_strlen(string) }
}

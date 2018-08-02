#![allow(private_no_mangle_fns)]

#[lang = "oom"]
#[no_mangle]
pub fn rust_oom() -> ! {
    loop {}
}

use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
pub fn my_panic(_: &PanicInfo) -> ! {
    loop {}
}

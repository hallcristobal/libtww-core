#![allow(private_no_mangle_fns)]
#[cfg(feature = "alloc")]
use core_alloc::alloc::Layout;
#[cfg(feature = "alloc")]
#[lang = "oom"]
fn rust_oom(_: Layout) -> ! {
    loop {}
}

use core::panic::PanicInfo;
#[panic_implementation]
#[no_mangle]
pub fn my_panic(_: &PanicInfo) -> ! {
    loop {}
}

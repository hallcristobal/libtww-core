#![allow(private_no_mangle_fns)]

use gcn;

#[cfg(feature = "alloc")]
use arrayvec::ArrayString;
#[cfg(feature = "alloc")]
use gcn::os::report;
#[cfg(feature = "alloc")]
use core::fmt::Write;

#[cfg(feature = "alloc")]
use core_alloc::alloc::Layout;
#[cfg(feature = "alloc")]
#[lang = "oom"]
fn rust_oom(layout: Layout) -> ! {
    unsafe {
        let mut message = ArrayString::<[u8; 1024]>::new();
        write!(message, "{:?}\0", layout).ok();
        report(message.as_ptr());
        loop {}
    }
}

use core::panic::PanicInfo;
#[panic_implementation]
#[no_mangle]
pub fn my_panic(info: &PanicInfo) -> ! {
    gcn::report_panic(info);
    loop {}
}

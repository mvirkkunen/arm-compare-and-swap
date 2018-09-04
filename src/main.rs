#![no_std]
#![no_main]
#![feature(used, start, panic_handler)]

// Observations:
//
// - changing inline(always) to inline(never) makes it work (it tends on to
//   inline without the attribute and break as well)
// - removing the "value" field makes it work
//
// Build with nightly in release mode:
//
// cargo +nightly build --release

use core::sync::atomic::{AtomicUsize, Ordering};

#[no_mangle]
pub extern "C" fn _start() {
    let foo = Foo::new();

    foo.bar();

    loop { }
}

struct Foo {
    value: u32,
    atomic: AtomicUsize,
}

impl Foo {
    pub fn new() -> Foo {
        Foo {
            value: 0,
            atomic: AtomicUsize::new(0),
        }
    }

    #[inline(always)]
    pub fn bar(&self) {
        if self.atomic.compare_and_swap(
            0,
            1,
            Ordering::SeqCst) != 0
        {
            panic!("cell not mutably borrowable");
        }
    }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! { loop { } }

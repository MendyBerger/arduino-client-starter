#![no_std]
// Replacing the allocator and using the `alloc` crate are still unstable.
#![feature(alloc_error_handler)]

extern crate alloc;

mod arduino;
mod app;

use app::App;
use static_alloc::Bump;

#[no_mangle]
pub extern fn _start() {
    let mut app = App::new();
    loop {
        app.run();
    }
}

#[global_allocator]
static A: Bump<[u8; 1 << 16]> = Bump::uninit();

#[panic_handler]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    app::println("full crash");
    core::arch::wasm32::unreachable();
}

#[alloc_error_handler]
pub fn oom(_: ::core::alloc::Layout) -> ! {
    app::println("alloc crash");
    core::arch::wasm32::unreachable();
}

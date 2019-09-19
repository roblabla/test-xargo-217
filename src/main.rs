#![feature(lang_items, alloc_error_handler, link_args)]
#![no_std]
#![link_args="-nostdlib"]
extern crate alloc;

fn main() {
    spin::Mutex::new(12);
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

#[panic_handler] #[no_mangle]
pub extern fn panic_fmt(p: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

struct Allocator;

unsafe impl alloc::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: alloc::alloc::Layout) -> *mut u8 {
        loop {}
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::alloc::Layout) {
        loop {}
    }
}

#[alloc_error_handler]
fn rust_oom(_: alloc::alloc::Layout) -> ! {
    loop {}
}

#[lang = "start"]
fn start(rmain: fn(), _argc: isize, _argv: *const *const u8) -> isize {
    rmain();
    0
}

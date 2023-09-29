#![no_std]
#![no_main]

mod lang_items;
mod sbi;
use crate::sbi::shutdown;

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    shutdown();
}
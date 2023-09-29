#![no_std]
#![no_main]

mod lang_items;
mod sbi;
use crate::sbi::shutdown;

#[no_mangle]
extern "C" fn _start() {
    shutdown();
}

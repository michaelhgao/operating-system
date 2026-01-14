#![no_std]
#![no_main]

use core::panic::PanicInfo;
use uefi::prelude::*;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> Status {
    loop {}
}

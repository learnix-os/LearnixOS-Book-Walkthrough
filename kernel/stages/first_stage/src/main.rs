#![no_std]
#![no_main]
mod disk;

use common::constants::addresses::DISK_NUMBER_OFFSET;
use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};
use disk::DiskAddressPacket;
global_asm!(include_str!("../asm/boot.s"));

#[unsafe(no_mangle)]
fn first_stage() {
    // Read the disk number the os was booted from
    let disk_number = unsafe { core::ptr::read(DISK_NUMBER_OFFSET as *const u8) };

    let dap = DiskAddressPacket::new(128, 0, 0x7e0, 1);
    dap.load(disk_number);

    let msg = b"Hello, World!";
    for &ch in msg {
        unsafe {
            asm!(
                "mov ah, 0x0E",
                "mov al, {0}",
                "int 0x10",
                in(reg_byte) ch,
                out("ax") _,
            );
        }
    }
    loop {}
}

#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

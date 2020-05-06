#![feature(abi_efiapi)]
#![no_std]
#![no_main]
use uefi::prelude::*;
use uefi::data_types::CStr16;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {
    let message = "Hello, Rust+UEFI!\n";

    // FIXME: BAD CODE
    let bytes = message.as_bytes();
    let mut ucs = [0u16; 32];
    for i in 0..bytes.len() {
        ucs[i] = bytes[i] as u16
    }
    let str = unsafe { CStr16::from_u16_with_nul_unchecked(&ucs) };

    st.stdout().reset(false).unwrap_success();
    st.stdout().output_string(str).unwrap_success();
    loop {}
    // Status::SUCCESS
}

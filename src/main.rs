#![feature(abi_efiapi)]
#![no_std]
#![no_main]
use uefi::prelude::*;
use core::panic::PanicInfo;
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {

    st.stdout().reset(false).unwrap_success();

    writeln!(st.stdout(), "Hello, world!").unwrap();

    loop {}
    // Status::SUCCESS
}

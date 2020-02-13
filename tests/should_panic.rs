#![no_std]
#![no_main]

use core::panic::PanicInfo;

use burrit_os::{exit_qemu, QemuExitCode, serial_print, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    serial_print!("should fail... ");
    assert_eq!(0, 1);
}

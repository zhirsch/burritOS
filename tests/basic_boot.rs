#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(burrit_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use burrit_os::{hlt_loop, println, tests};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    test_main();
    hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    burrit_os::test_panic_handler(info);
}

tests! {
    test_println {
        println!("test_println output");
    }
}

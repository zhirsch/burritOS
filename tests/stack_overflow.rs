#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use burrit_os::{exit_qemu, serial_print, serial_println, QemuExitCode};
use lazy_static::lazy_static;

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(burrit_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    serial_print!("stack overflow... ");

    burrit_os::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    burrit_os::test_panic_handler(info);
}

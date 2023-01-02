#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use x86_64::instructions::port::Port;

use core::panic::PanicInfo;
pub mod vga_buffer;
pub mod serial;

pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}



pub trait Testable {
    fn run(&self) -> ();
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

impl<T> Testable for T 
where T:Fn() 
{
    fn run(&self) -> () {
        serial_println!("{}...\t", core::any::type_name::<T>()); // core::any::type_name will print the name of Fn()
        self();
        serial_println!("[ok]");
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> !{
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}


#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> !{
    test_main();
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    test_panic_handler(info)
}

#[test_case]
fn should_success() {
    assert_eq!(1, 1);
}

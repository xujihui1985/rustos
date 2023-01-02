#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use rustos::println;

#[no_mangle]
pub extern "C" fn _start() -> !{
    println!("hello world!");

    rustos::init();

    use x86_64::registers::control::Cr3;

    let (l4_pt, _) = Cr3::read();
    println!("l4 page table {:?}", l4_pt.start_address());

//    x86_64::instructions::interrupts::int3();
     // trigger page fault by set a virtual memory
    //let ptr = 0x205078 as *mut u32;

    //unsafe {let x = *ptr;}

    //println!("read worked");

    //unsafe { *ptr = 42; }

    //unsafe {
        //*(0xdeadbeee as *mut u64) = 42
    //}
    //
    //fn stack_overflow() {
        //stack_overflow();
    //}

//    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("it success");

    rustos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    rustos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    rustos::test_panic_handler(info)
}



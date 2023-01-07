#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use rustos::{println, memory::{self, BootInfoFrameAllocator}, allocator};
use bootloader::{BootInfo, entry_point};
use x86_64::{
    structures::paging::{Translate,Page},
    VirtAddr,
};
extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("hello world!");

    rustos::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed");

    let x = Box::new(31);
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());


    //let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    //for (i, entry) in l4_table.iter().enumerate() {
        //if !entry.is_unused() {
            //println!("L4 entry {}: {:?}", i, entry);

            //let phys = entry.frame().unwrap().start_address();
            //let virt = phys.as_u64() + boot_info.physical_memory_offset;
            //let ptr = VirtAddr::new(virt).as_mut_ptr();
            //let l3_table: &PageTable = unsafe { &*ptr };

            //for (i, entry) in l3_table.iter().enumerate() {

                //if !entry.is_unused() {
                    //println!(">> L3 entry {}: {:?}", i, entry);
                //}
            //}
        //}
    //}

    //let mut frame_allocator = memory::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    let addresses = [
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for &addr in addresses.iter() {
       let virt = VirtAddr::new(addr) ;
       let phys = mapper.translate_addr(virt);
       println!("{:?} -> {:?}", virt, phys);
    }

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


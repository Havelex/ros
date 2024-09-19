#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use ros::{print, println, sleeping::sleep, vga_buffer::WRITER};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use ros::allocator;
    use ros::memory;
    use ros::memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;

    ros::init();

    println!("Initializing memory:");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    print!("    Initializing mapper...");
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    println!("[OK]");
    print!("    Initializing frame allocator...");
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    println!("[OK]");
    println!("Memory: [OK]");

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    //end mem init

    print!("\nInit complete");
    for _ in 0..3 {
        sleep(500);
        print!(".");
    }
    sleep(1000);
    {
        WRITER.lock().clear();
    }
    // end init

    // test main
    #[cfg(test)]
    test_main();

    ros::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ros::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}

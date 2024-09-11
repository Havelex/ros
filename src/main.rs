#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use ros::println;
use ros::task::{simple_executor::SimpleExecutor, Task};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use ros::allocator;
    use ros::memory;
    use ros::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");
    ros::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    // end init

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();

    // test main
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    ros::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
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

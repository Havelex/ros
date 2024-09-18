#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ros::{println, timer::pit::set_pit_frequency};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("HEEEEEEEEEEEEEEEEEEELLLLLLLLLLLLLLLOOOOOOOOOOOOO");
    set_pit_frequency(500.0);
    loop {}
}

#[test_case]
fn check_divisor() {
    use core::arch::asm;
    use ros::println;
    let mut value: u16;

    unsafe {
        asm!(
            "in ax, dx",
            in("dx") 0x40,
            out("ax") value,
            options(nomem, nostack, preserves_flags)
        )
    }

    println!("VALUE {}", value);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}

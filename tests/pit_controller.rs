#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{arch::asm, panic::PanicInfo};

use bootloader::{entry_point, BootInfo};
use ros::{serial_println, timer::pit::set_pit_frequency};

entry_point!(main);

fn main(_boot_info: &'static BootInfo) -> ! {
    let divisor = read_pit_divisor();
    serial_println!();
    serial_println!("Divisor: {}", divisor);
    set_pit_frequency(100000);

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}

#[test_case]
fn check_channel_0() {
    let divisor = read_pit_divisor();
    serial_println!();
    serial_println!("Divisor: {}", divisor);
}

fn read_pit_divisor() -> u16 {
    // Send command to latch the counter value for Channel 0
    outb(0x43, 0x00);

    let low: u8 = inb(0x40);

    // Read the high byte of the divisor
    let high: u8 = inb(0x40);

    // Combine high and low bytes
    (high as u16) << 8 | (low as u16)
}

// Function to write a byte to an I/O port using inline assembly
fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value
        );
    }
}

// Function to read a byte from an I/O port using inline assembly
fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value
        );
    }
    value
}

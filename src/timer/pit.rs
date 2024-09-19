use core::arch::asm;

use crate::{println, utils::asm::outb};

const COMMAND_PORT: u16 = 0x43;
const CHANNEL_0_PORT: u16 = 0x40;
const DEFAULT_FREQUENCY: u32 = 1_193_181;

pub static mut PIT_FREQUENCY: u32 = DEFAULT_FREQUENCY;

pub fn set_pit_frequency(freq: u32) {
    assert_ne!(freq, 0);

    let divisor = DEFAULT_FREQUENCY / freq;
    assert!(divisor > 1 && divisor < 65536);

    let divisor = DEFAULT_FREQUENCY / freq;

    println!("\nFrequency: {}", freq);
    println!("Divisor: {}", divisor);

    let low_byte = (divisor & 0xFF) as u8;
    let high_byte = ((divisor >> 8) & 0xFF) as u8;

    unsafe {
        PIT_FREQUENCY = freq;

        outb(COMMAND_PORT, 0x36);
        outb(CHANNEL_0_PORT, low_byte);
        outb(CHANNEL_0_PORT, high_byte);

        // asm!(
        //     "out dx, al",
        //     in("dx") COMMAND_PORT,
        //     in("al") 0x36u8
        // );
        //
        // asm!(
        //     "out dx, al",
        //     in ("dx") CHANNEL_0_PORT,
        //     in("al") low_byte
        // );
        //
        // asm!(
        //     "out dx, al",
        //     in("dx") CHANNEL_0_PORT,
        //     in("al") high_byte
        // );
    }
}

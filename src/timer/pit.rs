use core::arch::asm;

use crate::interrupts::PICS;

const COMMAND_PORT: u16 = 0x43;
const CHANNEL_0_PORT: u16 = 0x40;
const DEFAULT_FREQUENCY: f64 = 1_193_181.66;

pub static mut PIT_FREQUENCY: f64 = DEFAULT_FREQUENCY;

pub fn set_pit_frequency(hz: f64) {
    let divisor = (DEFAULT_FREQUENCY / hz) as u16;

    unsafe {
        PIT_FREQUENCY = hz;

        asm!(
            "out dx, al",
            in("al") 0x36u8,
            in("dx") COMMAND_PORT
        );

        asm!(
            "out dx, al",
            in("al") (divisor & 0xFF) as u8,
            in ("dx") CHANNEL_0_PORT
        );

        asm!(
            "out dx, al",
            in("al") ((divisor & 0xFF00) >> 8) as u8,
            in("dx") CHANNEL_0_PORT
        );
    }
}

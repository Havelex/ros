use core::sync::atomic::Ordering;

use crate::{interrupts::TICKS, timer::pit::PIT_FREQUENCY};

pub fn sleep(ms: u32) {
    let ticks_per_ms = unsafe { PIT_FREQUENCY } / 1_000;
    let target_ticks = ms * ticks_per_ms;
    let start_ticks = unsafe { TICKS.load(Ordering::Relaxed) };

    while unsafe { TICKS.load(Ordering::Relaxed) } < start_ticks + target_ticks as usize {}
}

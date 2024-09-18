use crate::{interrupts::COUNTER, timer::pit::PIT_FREQUENCY};

pub fn sleep(ms: usize) {
    unsafe {
        let start = COUNTER;
        while COUNTER - start < ms * (PIT_FREQUENCY / 1000.0) as usize {}
    }
}

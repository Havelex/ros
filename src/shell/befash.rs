use alloc::string::{String, ToString};

use crate::print;

pub async fn spawn() {
    let mut directory = String::from("~");
    cmd_line(&mut directory);
}

fn cmd_line(directory: &mut String) {
    print!("{} -> ", directory);
}

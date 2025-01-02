#![allow(unused_macros, unused_macro_rules, unused)]

use std::io::Write;

// GENERAL
pub const ESC: &str = "\x1B";
pub const CSI: &str = "[";



// STYLE
#[macro_export]
macro_rules! set_style {
    ($args:expr) => {
        { format!("{ESC}{CSI}{ar}m", ar=$args) }
    };
}

pub const RESET_STYLE: &str = "\x1B[0m";

pub fn reset_style() {
    print!("{RESET_STYLE}");
    std::io::stdout().flush().unwrap();
}




// ERASE
pub const ERASE_SCREEN: &str = "\x1B[2J";

pub fn erase_screen() {
    print!("{ERASE_SCREEN}");
    std::io::stdout().flush().unwrap();
}




// CURSOR MOVEMENT
pub const RESET_CURSOR: &str = "\x1B[H";

#[macro_export]
macro_rules! move_cursor_up {
    ($lines:expr) => {
        { format!("{ESC}{CSI}{li}A", li=$lines) }
    };
}

#[macro_export]
macro_rules! move_cursor_down {
    ($lines:expr) => {
        { format!("{ESC}{CSI}{li}B", li=$lines) }
    };
}

#[macro_export]
macro_rules! move_cursor_right {
    ($columns:expr) => {
        { format!("{ESC}{CSI}{co}C", co=$columns) }
    };
}

#[macro_export]
macro_rules! move_cursor_left {
    ($columns:expr) => {
        { format!("{ESC}{CSI}{co}D", co=$columns) }
    };
}
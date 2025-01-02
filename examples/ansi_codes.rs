use std::{io::Write, thread::sleep, time::Duration};

use rust_tuil::*;
use ansi::*;



fn main() {
    let mut e = 0;

    print!("{E}{R}", E=ERASE_SCREEN, R=RESET_CURSOR);   
    std::io::stdout().flush().unwrap();

    loop {
        sleep(Duration::from_secs(1));
        e += 1;
        
        print!("{c}{e}", c=move_cursor_right!(3));
        print!("{c}", c=move_cursor_down!(3));
        print!("{c}{e}", c=move_cursor_left!(1));
        std::io::stdout().flush().unwrap();
    }
}
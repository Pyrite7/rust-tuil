


pub const ESC: &str = "\x1B";
pub const CSI: &str = "[";


macro_rules! set_style {
    ($args:expr) => {
        { format!("{ESC}{CSI}{args}m", args=$args) }
    };
}

pub const RESET_STYLE: &str = "\x1B[0m";





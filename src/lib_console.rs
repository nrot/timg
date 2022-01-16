#[cfg(unix)]
#[path="unix/console.rs"]
mod console;

#[cfg(windows)]
#[path="win/console.rs"]
mod console;

pub use console::get_terminal_size;


#[derive(Debug, Clone)]
pub struct Size {
    pub rows: u16,
    pub cols: u16
}

impl Default for Size{
    fn default() -> Self {
        Size { rows: 0, cols: 0 }
    }
}

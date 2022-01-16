extern crate libc;

use libc::ioctl;

use self::super::Size;

use self::libc::{STDOUT_FILENO, TIOCGWINSZ, c_ushort};

#[repr(C)]
#[derive(Debug)]
struct UnixSize{
    rows: c_ushort,
    cols: c_ushort,
    x: c_ushort,
    y: c_ushort
}

pub fn get_terminal_size() -> Result<Size, isize> {
    let mut us = UnixSize{
        rows: 0, cols: 0, x: 0, y:0
    };
    let r = unsafe{
        ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut us)
    };
    if r == 0{
        Ok(Size{
            rows: us.rows,
            cols: us.cols
        })
    } else {
        Err(r.try_into().unwrap())
    }
}
extern crate winapi;

use self::super::Size;
use std::ptr;
use std::mem::MaybeUninit;

use winapi::um::wincon::GetConsoleScreenBufferInfo;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winnt::HANDLE;

pub fn get_terminal_size() -> Result<Size, isize> {
    let handle: HANDLE = unsafe {
        winapi::um::fileapi::CreateFileA(
            b"CONOUT$\0".as_ptr() as *const i8,
            winapi::um::winnt::GENERIC_READ | winapi::um::winnt::GENERIC_WRITE,
            winapi::um::winnt::FILE_SHARE_WRITE,
            ptr::null_mut(),
            winapi::um::fileapi::OPEN_EXISTING,
            0,
            ptr::null_mut(),
        )
    };

    if handle == INVALID_HANDLE_VALUE{
        return Err(handle as isize);
    };
    unsafe{
        let mut info = MaybeUninit::uninit().assume_init();
        if GetConsoleScreenBufferInfo(handle, &mut info)==0{
            Err(-1)
        } else {
            Ok(Size{
                rows: (info.srWindow.Bottom - info.srWindow.Top + 1) as u16,
                cols: (info.srWindow.Right - info.srWindow.Left + 1) as u16
            })
        }
    }
}

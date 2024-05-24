use crate::error::{
        Error,
        Result,
};
use libc::{
        c_ushort,
        ioctl,
        TIOCGWINSZ,
};
use std::{
        io,
        os::fd::AsRawFd,
};

#[repr(C)]
struct Size
{
        ws_row: c_ushort,
        ws_col: c_ushort,
        ws_xpixel: c_ushort,
        ws_ypixel: c_ushort,
}

/// Gets terminal size.
///
/// # Usage
///
/// ```no_run
/// use ruterm::size;
///
/// let (width, height) = size().unwrap();
/// ```
///
/// # Note
///
/// Contains unsafe bindings!
pub fn size() -> Result<(u16, u16)>
{
        let size = Size {
                ws_row: 0,
                ws_col: 0,
                ws_xpixel: 0,
                ws_ypixel: 0,
        };
        match unsafe { ioctl(io::stdout().as_raw_fd(), TIOCGWINSZ, &size) } {
                0 => Ok((size.ws_col, size.ws_row)),
                _ => Err(Error("failed to get terminal size")),
        }
}

use crate::error::{
        Error,
        Result,
};
use std::io::{
        self,
        Read,
        Write,
};

/// Writes `content` to `to`.
///
/// # Usage
///
/// ```no_run
/// use std::io;
/// use terminal::io::write_to;
///
/// let mut to = io::stdout();
/// write_to(&mut to, b"Hello").unwrap();
/// ```
pub fn write_to(to: &mut dyn Write, content: &[u8]) -> Result<usize>
{
        to.write(content).map_err(|_| Error("failed to write"))
}

/// Writes `content` to stdout.
///
/// # Usage
///
/// ```no_run
/// use terminal::io::write;
///
/// write(b"Hello").unwrap();
/// ```
pub fn write(content: &[u8]) -> Result<usize>
{
        write_to(&mut io::stdout(), content)
}

/// Reads byte from `from`.
///
/// # Usage
///
/// ```no_run
/// use std::io;
/// use terminal::io::read_from;
///
/// let mut from = io::stdin();
/// if let Some(byte) = read_from(&mut from) {
///         println!("{byte}");
/// }
/// ```
pub fn read_from(from: &mut impl Read) -> Option<u8>
{
        let mut buffer: [u8; 1] = [0];
        if from.read_exact(&mut buffer).is_ok() {
                Some(buffer[0])
        }
        else {
                None
        }
}

/// Reads byte from stdin.
///  
/// # Usage
///
/// ```no_run
/// use terminal::io::read;
///
/// if let Some(byte) = read() {
///         println!("{byte}");
/// }
/// ```
pub fn read() -> Option<u8>
{
        read_from(&mut io::stdin())
}

/// Flushes `to`.
///
/// # Usage
///
/// ```no_run
/// use std::io;
/// use terminal::io::flush_to;
///
/// let mut to = io::stdout();
/// flush_to(&mut to).unwrap();
/// ```
pub fn flush_to(to: &mut dyn Write) -> Result<()>
{
        to.flush().map_err(|_| Error("failed to flush"))
}

/// Flushes stdout.
pub fn flush() -> Result<()>
{
        flush_to(&mut io::stdout())
}

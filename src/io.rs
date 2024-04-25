use crate::error::{
        Error,
        Result,
};
use std::io::{
        self,
        Read,
        Write,
};

/// Writes `content` to `output`.
///
/// # Usage
///
/// ```no_run
/// use ruterm::io::write_to;
/// use std::io;
///
/// let mut output = io::stdout();
/// write_to(&mut output, b"Hello").unwrap();
/// ```
pub fn write_to(output: &mut dyn Write, content: &[u8]) -> Result<usize>
{
        output.write(content).map_err(|_| Error("failed to write"))
}

/// Writes `content` to stdout.
///
/// # Usage
///
/// ```no_run
/// use ruterm::io::write;
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
/// use ruterm::io::read_from;
/// use std::io;
///
/// let mut input = io::stdin();
/// if let Some(byte) = read_from(&mut input) {
///         println!("{byte}");
/// }
/// ```
pub fn read_from(input: &mut impl Read) -> Option<u8>
{
        let mut buffer: [u8; 1] = [0];
        if input.read_exact(&mut buffer).is_ok() {
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
/// use ruterm::io::read;
///
/// if let Some(byte) = read() {
///         println!("{byte}");
/// }
/// ```
pub fn read() -> Option<u8>
{
        read_from(&mut io::stdin())
}

/// Flushes `output`.
///
/// # Usage
///
/// ```no_run
/// use ruterm::io::flush_to;
/// use std::io;
///
/// let mut output = io::stdout();
/// flush_to(&mut output).unwrap();
/// ```
pub fn flush_to(output: &mut dyn Write) -> Result<()>
{
        output.flush().map_err(|_| Error("failed to flush"))
}

/// Flushes stdout.
pub fn flush() -> Result<()>
{
        flush_to(&mut io::stdout())
}

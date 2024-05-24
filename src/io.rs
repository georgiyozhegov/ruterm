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
/// use ruterm::io::write_with_output;
/// use std::io;
///
/// let mut output = io::stdout();
/// write_with_output(&mut output, "Hello").unwrap();
/// ```
pub fn write_with_output<T: ToString>(output: &mut dyn Write, content: T) -> Result<usize>
{
        output.write(content.to_string().as_bytes()).map_err(|_| Error("failed to write"))
}

/// Writes `content` to stdout. Same as `write_with_output`.
pub fn write<T: ToString>(content: T) -> Result<usize>
{
        write_with_output(&mut io::stdout(), content)
}

/// Reads one byte from `input`.
///
/// # Usage
///
/// ```no_run
/// use ruterm::io::read_with_input;
/// use std::io;
///
/// let mut input = io::stdin();
/// if let Some(byte) = read_with_input(&mut input) {
///         println!("{byte}");
/// }
/// ```
pub fn read_with_input(input: &mut impl Read) -> Option<u8>
{
        let mut buffer: [u8; 1] = [0];
        if input.read_exact(&mut buffer).is_ok() {
                Some(buffer[0])
        }
        else {
                None
        }
}

/// Reads one byte from stdin. Same as `read_with_input`.
pub fn read() -> Option<u8>
{
        read_with_input(&mut io::stdin())
}

/// Flushes `output`.
///
/// # Usage
///
/// ```no_run
/// use ruterm::io::flush_with_output;
/// use std::io;
///
/// let mut output = io::stdout();
/// flush_with_output(&mut output).unwrap();
/// ```
pub fn flush_with_output(output: &mut dyn Write) -> Result<()>
{
        output.flush().map_err(|_| Error("failed to flush"))
}

/// Flushes stdout. Same as `flush_with_output`.
pub fn flush() -> Result<()>
{
        flush_with_output(&mut io::stdout())
}

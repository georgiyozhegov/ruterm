use crate::error::{
        Error,
        Result,
};
use std::io::{
        self,
        Read,
        Write,
};

/// Writes `content` to `output`. Same as [`write()`].
pub fn write_with_output<'s>(output: &mut dyn Write, content: &'s str) -> Result<usize>
{
        output.write(content.to_string().as_bytes())
                .map_err(|_| Error("failed to write"))
}

/// Writes `content` to stdout.
///
/// # Usage
///
/// ```no_run
/// use ruterm::io::write;
///
/// write("Hello").unwrap();
/// ```
pub fn write<'s>(content: &'s str) -> Result<usize>
{
        write_with_output(&mut io::stdout(), content)
}

/// Reads one byte from `input`. Same as [`read()`].
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

/// Reads one byte from stdin.
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
        read_with_input(&mut io::stdin())
}

/// Flushes `output`. Same as [`flush()`].
pub fn flush_with_output(output: &mut dyn Write) -> Result<()>
{
        output.flush().map_err(|_| Error("failed to flush"))
}

/// Flushes stdout.
pub fn flush() -> Result<()>
{
        flush_with_output(&mut io::stdout())
}

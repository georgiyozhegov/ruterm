use std::io::{
      self,
      Read,
      Write,
};



/// Writes to stdout, then flushes it.
///
/// ```
/// write(b"Hello");
/// ```
pub fn write(content: &[u8])
{
      let mut out = io::stdout();
      out.write(content).unwrap();
      out.flush().unwrap();
}


/// Reads 1 byte from stdin.
///
/// ```
/// if let Some(byte) = read() {
///       println!("{byte}");
/// }
/// ```
pub fn read() -> Option<u8>
{
      let mut buffer: [u8; 1] = [0];
      if io::stdin().read(&mut buffer).is_ok() {
            Some(buffer[0])
      }
      else {
            None
      }
}

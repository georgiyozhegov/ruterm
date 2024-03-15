use crate::io::write;
use base::result::{
      Error,
      Result,
};
use std::io;



const MIN_RESPONSE_LEN: usize = 6;


fn response() -> Result<String>
{
      write(b"\x1b[6n"); // Get cursor position
      let mut response = String::new();
      io::stdin()
            .read_line(&mut response)
            .map_err(|_| Error::Terminal("failed to read terminal response"))?;
      if response.len() < MIN_RESPONSE_LEN {
            Err(Error::Terminal("invalid terminal response"))
      }
      else {
            Ok(response)
      }
}


/// Gets cursor position.
///
/// Start position is top left corner of terminal window.
///
/// # Usage
///
/// ```
/// let (x, y) = get().unwrap();
/// ```
pub fn get() -> Result<(u16, u16)>
{
      let response = response()?;
      let position = &response[3..response.len() - 1]
            .split(';')
            .map(|s| s.parse::<u16>().unwrap()) // Terminal won't sen incorrect numbers
            .collect::<Vec<_>>();
      Ok((position[0], position[1]))
}


/// Sets cursor position.
///
/// Start position is top left corner of terminal window.
///
/// # Usage
///
/// ```
/// set(2, 5);
/// ```
pub fn set(x: u16, y: u16)
{
      write(format!("\x1b[{};{}H", y, x).as_bytes())
}


/// Direction of cursor movement.
pub enum Direction
{
      Left,
      Down,
      Up,
      Right,
}

impl ToString for Direction
{
      fn to_string(&self) -> String
      {
            match self {
                  Self::Left => 'D',
                  Self::Down => 'B',
                  Self::Up => 'A',
                  Self::Right => 'C',
            }
            .to_string()
      }
}


/// Moves cursor in the terminal window.
///
/// # Usage
///
/// ```
/// move_(Direction::Up, 1); // move up once
/// move_(Direction::Right, 2); // move right twice
/// ```
pub fn move_(direction: Direction, distance: u16)
{
      write(format!("\x1b[{}{}", distance, direction.to_string()).as_bytes());
}


/// Moves curosr on the start position and clears the screen.
pub fn start()
{
      write(b"\x1b[2J");
}


/// Hides cursor.
pub fn hide()
{
      write(b"\x1b[?25l");
}


/// Shows cursor.
pub fn show()
{
      write(b"\x1b[?25h");
}

use crate::error::{
        Error,
        Result,
};
use crate::io::write;
use std::io;

const MIN_RESPONSE_LEN: usize = 6;

fn response() -> Result<String>
{
        write(b"\x1b[6n")?; // Get cursor position
        let mut response = String::new();
        io::stdin()
                .read_line(&mut response)
                .map_err(|_| Error("failed to read terminal response"))?;
        if response.len() < MIN_RESPONSE_LEN {
                Err(Error("invalid terminal response"))
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
/// ```no_run
/// use terminal::cursor;
///
/// let (x, y) = cursor::get().unwrap();
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
/// use terminal::cursor;
///
/// cursor::set(2, 5);
/// ```
pub fn set(x: u16, y: u16) -> Result<usize>
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
/// use terminal::cursor::{
///         self,
///         Direction,
/// };
///
/// cursor::move_(Direction::Up, 1); // move up once
/// cursor::move_(Direction::Right, 2); // move right twice
/// ```
pub fn move_(direction: Direction, distance: u16) -> Result<usize>
{
        write(format!("\x1b[{}{}", distance, direction.to_string()).as_bytes())
}

/// Moves cursor on the start position and clears the screen.
pub fn start() -> Result<usize>
{
        write(b"\x1b[2J")
}

/// Hides cursor.
pub fn hide() -> Result<usize>
{
        write(b"\x1b[?25l")
}

/// Shows cursor.
pub fn show() -> Result<usize>
{
        write(b"\x1b[?25h")
}

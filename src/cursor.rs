use crate::{
        error::{
                Error,
                Result,
        },
        io::write,
};
use std::io;

const MIN_RESPONSE_LENGTH: usize = 6;
const RESPONSE_NUMBER_OF_PARAMETERS: usize = 2;

fn response() -> Result<String>
{
        write(b"\x1b[6n")?; // Get cursor position
        let mut response = String::new();
        io::stdin()
                .read_line(&mut response)
                .map_err(|_| Error("failed to read terminal response"))?;
        if response.len() < MIN_RESPONSE_LENGTH {
                Err(Error("invalid length of terminal response"))
        }
        else {
                Ok(response)
        }
}

fn position(response: String) -> Result<(u16, u16)>
{
        let response = response[3..response.len() - 1]
                .split(';')
                .map(|s| s.parse::<u16>().unwrap())
                .collect::<Vec<_>>();
        if response.len() != RESPONSE_NUMBER_OF_PARAMETERS {
                Err(Error("invalid number of terminal response parameters"))
        }
        else {
                Ok((response[0], response[1]))
        }
}

/// Gets cursor position.
///
/// Start position is top left corner of terminal window.
///
/// # Usage
///
/// ```no_run
/// use ruterm::cursor;
///
/// let (x, y) = cursor::get().unwrap();
/// ```
pub fn get() -> Result<(u16, u16)>
{
        let response = response()?;
        position(response)
}

/// Sets cursor position.
///
/// Start position is top left corner of terminal window.
///
/// # Usage
///
/// ```no_run
/// use ruterm::cursor;
///
/// cursor::set(2, 5).unwrap();
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
/// ```no_run
/// use ruterm::cursor::{
///         self,
///         Direction,
/// };
///
/// cursor::move_(Direction::Up, 1).unwrap(); // move up once
/// cursor::move_(Direction::Right, 2).unwrap(); // move right twice
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

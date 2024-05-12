use crate::{
        error::{
                Error,
                Result,
        },
        io::{
                write,
                write_to,
        },
};
use std::io::{
        self,
        BufRead,
        Write,
};

const MIN_RESPONSE_LENGTH: usize = 6;
const RESPONSE_NUMBER_OF_PARAMETERS: usize = 2;

fn response(input: &mut dyn BufRead) -> Result<String>
{
        write(b"\x1b[6n")?; // get cursor position
        let mut response = String::new();
        input.read_line(&mut response)
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

/// Gets cursor position. Uses `input`.
///
/// Start position is top left corner of terminal window.
///
/// # Usage
///
/// ```no_run
/// use ruterm::cursor;
/// use std::io;
///
/// let mut input = io::stdin();
/// let (x, y) = cursor::get_from(&mut input.lock()).unwrap();
/// ```
pub fn get_from(input: &mut dyn BufRead) -> Result<(u16, u16)>
{
        let response = response(input)?;
        position(response)
}

/// Gets cursor position. Uses stdin. Same as `get_from`.
pub fn get() -> Result<(u16, u16)>
{
        let response = response(&mut io::stdin().lock())?;
        position(response)
}

/// Sets cursor position. Writes to `output`.
///
/// Start position is top left corner of terminal window.
///
/// # Usage
///
/// ```no_run
/// use ruterm::cursor;
/// use std::io;
///
/// let mut output = io::stdout();
/// cursor::set_to(&mut output, 2, 5).unwrap();
/// ```
pub fn set_to(output: &mut dyn Write, x: u16, y: u16) -> Result<usize>
{
        write_to(output, format!("\x1b[{};{}H", y, x).as_bytes())
}

// Sets cursor position. Writes to stdout. Same as `set_to`.
pub fn set(x: u16, y: u16) -> Result<usize>
{
        set_to(&mut io::stdout(), x, y)
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

/// Moves cursor in the terminal window. Writes to `output`.
///
/// # Usage
///
/// ```no_run
/// use ruterm::cursor::{
///         self,
///         Direction,
/// };
/// use std::io;
///
/// let mut output = io::stdout();
/// cursor::move_to(&mut output, Direction::Up, 1).unwrap(); // move up once
/// cursor::move_to(&mut output, Direction::Right, 2).unwrap(); // move right twice
/// ```
pub fn move_to(output: &mut dyn Write, direction: Direction, distance: u16) -> Result<usize>
{
        write_to(
                output,
                format!("\x1b[{}{}", distance, direction.to_string()).as_bytes(),
        )
}

/// Moves cursor in the terminal window. Writes to stdout. Same as `move_`.
pub fn move_(direction: Direction, distance: u16) -> Result<usize>
{
        move_to(&mut io::stdout(), direction, distance)
}

/// Moves cursor on the start position and clears the screen. Writes to `output`.
pub fn start_to(output: &mut dyn Write) -> Result<usize>
{
        write_to(output, b"\x1b[2J")
}

/// Moves cursor on the start position and clears the screen. Writes to stdout. Same as `start_to`.
pub fn start() -> Result<usize>
{
        start_to(&mut io::stdout())
}

/// Hides cursor. Writes to `output`.
pub fn hide_to(output: &mut dyn Write) -> Result<usize>
{
        write_to(output, b"\x1b[?25l")
}

/// Hides cursor. Writes to stdout. Same as `hide_to`.
pub fn hide() -> Result<usize>
{
        hide_to(&mut io::stdout())
}

/// Shows cursor. Writes to `output`.
pub fn show_to(output: &mut dyn Write) -> Result<usize>
{
        write_to(output, b"\x1b[?25h")
}

/// Shows cursor. Writes to stdout. Smae as `show_to`.
pub fn show() -> Result<usize>
{
        show_to(&mut io::stdout())
}

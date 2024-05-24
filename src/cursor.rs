use crate::{
        error::{
                Error,
                Result,
        },
        io::{
                write,
                write_with_output,
        },
};
use std::io::{
        self,
        BufRead,
        Write,
};

const MIN_RESPONSE_LENGTH: usize = 6;
const POSITION_LENGTH: usize = 2;

fn response(input: &mut dyn BufRead) -> Result<String>
{
        write("\x1b[6n")?;
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
        if response.len() != POSITION_LENGTH {
                Err(Error("invalid number of terminal response parameters"))
        }
        else {
                Ok((response[0], response[1]))
        }
}

/// Gets cursor position. Reads from `input`. Same as [`get`] function.
pub fn get_with_input(input: &mut dyn BufRead) -> Result<(u16, u16)>
{
        let response = response(input)?;
        position(response)
}

/// Gets cursor position. Reads from stdin.
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
        get_with_input(&mut io::stdin().lock())
}

/// Sets cursor position. Writes to `output`. Same as [`set`] function.
pub fn set_with_output(output: &mut dyn Write, x: u16, y: u16) -> Result<usize>
{
        write_with_output(output, format!("\x1b[{};{}H", y, x))
}

/// Sets cursor position. Writes to stdout.
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
        set_with_output(&mut io::stdout(), x, y)
}

/// Direction of cursor movement. Used in [`move_`] function.
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

/// Moves cursor in the terminal window. Writes to `output`. Same as [`move_`].
pub fn move_with_output(
        output: &mut dyn Write,
        direction: Direction,
        distance: u16,
) -> Result<usize>
{
        write_with_output(
                output,
                format!("\x1b[{}{}", distance, direction.to_string()),
        )
}

/// Moves cursor in the terminal window. Writes to stdout.
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
        move_with_output(&mut io::stdout(), direction, distance)
}

/// Moves cursor on the start position and clears the screen. Writes to `output`. Same as [`start`].
pub fn start_with_output(output: &mut dyn Write) -> Result<usize>
{
        write_with_output(output, "\x1b[2J")
}

/// Moves cursor on the start position and clears the screen. Writes to stdout.
pub fn start() -> Result<usize>
{
        start_with_output(&mut io::stdout())
}

/// Hides cursor. Writes to `output`. Same as [`hide`].
pub fn hide_with_output(output: &mut dyn Write) -> Result<usize>
{
        write_with_output(output, "\x1b[?25l")
}

/// Hides cursor. Writes to stdout.
pub fn hide() -> Result<usize>
{
        hide_with_output(&mut io::stdout())
}

/// Shows cursor. Writes to `output`. Same as [`show`].
pub fn show_with_output(output: &mut dyn Write) -> Result<usize>
{
        write_with_output(output, "\x1b[?25h")
}

/// Shows cursor. Writes to stdout.
pub fn show() -> Result<usize>
{
        show_with_output(&mut io::stdout())
}

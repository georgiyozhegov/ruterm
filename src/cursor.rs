use crate::{
        error::Result,
        tio::write_with_output,
};
use std::io::{
        self,
        Write,
};

/// Sets cursor position. Writes to `output`. Same as [`set()`].
pub fn set_with_output(output: &mut dyn Write, x: u16, y: u16) -> Result<usize>
{
        write_with_output(output, format!("\x1b[{};{}H", y, x).as_str())
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

/// Direction of cursor movement. Used in [`move_()`].
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

/// Moves cursor in the terminal window. Writes to `output`. Same as [`move_()`].
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

/// Moves cursor on the start position and clears the screen. Writes to `output`. Same as [`start()`].
pub fn start_with_output(output: &mut dyn Write) -> Result<usize>
{
        write_with_output(output, "\x1b[2J")
}

/// Moves cursor on the start position and clears the screen. Writes to stdout.
pub fn start() -> Result<usize>
{
        start_with_output(&mut io::stdout())
}

/// Makes cursor invisible. Writes to `output`. Same as [`hide()`].
pub fn hide_with_output(output: &mut dyn Write) -> Result<usize>
{
        write_with_output(output, "\x1b[?25l")
}

/// Makes cursor invisible. Writes to stdout.
pub fn hide() -> Result<usize>
{
        hide_with_output(&mut io::stdout())
}

/// Makes cursor visible. Writes to `output`. Same as [`show()`].
pub fn show_with_output(output: &mut dyn Write) -> Result<usize>
{
        write_with_output(output, "\x1b[?25h")
}

/// Makes cursor visible. Writes to stdout.
pub fn show() -> Result<usize>
{
        show_with_output(&mut io::stdout())
}

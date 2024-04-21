use crate::{
        cursor::{
                self,
                Direction,
        },
        error::Result,
        io::write_to,
};
use std::io::{
        self,
        Write,
};

/// Writes `text` to `to` in rendered form without need to add cursor manipulation stuff.
///
/// # Usage
///
/// ```no_run
/// use std::io;
/// use terminal::render::render_to;
///
/// let mut output = io::stdout();
/// render_to(
///         &mut output,
///         vec!["* *", " * ", "* *"]
///                 .iter()
///                 .map(|string| string.to_string())
///                 .collect(), // convert &str into String
/// )
/// .unwrap();
/// ```
pub fn render_to(output: &mut dyn Write, text: Vec<String>) -> Result<()>
{
        for line in text {
                write_to(output, line.as_bytes())?;
                cursor::move_(Direction::Down, 1)?;
                cursor::move_(Direction::Left, line.len() as u16)?;
        }
        Ok(())
}

/// Writes `text` to stdout in rendered form without need to add cursor manipulation stuff.
///
/// # Usage
///
/// ```no_run
/// use terminal::render::render;
///
/// render(
///         vec!["* *", " * ", "* *"]
///                 .iter()
///                 .map(|string| string.to_string())
///                 .collect(), // convert &str into String
/// )
/// .unwrap();
/// ```
pub fn render(text: Vec<String>) -> Result<()>
{
        render_to(&mut io::stdout(), text)
}

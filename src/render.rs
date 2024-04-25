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

/// Writes `text` to `output` in rendered form.
///
/// # Usage
///
/// ```no_run
/// use ruterm::render::render_to;
/// use std::io;
///
/// let mut output = io::stdout();
/// render_to(
///         &mut output,
///         vec!["* *", " * ", "* *"] // each element of the vec represents new line
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

/// Writes `text` to stdout in rendered form.
///
/// # Usage
///
/// ```no_run
/// use ruterm::render::render;
///
/// render(
///         vec!["* *", " * ", "* *"] // each element of the vec represents new line
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

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

pub const END: &str = "\n";

/// Writes `text` to `output` in rendered form.
///
/// # Usage
///
/// ```no_run
/// use ruterm::render::{
///         render_to,
///         END,
/// };
/// use std::io;
///
/// let mut output = io::stdout();
/// render_to(&mut output, vec!["* *", END, " * ", END, "* *"]).unwrap();
/// ```
pub fn render_to<T>(output: &mut dyn Write, text: Vec<T>) -> Result<()>
where
        T: ToString,
{
        let mut shift = 0;
        for line in text {
                let line = line.to_string();
                match line.as_str() {
                        END => {
                                cursor::move_(Direction::Down, 1)?;
                                cursor::move_(Direction::Left, shift)?;
                                shift = 0;
                        }
                        _ => {
                                write_to(output, line.as_bytes())?;
                                if !line.starts_with("\x1b") {
                                        shift += line.len() as u16;
                                }
                        }
                }
        }
        Ok(())
}

/// Writes `text` to stdout in rendered form.
///
/// # Usage
///
/// ```no_run
/// use ruterm::render::{
///         render,
///         END,
/// };
///
/// render(vec!["* *", END, " * ", END, "* *"]).unwrap();
/// ```
pub fn render<T>(text: Vec<T>) -> Result<()>
where
        T: ToString,
{
        render_to(&mut io::stdout(), text)
}

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
use std::str::Chars;

pub const ESCAPE_END: [char; 9] = ['m', 'A', 'B', 'C', 'D', 'H', 'J', 'l', 'h'];

fn skip_invisible(line: &mut Chars)
{
        while let Some(c) = line.next() {
                if ESCAPE_END.contains(&c) {
                        break;
                }
        }
}

fn visible_length(line: &String) -> u16
{
        let mut length = 0;
        let mut line = line.chars();
        while let Some(c) = line.next() {
                if c == '\x1b' {
                        skip_invisible(&mut line);
                }
                else {
                        length += 1;
                }
        }
        length
}

pub const END: &str = "\n";

/// Writes `text` to `output` in rendered form.
///
/// # Usage
///
/// ```no_run
/// use ruterm::render::{
///         render_to,
///         END, // moves to the newline
/// };
/// use std::io;
///
/// let mut output = io::stdout();
/// render_to(&mut output, vec!["* *", END, " * ", END, "* *", END]).unwrap();
/// ```
///
/// You can apply style to the text:
/// ```no_run
/// use ruterm::{
///         render::{
///                 render_to,
///                 END,
///         },
///         style::{
///                 color::fore,
///                 RESET,
///         },
/// };
/// use std::io;
///
/// let mut output = io::stdout();
/// render_to(
///         &mut output,
///         vec![
///                 fore::RED,
///                 "red",
///                 END,
///                 fore::GREEN,
///                 "green",
///                 END,
///                 fore::BLUE,
///                 "blue",
///                 RESET, // don't forget to reset style!
///         ],
/// )
/// .unwrap();
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
                                cursor::move_to(output, Direction::Down, 1)?;
                                cursor::move_to(output, Direction::Left, shift)?;
                                shift = 0;
                        }
                        _ => {
                                write_to(output, line.as_bytes())?;
                                shift += visible_length(&line);
                        }
                }
        }
        Ok(())
}

/// Writes `text` to stdout in rendered form. Same as `render_to`.
pub fn render<T>(text: Vec<T>) -> Result<()>
where
        T: ToString,
{
        render_to(&mut io::stdout(), text)
}

#[cfg(test)]
mod tests
{
        use super::*;

        #[test]
        fn visible_length_()
        {
                let code = concat!("Hi", "\x1b[?25h", "Hello").to_string();
                assert_eq!(("Hi".len() + "Hello".len()) as u16, visible_length(&code));
        }

        #[test]
        fn render_()
        {
                let text = vec!["1line", "1line", END, "2line"];
                let mut buffer = Vec::new();
                render_to(&mut buffer, text).unwrap();
                assert_eq!(
                        concat!("1line", "1line", "\x1b[1B", "\x1b[10D", "2line").to_string(),
                        buffer.iter().map(|b| *b as char).collect::<String>()
                );
        }
}

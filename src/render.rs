use crate::{
        cursor::{
                self,
                Direction,
        },
        error::Result,
        io::write_with_output,
};
use std::io::{
        self,
        Write,
};
use std::str::Chars;

const ESCAPE_END: [char; 9] = ['m', 'A', 'B', 'C', 'D', 'H', 'J', 'l', 'h'];

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

/// Moves cursor to the beginning of the next line. Used in [`render`] function.
pub const END: &str = "__render_END__";

/// Writes `text` to `output` in rendered form. Same as [`render`].
pub fn render_with_output<T>(output: &mut dyn Write, text: Vec<T>) -> Result<()>
where
        T: ToString,
{
        let mut shift = 0;
        for line in text {
                let line = line.to_string();
                match line.as_str() {
                        END => {
                                cursor::move_with_output(output, Direction::Down, 1)?;
                                cursor::move_with_output(output, Direction::Left, shift)?;
                                shift = 0;
                        }
                        _ => {
                                write_with_output(output, &line)?;
                                shift += visible_length(&line);
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
///         END, // moves to the newline
/// };
///
/// render(vec!["* *", END, " * ", END, "* *", END]).unwrap();
/// ```
///
/// You can apply style to the text:
/// ```no_run
/// use ruterm::{
///         render::{
///                 render,
///                 END,
///         },
///         view::{
///                 color::fore,
///                 RESET,
///         },
/// };
///
/// render(vec![
///         fore::RED,
///         "red",
///         END,
///         fore::GREEN,
///         "green",
///         END,
///         fore::BLUE,
///         "blue",
///         RESET, // don't forget to reset style!
/// ])
/// .unwrap();
/// ```
pub fn render<T>(text: Vec<T>) -> Result<()>
where
        T: ToString,
{
        render_with_output(&mut io::stdout(), text)
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
                render_with_output(&mut buffer, text).unwrap();
                assert_eq!(
                        concat!("1line", "1line", "\x1b[1B", "\x1b[10D", "2line").to_string(),
                        buffer.iter().map(|b| *b as char).collect::<String>()
                );
        }
}

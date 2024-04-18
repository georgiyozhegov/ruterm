use crate::{
        cursor::{
                self,
                Direction,
        },
        error::Result,
        io,
};
use std::io::{
        self as io_,
        Read,
        Write,
};

pub fn render_to(to: &mut dyn Write, text: String) -> Result<()>
{
        for line in text.lines() {
                io::write_to(to, line.as_bytes())?;
                cursor::move_(Direction::Down, 1)?;
                cursor::move_(Direction::Left, line.len() as u16)?;
        }
        Ok(())
}

pub fn render(text: String) -> Result<()>
{
        render_to(&mut io_::stdout(), text)
}

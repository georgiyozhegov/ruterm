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

pub fn render_to(to: &mut dyn Write, text: Vec<String>) -> Result<()>
{
        for line in text {
                write_to(to, line.as_bytes())?;
                cursor::move_(Direction::Down, 1)?;
                cursor::move_(Direction::Left, line.len() as u16)?;
        }
        Ok(())
}

pub fn render(text: Vec<String>) -> Result<()>
{
        render_to(&mut io::stdout(), text)
}

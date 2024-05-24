use ruterm::{
        cursor,
        error::Result,
        in_raw,
        io::write,
        size,
};

fn main() -> Result<()>
{
        in_raw!({
                cursor::start()?; // clear screen
                let (w, h) = size()?;
                cursor::set(w / 2, h / 2)?; // sets cursor in the center
                write("Hello from raw mode!\n\r")?;
        });

        Ok(())
}

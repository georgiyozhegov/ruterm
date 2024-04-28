use ruterm::{
        cursor,
        error::Result,
        in_raw,
        io::write,
        size,
        style::{
                color::fore,
                RESET,
        },
};

fn main() -> Result<()>
{
        in_raw!({
                cursor::start()?; // clear screen
                let (w, h) = size()?;
                cursor::set(w / 2, h / 2)?; // move cursor to the center
                write(fore::GREEN.as_bytes())?; // green foreground
                write(b"Hello from raw mode!\n\r")?;
                write(RESET.as_bytes())?; // reset style
                cursor::set(0, h)?; // move cursor to the bottom
        });

        Ok(())
}

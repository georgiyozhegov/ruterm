use std::{
        thread::sleep,
        time::Duration,
};
use terminal::{
        cursor::{
                self,
                Direction,
        },
        error::Result,
        in_raw,
        io,
        size,
};

const SPEED: u16 = 2;

macro_rules! fps {
        ($fps:expr) => {
                1000 / $fps
        };
}

fn draw(x: u16, y: u16) -> Result<()>
{
        cursor::set(x, y)?;
        io::write(b"o==o\n")?;
        cursor::move_(Direction::Left, 4)?;
        io::write(b"|  |\n")?;
        cursor::move_(Direction::Left, 4)?;
        io::write(b"|##|\n")?;
        cursor::move_(Direction::Left, 4)?;
        io::write(b"*==*\n")?;
        cursor::move_(Direction::Left, 4)?;
        Ok(())
}

fn update(x: &mut u16, y: &mut u16, w: i16, h: i16, key: Option<u8>)
{
        match key {
                Some(b'h') => {
                        if (*x as i16 - SPEED as i16) > 0 {
                                *x -= SPEED
                        }
                }
                Some(b'j') => {
                        if (*y + SPEED) < h as u16 {
                                *y += SPEED
                        }
                }
                Some(b'k') => {
                        if (*y as i16 - SPEED as i16) > 0 {
                                *y -= SPEED
                        }
                }
                Some(b'l') => {
                        if (*x - SPEED) < w as u16 {
                                *x += SPEED
                        }
                }
                _ => {}
        }
}

fn delay(delay: u64)
{
        sleep(Duration::from_millis(delay));
}

fn main() -> Result<()>
{
        let mut key;
        let (w, h) = size()?;
        let (mut x, mut y) = (w / 2, h / 2);

        in_raw!({
                cursor::hide()?;

                loop {
                        cursor::start()?;
                        key = io::read();
                        if key == Some(b'q') {
                                cursor::set(0, h)?;
                                break;
                        }
                        update(&mut x, &mut y, w as i16, h as i16, key);
                        draw(x, y)?;
                        delay(fps!(60));
                }

                cursor::show()?;
        });

        Ok(())
}

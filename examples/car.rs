use std::{
        process::exit,
        task::Wake,
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

const SPEED: u16 = 1;

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

fn start(w: u16, h: u16) -> Result<bool>
{
        cursor::set(w / 2 - 11, h / 2)?;
        io::write(b"Press 'h' to move left\n")?;
        cursor::move_(Direction::Left, 22)?;
        io::write(b"      'j' to move up\n")?;
        cursor::move_(Direction::Left, 14 + 6)?;
        io::write(b"      'k' to move down\n")?;
        cursor::move_(Direction::Left, 16 + 6)?;
        io::write(b"      'l' to move right.\n")?;
        cursor::move_(Direction::Left, 18 + 6)?;
        io::write(b"      'q' to quit and 's' to start...\n")?;

        loop {
                match io::read() {
                        Some(b'q') => return Ok(false),
                        Some(b's') => return Ok(true),
                        _ => {}
                }
        }
}

fn game(x: &mut u16, y: &mut u16, w: u16, h: u16, key: &mut Option<u8>) -> Result<()>
{
        loop {
                cursor::start()?;
                draw(*x, *y)?;
                *key = io::read();
                if *key == Some(b'q') {
                        cursor::set(0, h)?;
                        break;
                }
                update(x, y, w as i16, h as i16, *key);
                delay(fps!(100));
        }
        Ok(())
}

fn main() -> Result<()>
{
        let mut key = None;
        let (w, h) = size()?;
        let (mut x, mut y) = (w / 2, h / 2);

        in_raw!({
                cursor::hide()?;
                cursor::start()?;

                if start(w, h)? {
                        game(&mut x, &mut y, w, h, &mut key)?
                }

                cursor::set(0, h)?;
                cursor::show()?;
        });

        Ok(())
}

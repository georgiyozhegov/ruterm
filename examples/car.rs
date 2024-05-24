use ruterm::{
        cursor,
        error::Result,
        in_raw,
        io,
        render::{
                render,
                render_with_output,
                END,
        },
        size,
        view::{
                color::fore,
                RESET,
        },
};
use std::{
        io::{
                self as io_,
                Stdout,
        },
        thread::sleep,
        time::Duration,
};

const SPEED: u16 = 1;
const FPS: u64 = 60;

macro_rules! fps {
        ($fps:expr) => {
                1000 / $fps
        };
}

#[rustfmt::skip]
fn draw(x: u16, y: u16, out: &mut Stdout) -> Result<()>
{
        cursor::set(x, y)?;
        render_with_output(
                out,
                vec![
                        fore::YELLOW, "o", fore::GREEN, "==", fore::YELLOW, "o",
                        RESET,
                        END,
                        fore::GREEN, "|  |",
                        END,
                        "|XX|",
                        RESET,
                        END,
                        fore::RED, "*", fore::GREEN, "==", fore::RED, "*", RESET,
                        END,
                ],
        )?;
        io::flush_with_output(out)?;
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

enum Mode
{
        Play,
        Exit,
}

#[rustfmt::skip]
fn start(w: u16, h: u16) -> Result<Mode>
{
        cursor::set(w / 2 - 11, h / 2)?;
        render(vec![
                "Press ",
                "'h'",
                " to move left", END,
                "      'j' to move up", END,
                "      'k' to move down", END,
                "      'l' to move right", END,
                "      'q' to quit", END,
                "      'p' to play", END,
        ])?;
        io::flush()?;

        loop {
                match io::read() {
                        Some(b'q') => return Ok(Mode::Exit),
                        Some(b'p') => return Ok(Mode::Play),
                        _ => {}
                }
        }
}

fn game(x: &mut u16, y: &mut u16, w: u16, h: u16, key: &mut Option<u8>) -> Result<()>
{
        let mut stdout = io_::stdout();
        let mut stdin = io_::stdin();
        loop {
                cursor::start()?;
                draw(*x, *y, &mut stdout)?;
                *key = io::read_with_input(&mut stdin);
                if *key == Some(b'q') {
                        cursor::set(0, h)?;
                        break;
                }
                update(x, y, w as i16, h as i16, *key);
                delay(fps!(60));
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

                match start(w, h)? {
                        Mode::Play => game(&mut x, &mut y, w, h, &mut key)?,
                        Mode::Exit => {}
                }

                cursor::set(0, h)?;
                cursor::show()?;
        });

        Ok(())
}

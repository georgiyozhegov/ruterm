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

const CAR_SPEED: u16 = 1;
const FPS: u64 = 60;

struct State
{
        key: Option<u8>,
        width: u16,
        height: u16,
        car_x: u16,
        car_y: u16,
}

impl State
{
        pub fn new() -> Result<Self>
        {
                let (width, height) = size()?;
                Ok(Self {
                        key: None,
                        width,
                        height,
                        car_x: width / 2,
                        car_y: height / 2,
                })
        }
}

#[rustfmt::skip]
fn draw(car_x: u16, car_y: u16, output: &mut Stdout) -> Result<()>
{
        cursor::set(car_x, car_y)?;
        render_with_output(
                output,
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
        io::flush_with_output(output)?;
        Ok(())
}

fn update(state: &mut State)
{
        match state.key {
                Some(b'h') => {
                        if (state.car_x as i16 - CAR_SPEED as i16) > 0 {
                                state.car_x -= CAR_SPEED
                        }
                }
                Some(b'j') => {
                        if (state.car_y + CAR_SPEED) < state.height as u16 {
                                state.car_y += CAR_SPEED
                        }
                }
                Some(b'k') => {
                        if (state.car_y as i16 - CAR_SPEED as i16) > 0 {
                                state.car_y -= CAR_SPEED
                        }
                }
                Some(b'l') => {
                        if (state.car_x - CAR_SPEED) < state.width as u16 {
                                state.car_x += CAR_SPEED
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
fn start(width: u16, height: u16) -> Result<Mode>
{
        cursor::set(width / 2 - 11, height / 2)?;
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

fn game(state: &mut State) -> Result<()>
{
        let mut stdout = io_::stdout();
        let mut stdin = io_::stdin();
        loop {
                cursor::start()?;
                draw(state.car_x, state.car_y, &mut stdout)?;
                state.key = io::read_with_input(&mut stdin);
                if state.key == Some(b'q') {
                        cursor::set(0, state.height)?;
                        break;
                }
                update(state);
                delay(1000 / FPS);
        }
        Ok(())
}

fn main() -> Result<()>
{
        let mut state = State::new()?;

        in_raw!({
                cursor::hide()?;
                cursor::start()?;

                match start(state.width, state.height)? {
                        Mode::Play => game(&mut state)?,
                        Mode::Exit => {}
                }

                cursor::set(0, state.height)?;
                cursor::show()?;
        });

        Ok(())
}

use ruterm::{
        cursor,
        error::Result,
        in_raw,
        io,
        render::{
                render,
                render_to,
                END,
        },
        size,
        style::{
                color::fore,
                style,
                RESET,
        },
};
use std::{
        io::{
                self as io_,
                Stdout,
        },
        sync::{
                Arc,
                Mutex,
        },
        thread::{
                self,
                sleep,
        },
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
        render_to(
                out,
                vec![
                        fore::YELLOW, "o", RESET, "==", fore::YELLOW, "o",
                        RESET,
                        END,
                        "|  |",
                        END,
                        "|##|",
                        END,
                        "*==*",
                        END,
                        RESET,
                ],
        )?;
        io::flush_to(out)?;
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
        Sync,
        Parallel,
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
                "      's' to start sync mode", END,
                "      'p' to start parallel mode", END,
        ])?;
        io::flush()?;

        loop {
                match io::read() {
                        Some(b'q') => return Ok(Mode::Exit),
                        Some(b's') => return Ok(Mode::Sync),
                        Some(b'p') => return Ok(Mode::Parallel),
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
                *key = io::read_from(&mut stdin);
                if *key == Some(b'q') {
                        cursor::set(0, h)?;
                        break;
                }
                update(x, y, w as i16, h as i16, *key);
                delay(fps!(60));
        }
        Ok(())
}

fn game_parallel(x: Arc<Mutex<u16>>, y: Arc<Mutex<u16>>, w: u16, h: u16) -> Result<()>
{
        let x_clone = Arc::clone(&x);
        let y_clone = Arc::clone(&y);
        let run_status = Arc::new(Mutex::new(true));
        let run_status_clone = Arc::clone(&run_status);
        let update_thread = thread::spawn(move || -> Result<()> {
                let mut key;
                let mut stdin = io_::stdin();
                loop {
                        key = io::read_from(&mut stdin);
                        if key == Some(b'q') {
                                cursor::set(0, h)?;
                                *run_status.lock().unwrap() = false;
                                return Ok(());
                        }
                        update(
                                &mut x.lock().unwrap(),
                                &mut y.lock().unwrap(),
                                w as i16,
                                h as i16,
                                key,
                        );
                }
        });
        let draw_thread = thread::spawn(move || -> Result<()> {
                let mut stdout = io_::stdout();
                loop {
                        if !*run_status_clone.lock().unwrap() {
                                return Ok(());
                        }
                        cursor::start()?;
                        draw(
                                *x_clone.lock().unwrap(),
                                *y_clone.lock().unwrap(),
                                &mut stdout,
                        )?;
                        delay(fps!(FPS));
                }
        });

        draw_thread.join().unwrap()?;
        update_thread.join().unwrap()?;

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
                        Mode::Sync => game(&mut x, &mut y, w, h, &mut key)?,
                        Mode::Parallel => game_parallel(
                                Arc::new(Mutex::new(x)),
                                Arc::new(Mutex::new(y)),
                                w,
                                h,
                        )?,
                        Mode::Exit => {}
                }

                cursor::set(0, h)?;
                cursor::show()?;
        });

        Ok(())
}

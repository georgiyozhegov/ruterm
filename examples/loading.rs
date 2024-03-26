use std::{
        thread::sleep as sleep_,
        time::Duration,
};
use terminal::{
        cursor,
        error::Result,
        in_raw,
        io::write,
        size,
};

fn sleep(delay: u64)
{
        sleep_(Duration::from_millis(delay));
}

fn center() -> Result<(u16, u16)>
{
        let (w, h) = size()?;
        Ok((w / 2, h / 2))
}

fn main() -> Result<()>
{
        let delay = Duration::from_millis(100);
        let rotations = 10;
        let radius = 5;

        cursor::hide();
        in_raw!({
                let (cx, cy) = center()?;
                let (mut x, mut y) = (cx - radius, cy - radius);
                let (mut mx, mut my) = (0, 0);

                for r in 1..rotations {
                        update(cx, cy, &mut mx, &mut my, &mut x, &mut y);
                        sleep(delay);
                }
        });
        cursor::show();

        Ok(())
}

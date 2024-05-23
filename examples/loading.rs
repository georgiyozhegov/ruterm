use ruterm::{
        cursor,
        error::Result,
        in_raw,
        io,
        size,
};
use std::{
        f64::consts::PI,
        thread::sleep,
        time::Duration,
};

fn coordinates(
        center_x: u16,
        center_y: u16,
        scale_x: f64,
        scale_y: f64,
        radius: u16,
        n_points: u32,
) -> Vec<(u16, u16)>
{
        let mut coordinates = Vec::new();
        let (mut t, mut x, mut y);
        for i in 1..n_points {
                t = 2.0 * PI * i as f64 / n_points as f64;
                x = center_x as f64 + (radius as f64 * t.cos()) * scale_x;
                y = center_y as f64 + (radius as f64 * t.sin()) * scale_y;
                coordinates.push((x as u16, y as u16));
        }
        coordinates
}

fn main() -> Result<()>
{
        let mut delay_coeff;
        let delay = 90.0;
        let radius = 12;
        let n_points = 40;

        cursor::hide()?;
        in_raw!({
                let (w, h) = size()?;
                let coordinates = coordinates(w / 2, h / 2, 1.4, 0.6, radius, n_points);

                cursor::start()?;
                
                cursor::set(w / 2 - 5, h / 2)?;
                io::write(b"Loading...")?;

                for (x, y) in coordinates {
                        cursor::set(x, y)?;
                        io::write(b"O")?;
                        io::flush()?;

                        delay_coeff = y as f64 / (h as f64 / 2.0 + radius as f64);
                        sleep(Duration::from_millis((delay * delay_coeff) as u64));
                }

                cursor::set(w / 2 - 5, h / 2)?;
                io::write(b"Completed.")?;

                cursor::set(0, h)?;
        });
        cursor::show()?;
        Ok(())
}

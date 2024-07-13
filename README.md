<div align="center">
    <img src="https://github.com/georgiyozhegov/terminal/assets/159022025/b9bf9c24-486c-44b1-ac9e-e856426c2acc" width=240px height=240px>
    <h1 align="center">Ruterm</h1>
    <p align="center">Tiny (~400 loc) library for working with the terminal</p>
    
[![docs.rs](https://img.shields.io/docsrs/ruterm)](https://docs.rs/ruterm/latest/ruterm/)
[![crates.io](https://img.shields.io/crates/d/ruterm)](https://crates.io/crates/ruterm)
![GitHub License](https://img.shields.io/github/license/georgiyozhegov/terminal)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/georgiyozhegov/terminal)
</div>

# Examples

<img style="height: 300px; width: auto;" src="https://github.com/georgiyozhegov/ruterm/assets/159022025/0575c990-7527-41ca-9068-eb88f4f27825">
<img style="height: 300px; weight: auto;"src="https://github.com/georgiyozhegov/ruterm/assets/159022025/93541f66-fb79-41ee-b564-8aaac6316564">


# Usage

```rust
use ruterm::{
    error::Result,
    in_raw,
    view::{color::fore, RESET},
    tio::write,
    size,
    cursor,
};

fn main() -> Result<()> {
    in_raw!({
        cursor::start()?; // clear screen
        let (w, h) = size()?;
        cursor::set(w / 2, h / 2)?; // move cursor to the center
        write(fore::GREEN)?; // green foreground
        write("Hello from raw mode!\n\r")?;
        write(RESET)?; // reset style
        cursor::set(0, h)?; // move cursor to the bottom
    });

    Ok(())
}
```

# Installation

From crates.io:
```bash
cargo add ruterm
```

From repository (more recent):
```bash
cargo add --git https://github.com/georgiyozhegov/ruterm.git
```

**Warning**: Currently, supports only Linux. <br>

# References

- [Kilo Editor](https://github.com/antirez/kilo)
- [Raw Mode Tutorial](https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html)
- [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)

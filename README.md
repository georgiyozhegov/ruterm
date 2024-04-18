# Installation
```bash
cargo add --git https://github.com/georgiyozhegov/terminal.git
```

# Usage
```rust
use terminal::{
    error::Result,
    in_raw,
    io::write,
    size,
    cursor,
};

fn main() -> Result<()> {
    in_raw!({
        cursor::start()?; // clear screen
        let (w, h) = size()?;
        cursor::set(w / 2, h / 2)?; // sets cursor in the center
        write(b"Hello from raw mode!\n\r")?;
    });

    Ok(())
}
```

# Examples
```bash
cargo run --example car
```
[example](https://github.com/georgiyozhegov/terminal/assets/159022025/e4c4beff-a252-425a-a6c8-a976a327c88d)

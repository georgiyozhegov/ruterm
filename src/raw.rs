use crate::error::{
        Error,
        Result,
};
use libc::STDIN_FILENO;
use termios::{
        tcsetattr,
        Termios as Termios_,
        ECHO,
        ICANON,
        ICRNL,
        IEXTEN,
        ISIG,
        IXON,
        OPOST,
        TCSAFLUSH,
        VMIN,
        VTIME,
};

fn termios_() -> Result<Termios_>
{
        Termios_::from_fd(STDIN_FILENO).map_err(|_| Error("failed to get termios from stdin"))
}

/// Termios settings.
///
/// Provides abstraction over termios settings.
///
/// # Usage
///
/// ```no_run
/// use terminal::raw::Termios;
///
/// let mut termios = Termios::new().unwrap();
/// termios.raw().unwrap(); // Enable raw mode
///
/// // ...
///
/// termios.original().unwrap(); // Restore original settings
/// ```
///
/// # Note
///
/// Contains unsafe bindings!
pub struct Termios
{
        original: Termios_,
        raw: Termios_,
}

impl Termios
{
        pub fn new() -> Result<Self>
        {
                Ok(Self {
                        original: termios_()?,
                        raw: termios_()?,
                })
        }

        pub fn raw(&mut self) -> Result<()>
        {
                self.raw.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
                self.raw.c_iflag &= !(IXON | ICRNL);
                self.raw.c_oflag &= !OPOST;
                self.raw.c_cc[VTIME] = 1;
                self.raw.c_cc[VMIN] = 0;
                tcsetattr(STDIN_FILENO, TCSAFLUSH, &self.raw)
                        .map_err(|_| Error("failed to set raw termios settings"))
        }

        pub fn original(&mut self) -> Result<()>
        {
                tcsetattr(STDIN_FILENO, TCSAFLUSH, &self.original)
                        .map_err(|_| Error("failed to restore original termios settings"))
        }
}

/// Enables raw mode in one line.
///
/// # Usage
///
/// ```ignore
/// use terminal::in_raw;
///
/// in_raw!({
///     // ...
/// });
/// ```
#[macro_export]
macro_rules! in_raw {
    ($block: block) => {
            let mut termios = terminal::raw::Termios::new()?; // no need to import
            termios.raw()?;
            $block
            termios.original()?;
    }
}

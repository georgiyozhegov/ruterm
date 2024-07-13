use crate::error::{
        Error,
        Result,
};
use std::os::fd::AsRawFd;
use termios::{
        tcsetattr,
        Termios as Termios_,
        CS8,
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

fn termios<D: AsRawFd>(fd: &D) -> Result<Termios_>
{
        Termios_::from_fd(fd.as_raw_fd()).map_err(|_| Error("Invalid file descriptor."))
}

fn raw(mut termios: Termios_) -> Termios_
{
        termios.c_lflag &= !(ECHO | ICANON | IEXTEN | ISIG);
        termios.c_iflag &= !(IXON | ICRNL);
        termios.c_oflag &= !OPOST;
        termios.c_cflag |= CS8;
        termios.c_cc[VTIME] = 1;
        termios.c_cc[VMIN] = 0;
        termios
}

/// Termios settings.
///
/// Provides abstraction over termios settings.
///
/// # Usage
///
/// ```no_run
/// use ruterm::raw::Termios;
/// use std::io;
///
/// let fd = io::stdin();
/// let mut termios = Termios::new(fd).unwrap();
/// termios.raw().unwrap(); // Enable raw mode
///
/// // ...
///
/// termios.original().unwrap(); // Restore original settings
/// ```
///
/// # Flags
///
/// - `ECHO`      disable echoing
/// - `ICANON`    read byte-by-byte
/// - `ISIG`      disable ctrl-c and ctrl-z exit
/// - `IXON`      disable software flow control
/// - `IEXTEN`    disable ctrl-v
/// - `ICRNL`     fix ctrl-m
/// - `OPOST`     disable output post-processing
/// - `VTIME`     read timeout
/// - `VMIN`      minimum number of bytes needed for read
///
/// # References
///
/// - [Kilo](https://github.com/antirez/kilo)
/// - [Tutorial](https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html)
///
/// # Note
///
/// Contains unsafe bindings!
pub struct Termios<D: AsRawFd>
{
        fd: D,
        original: Termios_,
        raw: Termios_,
}

impl<D: AsRawFd> Termios<D>
{
        pub fn new(fd: D) -> Result<Self>
        {
                Ok(Self {
                        original: termios(&fd)?,
                        raw: raw(termios(&fd)?),
                        fd, // trick: fd should be the last parameter
                })
        }

        pub fn raw(&self) -> Result<()>
        {
                tcsetattr(self.fd.as_raw_fd(), TCSAFLUSH, &self.raw)
                        .map_err(|_| Error("Failed to set raw flags."))
        }

        pub fn original(&self) -> Result<()>
        {
                tcsetattr(self.fd.as_raw_fd(), TCSAFLUSH, &self.original)
                        .map_err(|_| Error("Failed to restore original flags."))
        }
}

impl<D: AsRawFd> Drop for Termios<D>
{
        fn drop(&mut self)
        {
                self.original().unwrap();
        }
}

/// Enables raw mode in one line.
///
/// # Usage
///
/// ```ignore
/// use ruterm::in_raw;
///
/// in_raw!({
///     // ...
/// });
/// ```
#[macro_export]
macro_rules! in_raw {
        ($block: block) => {
                let fd = std::io::stdin();
                let mut termios = ruterm::raw::Termios::new(fd)?;
                termios.raw()?;
                $block
        };
}

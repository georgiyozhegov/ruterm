pub const RESET: &str = "\x1b[m";

/// Color codes.
pub mod color
{
        /// Foreground colors.
        ///
        /// # Basic usage
        ///
        /// ```no_run
        /// use ruterm::{
        ///         io::write,
        ///         style::color::fore::GREEN,
        ///         style::RESET,
        /// };
        ///
        /// let content = GREEN.to_string() + "This is a green text" + RESET;
        /// write(content.as_bytes()).unwrap();
        /// ```
        pub mod fore
        {
                pub const BLACK: &str = "\x1b[30m";
                pub const RED: &str = "\x1b[31m";
                pub const GREEN: &str = "\x1b[32m";
                pub const YELLOW: &str = "\x1b[33m";
                pub const BLUE: &str = "\x1b[34m";
                pub const MAGENTA: &str = "\x1b[35m";
                pub const CYAN: &str = "\x1b[36m";
                pub const WHITE: &str = "\x1b[37m";

                /// Foreground RGB color.
                ///
                /// # Usage
                ///
                /// ```no_run
                /// use ruterm::{
                ///         fore_rgb,
                ///         io::write,
                ///         style::RESET,
                /// };
                ///
                /// let content = fore_rgb!(255, 0, 0) + "This is a red text" + RESET;
                /// write(content.as_bytes()).unwrap();
                /// ```
                #[macro_export]
                macro_rules! fore_rgb {
                        ($r:expr, $g:expr, $b:expr) => {
                                format!("\x1b[38;2;{};{};{}", $r, $g, $b)
                        };
                }
        }

        /// Background colors.
        ///
        /// # Usage
        ///
        /// ```no_run
        /// use ruterm::{
        ///         io::write,
        ///         style::color::back::GREEN,
        ///         style::RESET,
        /// };
        ///
        /// let content = GREEN.to_string() + "This is a text with green background" + RESET;
        /// write(content.as_bytes()).unwrap();
        /// ```
        pub mod back
        {
                pub const BLACK: &str = "\x1b[40m";
                pub const RED: &str = "\x1b[41m";
                pub const GREEN: &str = "\x1b[42m";
                pub const YELLOW: &str = "\x1b[43m";
                pub const BLUE: &str = "\x1b[44m";
                pub const MAGENTA: &str = "\x1b[45m";
                pub const CYAN: &str = "\x1b[46m";
                pub const WHITE: &str = "\x1b[47m";

                /// Background RGB color.
                ///
                /// # Usage
                ///
                /// ```no_run
                /// use ruterm::{
                ///         back_rgb,
                ///         io::write,
                ///         style::RESET,
                /// };
                ///
                /// let content = back_rgb!(255, 0, 0) + "This is a text with red background" + RESET;
                /// write(content.as_bytes()).unwrap();
                /// ```
                #[macro_export]
                macro_rules! back_rgb {
                        ($r:expr, $g:expr, $b:expr) => {
                                format!("\x1b[48;2;{};{};{}", $r, $g, $b)
                        };
                }
        }
}

/// SGR codes.
pub mod style
{
        pub const BOLD: &str = "\x1b[1m";
        pub const ITALIC: &str = "\x1b[3m";
        pub const UNDERLINE: &str = "\x1b[4m";
        pub const BLINK: &str = "\x1b[5m";
        pub const STRIKE: &str = "\x1b[9m";
}

pub const RESET: &str = "\x1b[m";

pub mod color
{
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
        }

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
        }
}

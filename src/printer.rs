use std::fmt::Display;

pub trait Printer {
    /// Display normally
    fn print(&self)
    where
        Self: Display,
    {
        println!("{self}")
    }

    /// Display string as a success. (Bold Green)
    fn print_as_success(&self)
    where
        Self: Display,
    {
        println!("\x1b[1;32m{self}\x1b[0m")
    }

    /// Display string as a warning. (Bold Yellow)
    fn print_as_warning(&self)
    where
        Self: Display,
    {
        println!("\x1b[1;33m{self}\x1b[0m")
    }

    /// Display string as an error. (Bold Red)
    fn print_as_error(&self)
    where
        Self: Display,
    {
        println!("\x1b[1;31m{self}\x1b[0m")
    }
}

impl Printer for String {}
impl Printer for &str {}

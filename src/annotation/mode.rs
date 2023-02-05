use owo_colors::OwoColorize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Warning,
    Error,
    Info,
}

impl Mode {
    pub fn display<T: Display>(self, item: T) -> impl Display {
        ModeDisplay(self, item)
    }

    pub fn underline(self) -> char {
        match self {
            Mode::Warning | Mode::Error => '^',
            Mode::Info => '-',
        }
    }
}

struct ModeDisplay<T: Display>(Mode, T);

impl<T: Display> Display for ModeDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Mode::Warning => write!(f, "{}", self.1.bright_yellow()),
            Mode::Error => write!(f, "{}", self.1.bright_red()),
            Mode::Info => write!(f, "{}", self.1.bright_cyan()),
        }
    }
}

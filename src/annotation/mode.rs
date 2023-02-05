use std::fmt::{Display, Formatter};
use yansi::Paint;

/// Controls the theme/styling of an [`Annotation`].
///
/// [`Annotation`]: crate::annotation::Annotation
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    /// An information annotation. Prints highlights with hyphens and cyan text when color is
    /// enabled.
    ///
    /// # Example
    ///
    /// ```text
    /// 1 | hello world, I am here.
    ///   |       ----- note: all of the world
    /// ```
    Info,

    /// A warning annotation. Prints highlights with carets and yellow text when color is enabled.
    ///
    /// # Example
    ///
    /// ```text
    /// 1 | bigSum = 1 + 2 + (3 + 4)
    ///   |                  ^^^^^^^ brackets are not needed
    /// ```
    Warning,

    /// An error annotation. Prints highlights with carets and red text when color is enabled.
    ///
    /// # Example
    ///
    /// ```text
    /// 1 | if (if()) {
    ///   |     ^^ expected an expression
    /// ```
    Error,
}

impl Mode {
    /// Returns an object that implements [`Display`] to display an object in the mode's color.
    pub fn display<T: Display>(self, item: T) -> impl Display {
        ModeDisplay(self, item)
    }

    /// Returns the mode's underline character.
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
            Mode::Warning => write!(f, "{}", Paint::yellow(&self.1).bold()),
            Mode::Error => write!(f, "{}", Paint::red(&self.1).bold()),
            Mode::Info => write!(f, "{}", Paint::cyan(&self.1).bold()),
        }
    }
}

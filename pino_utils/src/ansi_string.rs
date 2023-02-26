//! Utilities for constructing ANSI strings

use std::{default, fmt::Display};

/// The 16 ANSI colors
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Purple = 35,
    Cyan = 36,
    White = 37,

    StrongBlack = 90,
    StrongRed = 91,
    StrongGreen = 92,
    StrongYellow = 93,
    StrongBlue = 94,
    StrongPurple = 95,
    StrongCyan = 96,
    StrongWhite = 97,
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

/// ANSI effects
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Normal,
    Bold,
    Dim,
    Underline,
    Blinking,
    Reversed,
}

impl Effect {
    fn value(&self) -> &str {
        match self {
            Effect::Normal => "0;",
            Effect::Bold => "1;",
            Effect::Dim => "2;",
            Effect::Underline => "4;",
            Effect::Blinking => "5;",
            Effect::Reversed => "7;",
        }
    }
}

impl Default for Effect {
    fn default() -> Self {
        Effect::Normal
    }
}

/// Construct a new ANSI string from a color and an effect
/// Deprecated
#[deprecated(
    since = "0.1.1",
    note = "Use new AnsiString, as it is much more ergonomic"
)]
pub fn ansi_string(s: &str, color: Color, effect: Effect) -> String {
    let color_val = color as isize;
    let effect_val = effect.value();
    format!("\x1b[{}{}m{}\x1b[0m", effect_val, color_val, s)
}

trait ToAnsiString {
    fn black(self) -> AnsiString;
    fn red(self) -> AnsiString;
    fn green(self) -> AnsiString;
    fn yellow(self) -> AnsiString;
    fn blue(self) -> AnsiString;
    fn purple(self) -> AnsiString;
    fn cyan(self) -> AnsiString;
    fn white(self) -> AnsiString;

    fn strong_black(self) -> AnsiString;
    fn strong_red(self) -> AnsiString;
    fn strong_green(self) -> AnsiString;
    fn strong_yellow(self) -> AnsiString;
    fn strong_blue(self) -> AnsiString;
    fn strong_purple(self) -> AnsiString;
    fn strong_cyan(self) -> AnsiString;
    fn strong_white(self) -> AnsiString;

    fn normal(self) -> AnsiString;
    fn bold(self) -> AnsiString;
    fn dim(self) -> AnsiString;
    fn underline(self) -> AnsiString;
    fn blinking(self) -> AnsiString;
    fn reversed(self) -> AnsiString;
}

#[derive(Default)]
pub struct AnsiString {
    pub value: String,
    pub color: Color,
    pub effect: Effect,
}

impl ToAnsiString for AnsiString {
    fn black(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::Black,
            effect: self.effect,
        }
    }
    fn red(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::Red,
            effect: self.effect,
        }
    }
    fn green(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::Green,
            effect: self.effect,
        }
    }
    fn yellow(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::Yellow,
            effect: self.effect,
        }
    }
    fn blue(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::Blue,
            effect: self.effect,
        }
    }
    fn purple(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::Purple,
            effect: self.effect,
        }
    }
    fn cyan(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::Cyan,
            effect: self.effect,
        }
    }
    fn white(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::White,
            effect: self.effect,
        }
    }
    fn strong_black(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::StrongBlack,
            effect: self.effect,
        }
    }
    fn strong_red(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::StrongRed,
            effect: self.effect,
        }
    }
    fn strong_green(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::StrongGreen,
            effect: self.effect,
        }
    }
    fn strong_yellow(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::StrongYellow,
            effect: self.effect,
        }
    }
    fn strong_blue(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::StrongBlue,
            effect: self.effect,
        }
    }
    fn strong_purple(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::StrongPurple,
            effect: self.effect,
        }
    }
    fn strong_cyan(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::StrongCyan,
            effect: self.effect,
        }
    }
    fn strong_white(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::StrongWhite,
            effect: self.effect,
        }
    }
    fn normal(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: Color::White,
            effect: Effect::Normal,
        }
    }
    fn bold(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: self.color,
            effect: Effect::Bold,
        }
    }
    fn dim(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: self.color,
            effect: Effect::Dim,
        }
    }
    fn underline(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: self.color,
            effect: Effect::Underline,
        }
    }
    fn blinking(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: self.color,
            effect: Effect::Blinking,
        }
    }
    fn reversed(self) -> AnsiString {
        AnsiString {
            value: self.value,
            color: self.color,
            effect: Effect::Reversed,
        }
    }
}

impl Display for AnsiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.color == Color::White && self.effect == Effect::Normal {
            write!(f, "{}", self.value)
        } else {
            let color_val = self.color as isize;
            let effect_val = self.effect.value();
            write!(f, "\x1b[{}{}m{}\x1b[0m", effect_val, color_val, self.value)
        }
    }
}
impl ToAnsiString for &str {
    fn black(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::Black,
            effect: Effect::Normal,
        }
    }
    fn red(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::Red,
            effect: Effect::Normal,
        }
    }
    fn green(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::Green,
            effect: Effect::Normal,
        }
    }
    fn yellow(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::Yellow,
            effect: Effect::Normal,
        }
    }
    fn blue(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::Blue,
            effect: Effect::Normal,
        }
    }
    fn purple(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::Purple,
            effect: Effect::Normal,
        }
    }
    fn cyan(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::Cyan,
            effect: Effect::Normal,
        }
    }
    fn white(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::White,
            effect: Effect::Normal,
        }
    }
    fn strong_black(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::StrongBlack,
            effect: Effect::Normal,
        }
    }
    fn strong_red(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::StrongRed,
            effect: Effect::Normal,
        }
    }
    fn strong_green(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::StrongGreen,
            effect: Effect::Normal,
        }
    }
    fn strong_yellow(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::StrongYellow,
            effect: Effect::Normal,
        }
    }
    fn strong_blue(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::StrongBlue,
            effect: Effect::Normal,
        }
    }
    fn strong_purple(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::StrongPurple,
            effect: Effect::Normal,
        }
    }
    fn strong_cyan(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::StrongCyan,
            effect: Effect::Normal,
        }
    }
    fn strong_white(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::StrongWhite,
            effect: Effect::Normal,
        }
    }
    fn normal(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::White,
            effect: Effect::Normal,
        }
    }
    fn bold(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::White,
            effect: Effect::Bold,
        }
    }
    fn dim(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::White,
            effect: Effect::Dim,
        }
    }
    fn underline(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::White,
            effect: Effect::Underline,
        }
    }
    fn blinking(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::White,
            effect: Effect::Blinking,
        }
    }
    fn reversed(self) -> AnsiString {
        AnsiString {
            value: self.into(),
            color: Color::White,
            effect: Effect::Reversed,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::ToAnsiString;

    #[test]
    fn basic() {
        println!("{}", "hello world".bold().red())
    }
}

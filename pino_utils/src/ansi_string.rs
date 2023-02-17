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
pub fn ansi_string(s: &str, color: Color, effect: Effect) -> String {
    let color_val = color as isize;
    let effect_val = effect.value();
    format!("\x1b[{}{}m{}\x1b[0m", effect_val, color_val, s)
}

#[derive(Default)]
pub struct AnsiString {
    pub value: String,
    pub color: Color,
    pub effect: Effect,
}

impl AnsiString {}

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

trait ToAnsiString {
    fn normal(self) -> AnsiString;
    fn bold(self) -> AnsiString;
    fn dim(self) -> AnsiString;
    fn underline(self) -> AnsiString;
    fn blinking(self) -> AnsiString;
    fn reversed(self) -> AnsiString;
}

impl ToAnsiString for String {
    fn normal(self) -> AnsiString {
        AnsiString {
            value: self,
            color: Color::White,
            effect: Effect::Normal,
        }
    }
    fn bold(self) -> AnsiString {
        AnsiString {
            value: self,
            color: Color::White,
            effect: Effect::Bold,
        }
    }
    fn dim(self) -> AnsiString {
        AnsiString {
            value: self,
            color: Color::White,
            effect: Effect::Dim,
        }
    }
    fn underline(self) -> AnsiString {
        AnsiString {
            value: self,
            color: Color::White,
            effect: Effect::Underline,
        }
    }
    fn blinking(self) -> AnsiString {
        AnsiString {
            value: self,
            color: Color::White,
            effect: Effect::Blinking,
        }
    }
    fn reversed(self) -> AnsiString {
        AnsiString {
            value: self,
            color: Color::White,
            effect: Effect::Reversed,
        }
    }
}

#[cfg(test)]
mod tests {}

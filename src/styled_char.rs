use std::fmt::{self, Display};

use crossterm::{
    queue,
    style::{Color, PrintStyledContent, StyledContent},
};

use crate::compound_style::CompoundStyle;
use crate::errors::Result;

/// A modifiable character which can be easily written or repeated. Can
/// be used for bullets, horizontal rules or quote marks.
pub struct StyledChar {
    compound_style: CompoundStyle,
    nude_char: char,
    styled_char: StyledContent<char>, // redundant, kept for performance
}

impl StyledChar {
    pub fn new(compound_style: CompoundStyle, nude_char: char) -> StyledChar {
        Self {
            nude_char,
            styled_char: compound_style.apply_to(nude_char),
            compound_style,
        }
    }
    pub fn from_fg_char(fg: Color, nude_char: char) -> StyledChar {
        Self::new(CompoundStyle::with_fg(fg), nude_char)
    }
    /// Change the char, keeping colors and attributes
    pub fn set_char(&mut self, nude_char: char) {
        self.nude_char = nude_char;
        self.styled_char = self.compound_style.apply_to(self.nude_char);
    }
    /// Change the fg color, keeping the char, bg color and attributes
    pub fn set_fg(&mut self, color: Color) {
        self.compound_style.set_fg(color);
        self.styled_char = self.compound_style.apply_to(self.nude_char);
    }
    /// Change the style (colors, attributes) of the styled char
    pub fn set_compound_style(&mut self, compound_style: CompoundStyle) {
        self.compound_style = compound_style;
        self.styled_char = self.compound_style.apply_to(self.nude_char);
    }
    /// Return a struct implementing `Display`, made of a (optimized) repetition
    ///  of the character with its style.
    pub fn repeated(&self, count: usize) -> StyledContent<String> {
        let mut s = String::new();
        for _ in 0..count {
            s.push(self.nude_char);
        }
        self.compound_style.apply_to(s)
    }
    pub fn queue_repeat<W>(&self, w: &mut W, count: usize) -> Result<()>
    where
        W: std::io::Write,
    {
        let mut s = String::new();
        for _ in 0..count {
            s.push(self.nude_char);
        }
        self.compound_style.queue(w, s)
    }
    pub fn queue<W>(&self, w: &mut W) -> Result<()>
    where
        W: std::io::Write,
    {
        Ok(queue!(w, PrintStyledContent(self.styled_char.clone()))?)
    }
}

impl Display for StyledChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.styled_char.fmt(f)
    }
}

use std::fmt::{self, Display};

use crossterm::{
    queue,
    style::{
        Attribute, Color, ContentStyle, PrintStyledContent, SetBackgroundColor, SetForegroundColor,
        StyledContent,
    },
};

use crate::{errors::Result, styled_char::StyledChar};

/// A style which may be applied to a compound
#[derive(Default, Clone)]
pub struct CompoundStyle {
    pub object_style: ContentStyle, // a crossterm content style
}

impl From<ContentStyle> for CompoundStyle {
    fn from(object_style: ContentStyle) -> CompoundStyle {
        CompoundStyle { object_style }
    }
}

impl CompoundStyle {
    /// Apply an `StyledContent` to the passed displayable object.
    pub fn apply_to<D: Display>(&self, val: D) -> StyledContent<D>
    where
        D: Clone,
    {
        self.object_style.apply(val)
    }

    /// Get an new instance of `CompoundStyle`
    pub fn new(
        foreground_color: Option<Color>,
        background_color: Option<Color>,
        attributes: Vec<Attribute>,
    ) -> CompoundStyle {
        CompoundStyle {
            object_style: ContentStyle {
                foreground_color,
                background_color,
                attributes,
            },
        }
    }

    /// Get an new instance of `CompoundStyle`
    pub fn with_fgbg(fg: Color, bg: Color) -> CompoundStyle {
        CompoundStyle {
            object_style: ContentStyle::new().foreground(fg).background(bg),
        }
    }

    /// Get an new instance of `CompoundStyle`
    pub fn with_fg(fg: Color) -> CompoundStyle {
        CompoundStyle {
            object_style: ContentStyle::new().foreground(fg),
        }
    }

    /// Get an new instance of `CompoundStyle`
    pub fn with_bg(bg: Color) -> CompoundStyle {
        CompoundStyle {
            object_style: ContentStyle::new().background(bg),
        }
    }

    /// Get an new instance of `CompoundStyle`
    pub fn with_attr(attr: Attribute) -> CompoundStyle {
        let mut cp = CompoundStyle::default();
        cp.add_attr(attr);
        cp
    }

    /// Set the foreground color to the passed color.
    pub fn set_fg(&mut self, color: Color) {
        self.object_style.foreground_color = Some(color);
    }

    /// Set the background color to the passed color.
    pub fn set_bg(&mut self, color: Color) {
        self.object_style.background_color = Some(color);
    }

    /// Set the colors to the passed ones
    pub fn set_fgbg(&mut self, fg: Color, bg: Color) {
        self.object_style.foreground_color = Some(fg);
        self.object_style.background_color = Some(bg);
    }

    /// Add an `Attribute`. Like italic, underlined or bold.
    pub fn add_attr(&mut self, attr: Attribute) {
        self.object_style.attributes.push(attr);
    }

    /// Add the defined characteristics of `other` to self, overwriting
    ///  its own one when defined
    pub fn overwrite_with(&mut self, other: &CompoundStyle) {
        self.object_style.foreground_color = other
            .object_style
            .foreground_color
            .or(self.object_style.foreground_color);
        self.object_style.background_color = other
            .object_style
            .background_color
            .or(self.object_style.background_color);
        self.object_style
            .attributes
            .extend(&other.object_style.attributes); // TODO duplicates ?
    }

    #[inline(always)]
    pub fn get_fg(&self) -> Option<Color> {
        self.object_style.foreground_color
    }

    #[inline(always)]
    pub fn get_bg(&self) -> Option<Color> {
        self.object_style.background_color
    }

    /// Write a string several times with the line compound style
    ///
    /// Implementation Note: performances here are critical
    #[inline(always)]
    pub fn repeat_string(&self, f: &mut fmt::Formatter<'_>, s: &str, count: usize) -> fmt::Result {
        if count > 0 {
            write!(f, "{}", self.apply_to(s.repeat(count)))
        } else {
            Ok(())
        }
    }

    /// Write 0 or more spaces with the line's compound style
    #[inline(always)]
    pub fn repeat_space(&self, f: &mut fmt::Formatter<'_>, count: usize) -> fmt::Result {
        self.repeat_string(f, " ", count)
    }

    /// write the value with this style on the given
    /// writer
    pub fn queue<W, D>(&self, w: &mut W, val: D) -> Result<()>
    where
        D: Clone + Display,
        W: std::io::Write,
    {
        Ok(queue!(w, PrintStyledContent(self.apply_to(val)))?)
    }

    /// write the string with this style on the given
    /// writer
    pub fn queue_str<W>(&self, w: &mut W, s: &str) -> Result<()>
    where
        W: std::io::Write,
    {
        self.queue(w, s.to_string())
    }

    pub fn queue_fg<W>(&self, w: &mut W) -> Result<()>
    where
        W: std::io::Write,
    {
        if let Some(fg) = self.object_style.foreground_color {
            queue!(w, SetForegroundColor(fg))?;
        }
        Ok(())
    }

    pub fn queue_bg<W>(&self, w: &mut W) -> Result<()>
    where
        W: std::io::Write,
    {
        if let Some(bg) = self.object_style.background_color {
            queue!(w, SetBackgroundColor(bg))?;
        }
        Ok(())
    }

    pub fn style_char(&self, nude_char: char) -> StyledChar {
        StyledChar::new(self.clone(), nude_char)
    }
}

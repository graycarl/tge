use super::Color;
use crate::math::{Position, Point, Scale, Angle};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum TextHorizontalAlign {
    Start,
    Center,
    End,
}

impl Default for TextHorizontalAlign {

    fn default() -> Self {
        TextHorizontalAlign::Start
    }

}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum TextVerticalAlign {
    Top,
    Middle,
    Bottom,
}

impl Default for TextVerticalAlign {

    fn default() -> Self {
        TextVerticalAlign::Top
    }

}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TextDrawParams {
    pub(crate) text_size: Option<f32>,
    pub(crate) horizontal_align: Option<TextHorizontalAlign>,
    pub(crate) vertical_align: Option<TextVerticalAlign>,
    pub(crate) wrap_width: Option<f32>,
    pub(crate) wrap_height: Option<f32>,
    pub(crate) char_spacing: Option<f32>,
    pub(crate) line_height: Option<f32>,
    pub(crate) line_spacing: Option<f32>,
    pub(crate) origin: Option<Point>,
    pub(crate) position: Option<Position>,
    pub(crate) rotation: Option<Angle>,
    pub(crate) scale: Option<Scale>,
    pub(crate) color: Option<Color>,
}

impl TextDrawParams {

    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = Some(size);
        self
    }

    pub fn horizontal_align(mut self, align: TextHorizontalAlign) -> Self {
        self.horizontal_align = Some(align);
        self
    }

    pub fn vertical_align(mut self, align: TextVerticalAlign) -> Self {
        self.vertical_align = Some(align);
        self
    }

    pub fn wrap_width(mut self, width: f32) -> Self {
        self.wrap_width = Some(width);
        self
    }

    pub fn wrap_height(mut self, height: f32) -> Self {
        self.wrap_height = Some(height);
        self
    }

    pub fn char_spacing(mut self, spacing: f32) -> Self {
        self.char_spacing = Some(spacing);
        self
    }

    pub fn line_height(mut self, height: f32) -> Self {
        self.line_height = Some(height);
        self
    }

    pub fn line_spacing(mut self, spacing: f32) -> Self {
        self.line_spacing = Some(spacing);
        self
    }

    pub fn origin(mut self, origin: impl Into<Point>) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn position(mut self, position: impl Into<Position>) -> Self {
        self.position = Some(position.into());
        self
    }

    pub fn rotation(mut self, angle: Angle) -> Self {
        self.rotation = Some(angle);
        self
    }

    pub fn scale(mut self, scale: impl Into<Scale>) -> Self {
        self.scale = Some(scale.into());
        self
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }

}

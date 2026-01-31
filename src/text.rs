extern crate fontdue;

use crate::*;

use fontdue::Font;

pub struct TextRenderSettings {
    pub font: Font,
    pub size: u16,
    pub blend: bool,
}

pub trait TextOperations: GraphicsRenderer {
    fn text(&mut self, col: Self::Color, x: u16, y: u16, txt: String, opts: TextRenderSettings);
}

impl<T: GraphicsRenderer> TextOperations for T {
    fn text(&mut self, col: Self::Color, x: u16, y: u16, txt: String, opts: TextRenderSettings) {}
}

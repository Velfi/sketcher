use super::constants::{BLACK, WHITE};
use image::Rgba;

#[derive(Debug, Clone, Copy)]
pub enum Background {
    /// The sketcher will draw on a white background
    White,
    /// The sketcher will draw on a black background
    Black,
}

impl From<Background> for Rgba<u8> {
    fn from(background: Background) -> Self {
        match background {
            Background::White => WHITE.into(),
            Background::Black => BLACK.into(),
        }
    }
}

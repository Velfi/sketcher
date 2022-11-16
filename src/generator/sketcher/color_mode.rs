use crate::generator::sketcher::constants::WHITE;

use super::constants::{BLACK, COLOR_ALPHA};
use image::Rgba;

#[derive(Debug, Clone, Copy)]
pub struct ColorMode {
    inner: Inner,
}

impl ColorMode {
    pub fn black() -> Self {
        Self {
            inner: Inner::Black,
        }
    }

    pub fn white() -> Self {
        Self {
            inner: Inner::White,
        }
    }

    pub fn cmyk() -> Self {
        Self {
            inner: Inner::Cmyk(CmykState::Cyan),
        }
    }

    pub fn rgb() -> Self {
        Self {
            inner: Inner::Rgb(RgbState::Red),
        }
    }

    pub fn toggle(&mut self) {
        use Inner::*;

        match self.inner {
            Cmyk(state) => {
                self.inner = match state {
                    CmykState::Cyan => Cmyk(CmykState::Magenta),
                    CmykState::Magenta => Cmyk(CmykState::Yellow),
                    CmykState::Yellow => Cmyk(CmykState::Black),
                    CmykState::Black => Cmyk(CmykState::Cyan),
                }
            }
            Rgb(state) => {
                self.inner = match state {
                    RgbState::Red => Rgb(RgbState::Green),
                    RgbState::Green => Rgb(RgbState::Blue),
                    RgbState::Blue => Rgb(RgbState::Red),
                }
            }
            _ => (),
        }
    }
}

impl From<ColorMode> for Rgba<u8> {
    fn from(color_mode: ColorMode) -> Self {
        use Inner::*;

        match color_mode.inner {
            Black => BLACK.into(),
            White => WHITE.into(),
            Cmyk(state) => state.into(),
            Rgb(state) => state.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Inner {
    /// The sketcher will draw in black
    Black,
    /// The sketcher will draw in white
    White,
    /// The sketcher will draw in CMYK color
    Cmyk(CmykState),
    /// The sketcher will draw in RGB color
    Rgb(RgbState),
}

impl From<Inner> for Rgba<u8> {
    fn from(color_mode: Inner) -> Self {
        match color_mode {
            Inner::Black => Rgba(BLACK),
            Inner::White => Rgba(WHITE),
            Inner::Cmyk(state) => state.into(),
            Inner::Rgb(state) => state.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CmykState {
    Cyan,
    Magenta,
    Yellow,
    Black,
}

impl From<CmykState> for Rgba<u8> {
    fn from(state: CmykState) -> Self {
        match state {
            CmykState::Cyan => Rgba([0, 255, 255, COLOR_ALPHA]),
            CmykState::Magenta => Rgba([255, 0, 255, COLOR_ALPHA]),
            CmykState::Yellow => Rgba([255, 255, 0, COLOR_ALPHA]),
            CmykState::Black => Rgba([0, 0, 0, COLOR_ALPHA]),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RgbState {
    Red,
    Green,
    Blue,
}

impl From<RgbState> for Rgba<u8> {
    fn from(state: RgbState) -> Self {
        match state {
            RgbState::Red => Rgba([255, 0, 0, COLOR_ALPHA]),
            RgbState::Green => Rgba([0, 255, 0, COLOR_ALPHA]),
            RgbState::Blue => Rgba([0, 0, 255, COLOR_ALPHA]),
        }
    }
}

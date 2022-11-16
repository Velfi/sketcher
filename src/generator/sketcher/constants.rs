use std::ops::RangeInclusive;

pub const BLACK: [u8; 4] = [0, 0, 0, 255];
pub const WHITE: [u8; 4] = [255, 255, 255, 255];
pub const HATCH_LENGTH: f32 = 12.0;
pub const RADIUS_RANGE: RangeInclusive<i32> = 4..=8;
pub const COLOR_ALPHA: u8 = 128;

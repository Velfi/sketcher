use super::Sketcher;
use image::RgbaImage;
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_line_segment},
    rect::Rect,
};
use rand::{thread_rng, Rng};

const BLACK: [u8; 4] = [0, 0, 0, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];
const HATCH_LENGTH: f32 = 12.0;

#[derive(Debug, Clone, Copy)]
enum Hatch {
    Right,
    Left,
}

impl Hatch {
    pub fn toggle(&mut self) {
        *self = match self {
            Hatch::Right => Hatch::Left,
            Hatch::Left => Hatch::Right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Style {
    CrossHatch,
    Lines,
}

pub struct LineSketcher<I> {
    previous_image: Option<I>,
    current_image: Option<I>,
    next_image: Option<I>,
    hatch: Hatch,
    style: Style,
}

impl LineSketcher<RgbaImage> {
    pub fn new(width: u32, height: u32, style: Style) -> Self {
        let mut current_image = RgbaImage::new(width, height);
        let dimensions = Rect::at(0, 0).of_size(width, height);
        draw_filled_rect_mut(&mut current_image, dimensions, WHITE.into());

        Self {
            previous_image: None,
            current_image: Some(current_image),
            next_image: None,
            hatch: Hatch::Right,
            style,
        }
    }
}

impl Sketcher for LineSketcher<RgbaImage> {
    type Image = RgbaImage;

    fn undo(&mut self) {
        self.current_image = self.previous_image.take();
    }

    fn next(&mut self) {
        self.next_image = match self.style {
            Style::Lines => draw_random_line(self.current_image.as_ref()),
            Style::CrossHatch => {
                self.hatch.toggle();
                draw_random_hatch(self.current_image.as_ref(), self.hatch)
            }
        };

        self.previous_image = self.current_image.take();
        self.current_image = self.next_image.take();
    }

    fn image(&self) -> &Self::Image {
        self.current_image.as_ref().expect("image should exist")
    }
}

fn random_xy(width: u32, height: u32) -> (f32, f32) {
    let x: u32 = thread_rng().gen_range(0..=width);
    let y: u32 = thread_rng().gen_range(0..=height);
    (x as f32, y as f32)
}

fn draw_random_line(image: Option<&RgbaImage>) -> Option<RgbaImage> {
    image.map(|image| {
        let random_start = random_xy(image.width(), image.height());
        let random_end = random_xy(image.width(), image.height());
        draw_line_segment(image, random_start, random_end, BLACK.into())
    })
}

fn draw_random_hatch(image: Option<&RgbaImage>, hatch: Hatch) -> Option<RgbaImage> {
    image.map(|image| {
        let start = random_xy(image.width(), image.height());
        let hatch_length = HATCH_LENGTH * thread_rng().gen_range(0.2..=1.2);
        let end = match hatch {
            Hatch::Right => (start.0 as f32 + hatch_length, start.1 as f32 + hatch_length),
            Hatch::Left => (start.0 as f32 - hatch_length, start.1 as f32 + hatch_length),
        };
        draw_line_segment(image, start, end, BLACK.into())
    })
}

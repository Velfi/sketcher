use super::{
    background::Background,
    color_mode::ColorMode,
    constants::{HATCH_LENGTH, RADIUS_RANGE},
    Sketcher,
};
use image::RgbaImage;
use imageproc::{
    drawing::{
        draw_filled_circle, draw_filled_rect_mut, draw_hollow_circle, draw_line_segment,
        draw_polygon,
    },
    rect::Rect,
};
use rand::{thread_rng, Rng};

use imageproc::point::Point;

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
    CrossHatch(Hatch),
    Lines,
    Circles,
    Dots,
    Triangles,
}

impl Style {
    pub fn cross_hatch() -> Self {
        Style::CrossHatch(Hatch::Right)
    }

    pub fn lines() -> Self {
        Style::Lines
    }

    pub fn circles() -> Self {
        Style::Circles
    }

    pub fn dots() -> Self {
        Style::Dots
    }

    pub fn triangles() -> Self {
        Style::Triangles
    }
}

pub struct ImageProcSketcher<I> {
    previous_image: Option<I>,
    current_image: Option<I>,
    next_image: Option<I>,
    options: Options,
}

pub struct Options {
    pub color_mode: ColorMode,
    pub background: Background,
    pub style: Style,
}

/// A sketcher implementation that uses the `imageproc` crate to draw.
impl ImageProcSketcher<RgbaImage> {
    pub fn new(width: u32, height: u32, options: Options) -> Self {
        let mut current_image = RgbaImage::new(width, height);
        let dimensions = Rect::at(0, 0).of_size(width, height);
        draw_filled_rect_mut(&mut current_image, dimensions, options.background.into());

        Self {
            previous_image: None,
            current_image: Some(current_image),
            next_image: None,
            options,
        }
    }
}

impl Sketcher for ImageProcSketcher<RgbaImage> {
    type Image = RgbaImage;

    fn undo(&mut self) {
        self.current_image = self.previous_image.take();
    }

    fn next(&mut self) {
        self.next_image = match &mut self.options.style {
            Style::Lines => draw_random_line(self.current_image.as_ref(), self.options.color_mode),
            Style::CrossHatch(hatch) => {
                hatch.toggle();
                draw_random_hatch(self.current_image.as_ref(), *hatch, self.options.color_mode)
            }
            Style::Circles => {
                draw_random_hollow_circle(self.current_image.as_ref(), self.options.color_mode)
            }
            Style::Dots => draw_random_dot(self.current_image.as_ref(), self.options.color_mode),
            Style::Triangles => {
                draw_random_triangle_2(self.current_image.as_ref(), self.options.color_mode)
            }
        };

        // rotate the current color if we're in RGB or CMYK mode
        self.options.color_mode.toggle();

        self.previous_image = self.current_image.take();
        self.current_image = self.next_image.take();
    }

    fn image(&self) -> &Self::Image {
        self.current_image.as_ref().expect("image should exist")
    }
}

fn random_xy_f32(width: u32, height: u32) -> (f32, f32) {
    let x: u32 = thread_rng().gen_range(0..=width);
    let y: u32 = thread_rng().gen_range(0..=height);
    (x as f32, y as f32)
}

fn random_xy_i32(width: u32, height: u32) -> (i32, i32) {
    let x: u32 = thread_rng().gen_range(0..=width);
    let y: u32 = thread_rng().gen_range(0..=height);
    (x as i32, y as i32)
}

fn draw_random_line(image: Option<&RgbaImage>, color_mode: ColorMode) -> Option<RgbaImage> {
    image.map(|image| {
        let start = random_xy_f32(image.width(), image.height());
        let end = random_xy_f32(image.width(), image.height());
        draw_line_segment(image, start, end, color_mode.into())
    })
}

fn draw_random_hatch(
    image: Option<&RgbaImage>,
    hatch: Hatch,
    color_mode: ColorMode,
) -> Option<RgbaImage> {
    image.map(|image| {
        let start = random_xy_f32(image.width(), image.height());
        let hatch_length = HATCH_LENGTH * thread_rng().gen_range(0.2..=1.2);
        let end = match hatch {
            Hatch::Right => (start.0 + hatch_length, start.1 + hatch_length),
            Hatch::Left => (start.0 - hatch_length, start.1 + hatch_length),
        };
        draw_line_segment(image, start, end, color_mode.into())
    })
}

fn draw_random_hollow_circle(
    image: Option<&RgbaImage>,
    color_mode: ColorMode,
) -> Option<RgbaImage> {
    image.map(|image| {
        let start = random_xy_i32(image.width(), image.height());
        let radius = thread_rng().gen_range(RADIUS_RANGE);
        draw_hollow_circle(image, start, radius, color_mode.into())
    })
}

fn draw_random_dot(image: Option<&RgbaImage>, color_mode: ColorMode) -> Option<RgbaImage> {
    image.map(|image| {
        let start = random_xy_i32(image.width(), image.height());
        draw_filled_circle(image, start, 3, color_mode.into())
    })
}

fn draw_random_triangle(image: Option<&RgbaImage>, color_mode: ColorMode) -> Option<RgbaImage> {
    image.map(|image| {
        let (start_x, start_y) = random_xy_i32(image.width(), image.height());
        let points = &[
            Point::new(start_x, start_y + 10),
            Point::new(start_x + 10, start_y - 10),
            Point::new(start_x - 10, start_y - 10),
        ];
        draw_polygon(image, points, color_mode.into())
    })
}

fn draw_random_triangle_2(image: Option<&RgbaImage>, color_mode: ColorMode) -> Option<RgbaImage> {
    image.map(|image| {
        let (start_x, start_y) = random_xy_i32(image.width(), image.height());
        let points = &[
            Point::new(start_x, start_y - 10),
            Point::new(start_x + 10, start_y + (10.0 - 3.16227766) as i32),
            Point::new(start_x - 10, start_y + (10.0 - 3.16227766) as i32),
        ];
        draw_polygon(image, points, color_mode.into())
    })
}

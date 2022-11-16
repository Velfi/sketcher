pub mod background;
pub mod color_mode;
pub mod constants;
pub mod image_proc_sketcher;

pub use background::Background;
pub use color_mode::ColorMode;

pub trait Sketcher {
    type Image;

    fn undo(&mut self);
    fn next(&mut self);
    fn image(&self) -> &Self::Image;
}

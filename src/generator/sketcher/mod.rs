pub mod line;

pub trait Sketcher {
    type Image;

    fn undo(&mut self);
    fn next(&mut self);
    fn image(&self) -> Self::Image;
}
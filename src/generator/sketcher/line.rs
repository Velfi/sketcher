use image::DynamicImage;
use super::Sketcher;

pub struct LineSketcher {
    previous_image: Option<DynamicImage>,
    current_image: DynamicImage,
    next_image: Option<DynamicImage>,
}

impl LineSketcher {
    pub fn new(width: u32, height: u32) -> Self {
        let current_image = DynamicImage::new_rgb8(width, height);

        Self { previous_image: None, current_image, next_image: None }
    }
}

impl Sketcher for LineSketcher {
    type Image = DynamicImage;

    fn undo(&mut self) {
        if let Some(previous_image) = &self.previous_image {
            self.current_image = previous_image.clone();
        }
    }

    fn next(&mut self) {
        self.previous_image = Some(self.current_image.clone());

        todo!()
    }

    fn image(&self) -> Self::Image {
        // TODO How can this be updated to return a ref instead of cloning?
        self.current_image.clone()
    }
}
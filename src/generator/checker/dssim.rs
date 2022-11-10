use crate::conversion::ConversionWrapper;

use super::Checker;
use image::RgbaImage;
use rgb::RGBA8;

type Image = RgbaImage;

pub struct DssimChecker {
    ground_truth: Image,
    dssim: dssim_core::Dssim,
}

impl DssimChecker {
    pub fn new(ground_truth: Image) -> Self {
        Self {
            ground_truth,
            dssim: dssim_core::Dssim::new(),
        }
    }
}

impl Checker for DssimChecker {
    type Similarity = f64;
    type Original = Image;
    type Other<'a> = &'a Self::Original;

    fn similarity(&self, other: Self::Other<'_>) -> Self::Similarity {
        // TODO converting this every time is so inefficient, fix that pls
        let bitmap_a: Vec<RGBA8> = ConversionWrapper::new(&self.ground_truth).into();
        let bitmap_b: Vec<RGBA8> = ConversionWrapper::new(other).into();
        let image_a = self
            .dssim
            .create_image_rgba(
                &bitmap_a,
                self.ground_truth.width() as usize,
                self.ground_truth.height() as usize,
            )
            .expect("dssim image can be created from rgba8 slice");
        let image_b = self
            .dssim
            .create_image_rgba(&bitmap_b, other.width() as usize, other.height() as usize)
            .expect("dssim image can be created from rgba8 slice");

        let (similarity, _) = self.dssim.compare(&image_a, image_b);
        similarity.into()
    }
}

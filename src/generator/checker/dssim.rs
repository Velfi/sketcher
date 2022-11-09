use super::Checker;
use image::DynamicImage;

pub struct DssimChecker {}

impl DssimChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl Checker for DssimChecker {
    type Similarity = dssim_core::Val;
    type Image = DynamicImage;

    fn check(
        image_a: Self::Image,
        image_b: Self::Image,
        previous_similarity: Self::Similarity,
    ) -> std::cmp::Ordering {
        todo!()
    }
}

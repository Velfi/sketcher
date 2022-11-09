pub mod dssim;

use std::cmp::Ordering;

pub trait Checker {
    type Similarity;
    type Image;

    fn check(image_a: Self::Image, image_b: Self::Image, previous_similarity: Self::Similarity) -> Ordering;
}

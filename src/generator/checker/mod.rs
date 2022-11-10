pub mod dssim;

pub trait Checker {
    type Similarity;
    type Original;
    type Other<'a>;

    fn similarity(&self, other: Self::Other<'_>) -> Self::Similarity;
}

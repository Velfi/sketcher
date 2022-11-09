use image::RgbImage;
use rgb::RGB8;

pub struct ConversionWrapper<T> {
  inner: T
}

impl<T> ConversionWrapper<T> {
  pub fn new(inner: T) -> Self {
    ConversionWrapper { inner }
  }
}

impl From<ConversionWrapper<&RgbImage>> for Vec<RGB8> {
    fn from(wrapper: ConversionWrapper<&RgbImage>) -> Self {
      wrapper.inner.pixels().map(|p| rgb::RGB8::from(p.0)).collect()
    }
}
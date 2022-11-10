use image::RgbaImage;
use rgb::RGBA8;

pub struct ConversionWrapper<T> {
  inner: T
}

impl<T> ConversionWrapper<T> {
  pub fn new(inner: T) -> Self {
    ConversionWrapper { inner }
  }
}

impl From<ConversionWrapper<&RgbaImage>> for Vec<RGBA8> {
    fn from(wrapper: ConversionWrapper<&RgbaImage>) -> Self {
      wrapper.inner.pixels().map(|p| rgb::RGBA8::from(p.0)).collect()
    }
}
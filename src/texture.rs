use std::path::Path;

use image::RgbImage;
use luminance::pixel::NormRGB8UI;
use luminance::texture::{Dim2, GenMipmaps, Sampler, Texture};
use luminance_glfw::GlfwSurface;

pub fn read_image(path: &Path) -> Option<RgbImage> {
    image::open(path).map(|img| img.flipv().to_rgb()).ok()
}

pub fn load_image(surface: &mut GlfwSurface, img: RgbImage) -> Texture<Dim2, NormRGB8UI> {
    let (width, height) = img.dimensions();
    let texels = img.into_raw();

    let tex = Texture::new(surface, [width, height], 0, Sampler::default()).unwrap();
    tex.upload_raw(GenMipmaps::No, &texels).unwrap();

    tex
}

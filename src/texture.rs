use std::path::Path;

use image::RgbImage;
use luminance::pixel::NormRGB8UI;
use luminance::texture::{Dim2, GenMipmaps, Sampler, Texture};
use luminance_glfw::GlfwSurface;

fn read_image(path: &Path) -> Option<RgbImage> {
    image::open(path).map(|img| img.flipv().to_rgb()).ok()
}

fn load_from_disk(surface: &mut GlfwSurface, img: RgbImage) -> Texture<Dim2, NormRGB8UI> {
    let (width, height) = img.dimensions();
    let texels = img.into_raw();

    let tex = Texture::new(surface, [width, height], 0, Sampler::default()).unwrap();
    tex.upload_raw(GenMipmaps::No, &texels).unwrap();

    tex
}

pub fn load_image(surface: &mut GlfwSurface,
                  texture_path: Option<&Path>)
                  -> Option<Texture<Dim2, NormRGB8UI>> {
    if let Some(path) = texture_path {
        if path.is_file() {
            if let Some(image) = read_image(path) {
                return Some(load_from_disk(surface, image));
            }
        } else {
            eprintln!("The texture image path is not a valid file. \
                   Please provide a path to a valid file.");
        }
    }

    None
}

use std::path::Path;

use image::RgbImage;

pub fn read_image(path: &Path) -> Option<RgbImage> {
    image::open(path).map(|img| img.flipv().to_rgb()).ok()
}

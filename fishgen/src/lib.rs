#[cfg(feature = "druid")]
use druid::Data;

use image::{
    imageops::{overlay, resize, FilterType},
    DynamicImage, ImageBuffer, Rgba,
};
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};
use lazy_static::lazy_static;

lazy_static! {
    static ref FISH: DynamicImage = image::load_from_memory(include_bytes!("fish.png")).unwrap();
}

const FISH_WIDTH: u32 = 289;
const FISH_HEIGHT: u32 = 269;
const MIN_SCALE: f32 = 0.1;

#[cfg_attr(feature = "druid", derive(Data, Clone))]
pub struct FishOutput {
    #[cfg_attr(feature = "druid", data(ignore))]
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn random_fish(width: u32, height: u32) -> FishOutput {
    let scale = fastrand::f32() * (1.0 - MIN_SCALE) + MIN_SCALE;

    let fish_width = (FISH_WIDTH as f32 * scale) as u32;
    let fish_height = (FISH_HEIGHT as f32 * scale) as u32;

    let fish = rotate_about_center(
        &(FISH.clone().to_rgba8()),
        fastrand::f32() * std::f32::consts::TAU,
        Interpolation::Bilinear,
        Rgba::<u8>([255, 255, 255, 0]),
    );

    let fish = resize(&fish, fish_width, fish_height, FilterType::Triangle);

    let mut image: ImageBuffer<Rgba<u8>, _> =
        image::ImageBuffer::from_pixel(width, height, Rgba::<u8>([255, 255, 255, 255]));

    let fish_x = fastrand::u32(0..(width - fish_width));
    let fish_y = fastrand::u32(0..(width - fish_width));

    overlay(&mut image, &fish, fish_x.into(), fish_y.into());

    FishOutput {
        image,
        x: fish_x,
        y: fish_y,
        width: fish_width,
        height: fish_height,
    }
}

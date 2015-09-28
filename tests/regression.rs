//! Compares results of image processing functions to existing "truth" images.
//! All test images are taken from the caltech256 dataset
//! http://authors.library.caltech.edu/7694/

#![feature(core)]
#![feature(test)]
#![feature(unboxed_closures)]

extern crate image;
extern crate test;
#[macro_use]
extern crate imageproc;

use std::path::Path;
use imageproc::utils::load_image_or_panic;
use image::Rgb;

fn compare_to_truth_rgb(
    input_path: &Path,
    truth_path: &Path,
    op: &Fn(&image::RgbImage) -> image::RgbImage) {

    let truth = load_image_or_panic(&truth_path).to_rgb();
    let input = load_image_or_panic(&input_path).to_rgb();
    let actual = op.call((&input,));

    assert_pixels_eq!(&actual, &truth);
}

fn rotate_nearest_about_center(image: &image::RgbImage) -> image::RgbImage {
    imageproc::affine::rotate_nearest(
        image,
        (image.width() as f32/2f32, image.height() as f32/2f32),
        std::f32::consts::PI/4f32,
        Rgb([0u8;3]))
}

#[test]
fn test_rotate_nearest_rgb() {
    let ip = Path::new("./tests/data/elephant.png");
    let tp = Path::new("./tests/data/truth/elephant_rotate_nearest.png");
    compare_to_truth_rgb(&ip, &tp, &rotate_nearest_about_center);
}
use std::{env, path::Path};

use image::{open, GenericImage, ImageResult, Rgb};
use imageproc::{
    binary_descriptors::{match_binary_descriptors, orb::orb, BinaryDescriptor},
    definitions::Image,
    drawing::draw_line_segment_mut,
};

fn main() -> ImageResult<()> {
    if env::args().len() != 4 {
        panic!("Please enter two input files and one output file")
    }

    let first_image_path_arg = env::args().nth(1).unwrap();
    let second_image_path_arg = env::args().nth(2).unwrap();
    let output_image_path_arg = env::args().nth(3).unwrap();

    let first_image_path = Path::new(&first_image_path_arg);
    let second_image_path = Path::new(&second_image_path_arg);

    if !first_image_path.is_file() {
        panic!("First image file does not exist");
    }
    if !second_image_path.is_file() {
        panic!("Second image file does not exist");
    }

    let first_image = open(first_image_path)?.to_luma8();
    let second_image = open(second_image_path)?.to_luma8();

    let start = std::time::Instant::now();
    let first_descriptors = orb(&first_image, 1000, 5, 1.414, None);
    let second_descriptors = orb(&second_image, 1000, 5, 1.414, None);
    let elapsed = start.elapsed();
    println!(
        "Computed {} descriptors in {:?} ({:?} per descriptor)",
        first_descriptors.len() + second_descriptors.len(),
        elapsed,
        elapsed / (first_descriptors.len() + second_descriptors.len()) as u32
    );

    // for _ in 0..100 {
    //     let (_first_descriptor_points, _first_descriptors) =
    //         orb(&first_image, 1000, 5, 1.414, None);
    // }

    let start = std::time::Instant::now();
    let matches = match_binary_descriptors(&first_descriptors, &second_descriptors, 64, None);
    let elapsed = start.elapsed();
    println!(
        "Matched {} descriptor pairs in {:?}",
        matches.len(),
        elapsed
    );

    // now that we've matched descriptors in both images, put them side by side
    // and draw lines connecting the descriptors together
    let first_image = open(first_image_path)?.to_rgb8();
    let second_image = open(second_image_path)?.to_rgb8();

    let (first_image_height, first_image_width) = (first_image.height(), first_image.width());
    let (second_image_height, second_image_width) = (second_image.height(), second_image.width());

    let (output_width, output_height) = (
        first_image_width + second_image_width,
        u32::max(first_image_height, second_image_height),
    );
    let mut output_image = Image::new(output_width, output_height);
    output_image.copy_from(&first_image, 0, 0).unwrap();
    output_image
        .copy_from(&second_image, first_image.width(), 0)
        .unwrap();
    for keypoint_match in matches.iter() {
        let start_point = keypoint_match.0.position();
        let end_point = keypoint_match.1.position();
        draw_line_segment_mut(
            &mut output_image,
            (start_point.x as f32, start_point.y as f32),
            (
                (end_point.x + first_image.width()) as f32,
                end_point.y as f32,
            ),
            Rgb([0, 255, 0]),
        )
    }
    output_image.save(&output_image_path_arg).unwrap();
    println!("Wrote output image to {}", output_image_path_arg);

    Ok(())
}
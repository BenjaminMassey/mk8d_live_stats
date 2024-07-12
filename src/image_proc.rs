use image::GenericImageView;

use crate::data;

use std::str::FromStr;

const IMAGE_THRESHOLD: f64 = 0.15;
const FIRST_ITEM_SPOT: ((f64, f64), (f64, f64)) = 
    ((0.09619, 0.10112), (0.07441, 0.13804));
const SECOND_ITEM_SPOT: ((f64, f64), (f64, f64)) =
    ((0.04719, 0.06581), (0.03902, 0.06421));
const PLACEMENT_SPOT: ((f64, f64), (f64, f64)) =
    ((0.83385, 0.77685), (0.13281, 0.19259));

pub fn get_items(full_shot: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> data::Items {
    let image_one = image::SubImage::new(
        full_shot,
        (FIRST_ITEM_SPOT.0.0 * full_shot.width() as f64) as u32,
        (FIRST_ITEM_SPOT.0.1 * full_shot.height() as f64) as u32,
        (FIRST_ITEM_SPOT.1.0 * full_shot.width() as f64) as u32,
        (FIRST_ITEM_SPOT.1.1 * full_shot.height() as f64) as u32,
    ).to_image();
    let image_two = image::SubImage::new(
        full_shot,
        (SECOND_ITEM_SPOT.0.0 * full_shot.width() as f64) as u32,
        (SECOND_ITEM_SPOT.0.1 * full_shot.height() as f64) as u32,
        (SECOND_ITEM_SPOT.1.0 * full_shot.width() as f64) as u32,
        (SECOND_ITEM_SPOT.1.1 * full_shot.height() as f64) as u32,
    ).to_image();
    let item_choices = std::fs::read_dir("./icons/items/raw")
        .unwrap()
        .chain(std::fs::read_dir("./icons/items/screenshots").unwrap())
        .map(|f| f.unwrap().path().to_owned())
        .collect::<Vec<std::path::PathBuf>>();
    let mut items = data::Items{ first: None, second: None };
    for (i, item_spot) in vec![image_one, image_two].iter().enumerate() {
        let mut max = -1f64;
        let mut result: Option<data::Item> = None;
        for item_choice in &item_choices {
            let item_image = image::open(item_choice)
                .expect("Could not load item icon")
                .resize_exact(
                    item_spot.width(),
                    item_spot.height(),
                    image::imageops::FilterType::Nearest
                )
                .into_rgba8();
            let similarity = image_compare::rgba_hybrid_compare(item_spot, &item_image)
                .expect("Images compare failed")
                .score;
            if similarity > max {
                max = similarity;
                let file_name = item_choice
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned();
                if similarity > IMAGE_THRESHOLD {
                    if file_name.contains("Nothing") {
                        result = None;
                    } else {
                        println!("{file_name}");
                        result = Some(
                            data::Item::from_str(
                                &file_name[..(file_name.len() - 4)]
                            ).unwrap()
                        );
                    }
                }
            }
        }
        if i == 0 {
            items.first = result;
        } else {
            items.second = result;
        }
    }
    items
}

pub fn get_placement(full_shot: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> u8 {
    let placement_area = image::SubImage::new(
        full_shot,
        (PLACEMENT_SPOT.0.0 * full_shot.width() as f64) as u32,
        (PLACEMENT_SPOT.0.1 * full_shot.height() as f64) as u32,
        (PLACEMENT_SPOT.1.0 * full_shot.width() as f64) as u32,
        (PLACEMENT_SPOT.1.1 * full_shot.height() as f64) as u32,
    ).to_image();
    let background_color = placement_area.get_pixel(0, 0).0;
    let placements = std::fs::read_dir("./icons/placements")
        .unwrap()
        .map(|f| f.unwrap().path().to_owned())
        .collect::<Vec<std::path::PathBuf>>();
    let mut max = -1f64;
    let mut result = 0u8;
    for placement in &placements {
        let original_placement_image = image::open(placement)
            .expect("Could not load item icon")
            .resize_exact(
                placement_area.width(),
                placement_area.height(),
                image::imageops::FilterType::Nearest,
            );
        let placement_image = image::ImageBuffer::from_fn(
            original_placement_image.width(),
            original_placement_image.height(),
            |x, y| {
                let pixel = original_placement_image.get_pixel(x, y);
                if pixel.0[3] == 0 {
                    image::Rgba(background_color)
                } else {
                    image::Rgba(pixel.0)
                }
            }
        );
        let similarity = image_compare::rgba_hybrid_compare(
                &placement_area,
                &placement_image,
            )
            .expect("Images compare failed")
            .score;
        if similarity > max {
            max = similarity;
            let file_name = placement
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            result = file_name[..(file_name.len() - 4)]
                .to_owned()
                .parse::<u8>()
                .unwrap();
        }
    }
    result
}
use crate::data;

use std::str::FromStr;

pub fn get_items(full_shot: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> data::Items {
    let image_one = image::SubImage::new(
        full_shot, 106, 63, 82, 86,
    ).to_image(); // TODO: dynamically calculated pos/size
    let image_two = image::SubImage::new(
        full_shot, 52, 41, 43, 40,
    ).to_image(); // TODO: dynamically calculated pos/size
    let item_choices = std::fs::read_dir("./icons/items/")
        .unwrap()
        .map(|f| f.unwrap().path().to_owned())
        .collect::<Vec<std::path::PathBuf>>();
    let mut items: Vec<String> = vec![];
    for item_spot in &vec![image_one, image_two] {
        let mut max = -1f64;
        let mut result = String::new();
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
                result = file_name[..(file_name.len() - 4)].to_owned();
            }
        }
        items.push(result);
    }
    data::Items { 
        first: Some(data::Item::from_str(&items[0]).unwrap()),
        second: Some(data::Item::from_str(&items[1]).unwrap()),
    }
}
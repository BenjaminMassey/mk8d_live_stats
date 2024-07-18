use enigo::Mouse;

pub fn get_screen_point() -> (i32, i32) {
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");
    let e = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    e.location().unwrap()
}

pub fn get_full_shot(
    screen: screenshots::Screen,
    top_left: (i32, i32),
    bottom_right: (i32, i32)
) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let broken = screen.capture_area(
        top_left.0,
        top_left.1,
        (bottom_right.0 - top_left.0) as u32,
        (bottom_right.1 - top_left.1) as u32,
    )
    .unwrap();
    image::ImageBuffer::from_vec(
        broken.width(),
        broken.height(),
        broken.to_vec())
        .unwrap()
}
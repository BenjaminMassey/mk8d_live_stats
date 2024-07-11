mod data;
mod image_proc;

fn main() {
    let full_shot = image::open("./test.png")
        .expect("Could not open full shot image")
        .into_rgba8();
    let player = data::Player {
        items: image_proc::get_items(&full_shot)
    };
    println!("{:?} and {:?}",
        player.items.first,
        player.items.second,
    )
}
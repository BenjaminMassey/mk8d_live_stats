mod data;
mod image_proc;

fn main() {
    let full_shot = image::open("./test-1.png")
        .expect("Could not open full shot image")
        .into_rgba8();
    let player = data::Player {
        items: image_proc::get_items(&full_shot),
        placement: image_proc::get_placement(&full_shot),
    };
    println!("Item {:?} & Item {:?} & Placement {}",
        player.items.first,
        player.items.second,
        player.placement,
    );
}
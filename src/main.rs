mod data;
mod image_proc;
mod screen;

fn main() {
    println!("top left and enter");
    let top_left = screen::get_screen_point();
    println!("bottom right and enter");
    let bottom_right = screen::get_screen_point();
    let screen = screenshots::Screen::from_point(0, 0).unwrap();
    loop {
        let full_shot = screen::get_full_shot(screen, top_left, bottom_right);
        let player = data::Player {
            items: image_proc::get_items(&full_shot),
            placement: image_proc::get_placement(&full_shot),
        };
        println!("Item {:?} & Item {:?} & Placement {}",
            player.items.first,
            player.items.second,
            player.placement,
        );
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
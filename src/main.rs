use tcod::console::Root;
use tcod::{FontLayout, FontType, input};

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(20, 20)
        .title("Modifier key repro")
        .init();

    tcod::system::set_fps(10);

    while !root.window_closed() {
        if let Some((_flags, event)) = input::check_for_event(input::KEY) {
            println!("{:?}", event);
        }
    }
}

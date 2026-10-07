use ori_native::prelude::*;

mod swipeable;

use swipeable::*;

#[ori_native::main]
pub fn main() {
    let mut data = Data {};

    App::new().run(&mut data, ui).unwrap();
}

struct Data {}

fn ui(_data: &Data) -> impl Effect<Data> + use<> {
    window(
        row(swipeable(text("swipeable").margin(20.0))
            .left_view(
                row(text("left"))
                    .justify_content(Justify::Center)
                    .align_items(Align::Center)
                    .flex(1.0),
            )
            .right_view(row(button(image(include_bytes!("icon/trash.svg")), |_| {})
                .align_items(Align::Center)
                .justify_content(Justify::Center)
                .border_width(0.0)
                .aspect_ratio(1.0)))
            .left_full(true)
            .justify_content(Justify::Center)
            .flex(1.0))
        .justify_content(Justify::Center)
        .align_items(Align::Center)
        .background(Color::WHITE.darken(0.1))
        .flex(1.0),
    )
}

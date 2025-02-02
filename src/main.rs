use std::error::Error;

mod date_picker;

use chrono::Local;
use date_picker::date_picker;
use ori::prelude::*;

#[ori::main]
fn main() -> Result<(), Box<dyn Error>> {
    ori::log::install()?;

    let window = Window::new().title("jio");

    let app = App::build()
        .window(window, ui)
        .style(include_style!("style.oss"));

    let mut data = Data {};

    ori::run(app, &mut data)?;

    Ok(())
}

pub struct Data {}

fn ui(_data: &mut Data) -> impl View<Data> {
    let now = Local::now();

    center(date_picker(now.date_naive(), |_, _, _| {}))
}

use std::{collections::HashMap, error::Error};

mod date_picker;

use chrono::{DateTime, Duration, Utc};
use date_picker::date_picker;
use ori::prelude::*;
use uuid::Uuid;

#[ori::main]
fn main() -> Result<(), Box<dyn Error>> {
    ori::log::install()?;

    let window = Window::new().title("jio");

    let app = App::build().window(window, ui);

    let mut data = Data {
        editing: None,
        events: HashMap::new(),
    };

    ori::run(app, &mut data)?;

    Ok(())
}

pub struct Data {
    pub editing: Option<Uuid>,
    pub events: HashMap<Uuid, Event>,
}

pub struct Event {
    pub title: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

fn ui(data: &mut Data) -> impl View<Data> {
    let view = center(add_event_button());

    match data.editing {
        Some(id) => {
            let edit = focus(event_editor(), move |data: &mut Data, lens| {
                lens(data.events.get_mut(&id).unwrap());
            });

            any(zstack![view, center(edit)])
        }
        None => any(view),
    }
}

fn event_editor() -> impl View<Event> {
    build(|_, event: &mut Event| {
        let title = text_input()
            .text(&event.title)
            .on_input(|cx, event: &mut Event, title| {
                event.title = title;
                cx.rebuild();
            });

        let start_date = event.start.date_naive();
        let start_date = date_picker(start_date, |cx, event: &mut Event, date| {
            event.start = date.and_time(event.start.time()).and_utc();
            cx.rebuild();
        });
        let start_date = popup(text!("{:?}", event.start.date_naive()), start_date);

        let end_date = event.end.date_naive();
        let end_date = date_picker(end_date, |cx, event: &mut Event, date| {
            event.end = date.and_time(event.end.time()).and_utc();
            cx.rebuild();
        });
        let end_date = popup(text!("{:?}", event.end.date_naive()), end_date);

        let view = vstack![title, start_date, end_date];

        container(view)
    })
}

fn add_event_button() -> impl View<Data> {
    let btn = button(text("Add Event"));

    on_click(btn, |cx, data: &mut Data| {
        let id = Uuid::new_v4();

        let event = Event {
            title: String::from("New Event"),
            start: Utc::now(),
            end: Utc::now() + Duration::hours(1),
        };

        data.events.insert(id, event);
        data.editing = Some(id);
        cx.rebuild();
    })
}

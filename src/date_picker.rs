use std::rc::Rc;

use chrono::{Datelike, Days, Month, NaiveDate};
use ori::prelude::*;

use crate::Data;

struct DatePickerState {
    year: i32,
    month: u32,
}

pub fn date_picker(
    from: NaiveDate,
    on_select: impl Fn(&mut EventCx, &mut Data, NaiveDate) + 'static,
) -> impl View<Data> {
    let on_select = Rc::new(on_select);
    let state = DatePickerState {
        year: from.year(),
        month: from.month(),
    };

    with_state(
        || state,
        move |state, _| {
            let next_month = button(text!(">"));
            let next_month = on_click(
                next_month,
                move |cx, (state, _): &mut (DatePickerState, _)| {
                    state.month += 1;

                    if state.month > 12 {
                        state.month = 1;
                        state.year += 1;
                    }

                    cx.rebuild();
                },
            );

            let prev_month = button(text!("<"));
            let prev_month = on_click(
                prev_month,
                move |cx, (state, _): &mut (DatePickerState, _)| {
                    state.month -= 1;

                    if state.month < 1 {
                        state.month = 12;
                        state.year -= 1;
                    }

                    cx.rebuild();
                },
            );

            let month = Month::try_from(state.month as u8).unwrap();
            let title = text!("{} {}", month.name(), state.year);

            let header = hstack![prev_month, title, next_month];
            let header = container(header);
            let header = class("header", header);

            let mut days = hwrap_any().gap(2.0);

            for name in ['m', 't', 'w', 't', 'f', 's', 's'].iter() {
                let name = text!("{}", name);
                let name = size(24.0, center(name));
                let name = class("day-name", name);

                days.push(any(name));
            }

            let mut date = NaiveDate::from_ymd_opt(state.year, state.month, 1).unwrap();
            date = date - Days::new(date.weekday() as u64);

            for _ in 0..42 {
                let mut class_name = "day";

                if date.month() != state.month {
                    class_name = "day-other-month";
                }

                let day = text!("{}", date.day());
                let day = button(center(day));
                let day = on_click(day, {
                    let on_select = on_select.clone();
                    move |cx, (_, data)| on_select(cx, data, date)
                });
                let day = size(24.0, day);
                let day = class(class_name, day);

                if date == from {
                    days.push(any(class("day-selected", day)));
                } else {
                    days.push(any(day));
                }

                date = date + Days::new(1);
            }

            let days = max_width(26.0 * 7.0, days);
            let days = height(27.0 * 7.0 - 5.0, days);

            let view = vstack![header, days];
            let view = container(view);

            class("date-picker", view)
        },
    )
}

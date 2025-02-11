use std::rc::Rc;

use chrono::{Datelike, Days, Month, NaiveDate};
use ori::prelude::*;

struct DatePickerState {
    year: i32,
    month: u32,
}

pub fn date_picker<T: 'static>(
    from: NaiveDate,
    on_select: impl Fn(&mut EventCx, &mut T, NaiveDate) + 'static,
) -> impl View<T> {
    let on_select = Rc::new(on_select);

    with_state(
        move || DatePickerState {
            year: from.year(),
            month: from.month(),
        },
        move |_, _| {
            let on_select = on_select.clone();
            build(move |cx, (state, _): &mut (DatePickerState, _)| {
                let next_month = switch_month_button(text!(">"));
                let next_month = on_click(next_month, self::next_month);

                let prev_month = switch_month_button(text!("<"));
                let prev_month = on_click(prev_month, self::prev_month);

                let month = Month::try_from(state.month as u8).unwrap();
                let title = text!("{} {}", month.name(), state.year);

                let header = hstack![prev_month, title, next_month].justify(Justify::SpaceBetween);
                let header = container(header).background(cx.theme().surface(1));

                let mut days = hwrap_any().gap(2.0);

                for name in ['m', 't', 'w', 't', 'f', 's', 's'].iter() {
                    let name = text!("{}", name)
                        .font_size(12.0)
                        .font_weight(FontWeight::BOLD)
                        .color(cx.theme().accent);
                    let name = size(24.0, center(name));

                    days.push(any(name));
                }

                let mut date = NaiveDate::from_ymd_opt(state.year, state.month, 1).unwrap();
                date = date - Days::new(date.weekday() as u64);

                for _ in 0..42 {
                    let mut day = text!("{}", date.day()).font_size(12.0);

                    if date.month() != state.month {
                        day = day.color(cx.theme().contrast_low());
                    }

                    let mut day = button(center(day)).padding(0.0).color(cx.theme().surface);

                    if date == from {
                        day = day
                            .border_width([0.0, 0.0, 2.0, 0.0])
                            .border_color(cx.theme().primary)
                            .border_radius(0.0);
                    }

                    let day = on_click(day, {
                        let on_select = on_select.clone();
                        move |cx, (_, data)| on_select(cx, data, date)
                    });

                    let day = size(24.0, day);
                    days.push(any(day));

                    date = date + Days::new(1);
                }

                let days = max_width(26.0 * 7.0, days);

                let view = vstack![header, days].align(Align::Stretch).gap(8.0);
                let view = pad(6.0, view);
                container(view).border_width(1.0)
            })
        },
    )
}

fn switch_month_button<T>(content: impl View<T>) -> impl View<T> {
    build(|cx, _| {
        button(content)
            .padding([6.0, 2.0])
            .color(cx.theme().surface(1))
    })
}

fn next_month<T>(cx: &mut EventCx, (state, _): &mut (DatePickerState, T)) {
    state.month += 1;

    if state.month > 12 {
        state.month = 1;
        state.year += 1;
    }

    cx.rebuild();
}

fn prev_month<T>(cx: &mut EventCx, (state, _): &mut (DatePickerState, T)) {
    state.month -= 1;

    if state.month < 1 {
        state.month = 12;
        state.year -= 1;
    }

    cx.rebuild();
}

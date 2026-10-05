use ori_native::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

#[builder]
pub fn swipeable<T>(
    contents: impl ViewSeq<T>,

    #[default = Color::TRANSPARENT] color: Color,

    #[default = row(())] left_view: impl View<T>,
    #[default = row(())] right_view: impl View<T>,

    #[default] left_full: bool,
    #[default] right_full: bool,

    #[default = Color::RED] left_color: Color,
    #[default = Color::GREEN] right_color: Color,

    #[default = |_, _| ()] mut on_open: impl (FnMut(&mut T, Side) -> impl Into<Action>) + 'static,
    #[default = |_| ()] mut on_close: impl (FnMut(&mut T) -> impl Into<Action>) + 'static,

    #[layout] layout: LayoutStyle,
    #[corners] corners: Corners<f32>,
    #[padding] padding: Sides<Length>,
    #[border] border: BorderStyle,
    #[shadow] shadow: Shadow,
    #[flex] flex: FlexStyle,
) -> impl View<T> {
    struct State<T> {
        left_width: f32,
        right_width: f32,

        width: f32,
        offset: f32,
        start: Option<f32>,

        changed: bool,
        side: Option<Side>,
        on_open: Option<Box<dyn FnMut(&mut T, Side) -> Action>>,
        on_close: Option<Box<dyn FnMut(&mut T) -> Action>>,
    }

    let mut contents = Some(contents);
    let mut left_view = Some(left_view);
    let mut right_view = Some(right_view);

    with(
        |_| State {
            left_width: 0.0,
            right_width: 0.0,

            width: 0.0,
            offset: 0.0,
            start: None,

            changed: false,
            side: None,
            on_open: None,
            on_close: None,
        },
        move |_, _| {
            let mut contents = contents.take();
            let mut left_view = left_view.take();
            let mut right_view = right_view.take();

            on_layout(
                pressable(move |(state, _): &(State<T>, _), _| {
                    let mut contents = contents.take();
                    let mut left_view = left_view.take();
                    let mut right_view = right_view.take();

                    animate(
                        spring(state.offset, move |_, offset| {
                            row((
                                row(on_layout(
                                    without(maybe(left_view.take())),
                                    move |(state, _): &mut (State<T>, _), width, _| {
                                        state.left_width = width;
                                        Action::new()
                                    },
                                ))
                                .top(0.0)
                                .bottom(0.0)
                                .left(0.0)
                                .width(offset)
                                .justify_content(Justify::Start)
                                .background(left_color)
                                .position(Position::Absolute)
                                .overflow(Overflow::Hidden),
                                row(on_layout(
                                    without(maybe(right_view.take())),
                                    |(state, _): &mut (State<T>, _), width, _| {
                                        state.right_width = width;
                                        Action::new()
                                    },
                                ))
                                .top(0.0)
                                .bottom(0.0)
                                .right(0.0)
                                .width(-offset)
                                .justify_content(Justify::End)
                                .background(right_color)
                                .position(Position::Absolute)
                                .overflow(Overflow::Hidden),
                                without(
                                    row(maybe_seq(contents.take()))
                                        .left(offset)
                                        .width(Fract(1.0))
                                        .background(color)
                                        .set_border(border)
                                        .set_padding(padding)
                                        .set_flex(flex),
                                ),
                            ))
                            .overflow(Overflow::Hidden)
                            .set_corners(corners)
                            .set_layout(layout)
                            .set_shadow(shadow)
                        })
                        .duration_bounce(0.1, 0.0),
                    )
                    .on_end(|(state, data): &mut (State<T>, T)| {
                        if let Some(ref mut on_open) = state.on_open
                            && let Some(side) = state.side
                            && state.changed
                        {
                            state.changed = false;
                            on_open(data, side)
                        } else if let Some(ref mut on_close) = state.on_close
                            && state.side.is_none()
                            && state.changed
                        {
                            state.changed = false;
                            on_close(data)
                        } else {
                            Action::new()
                        }
                    })
                })
                .on_event(move |(state, _), event| {
                    let left_width = if left_full {
                        state.width
                    } else {
                        state.left_width
                    };

                    let right_width = if right_full {
                        state.width
                    } else {
                        state.right_width
                    };

                    match event {
                        PressableEvent::Pressed(event) => {
                            state.start = Some(event.position.x - state.offset);

                            Action::new()
                        }

                        PressableEvent::Moved(event) => {
                            if let Some(start) = state.start {
                                let offset = event.position.x - start;
                                state.offset = offset.clamp(-right_width, left_width);

                                Action::rebuild()
                            } else {
                                Action::new()
                            }
                        }

                        PressableEvent::Released(..) | PressableEvent::Cancelled(..) => {
                            state.start = None;

                            let left_threshold = left_width.min(state.width / 2.0);
                            let right_threshold = right_width.min(state.width / 2.0);

                            let side = if state.offset >= left_threshold {
                                state.offset = left_width;
                                Some(Side::Left)
                            } else if state.offset <= -right_threshold {
                                state.offset = -right_width;
                                Some(Side::Right)
                            } else {
                                state.offset = 0.0;
                                None
                            };

                            state.changed |= state.side == side;
                            state.side = side;

                            Action::rebuild()
                        }

                        _ => Action::new(),
                    }
                }),
                |(state, _): &mut (State<T>, _), width, _| {
                    state.width = width;
                    Action::new()
                },
            )
        },
    )
    .update(move |state, _| {
        state.on_open = Some(Box::new(move |data, side| on_open(data, side).into()));
        state.on_close = Some(Box::new(move |data| on_close(data).into()));
    })
}

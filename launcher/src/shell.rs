//! The launcher's frame in the quiet look (`docs/design-system.md`): a flat sidebar, text tabs, and
//! controls that are a fill and a hairline. Nothing is cut, beveled or layered over a picture: the
//! selected row is one flat tint, and every division is a 1 px rule.
use crate::plate::alpha;
use crate::theme::{self, night, Step, Theme};
use crate::{hover_fade, keyboard, track_hover};
use gpui::{div, prelude::*, px, rgb, Div, FontWeight, Hsla, SharedString, Stateful};

/// A colour token as GPUI's.
pub fn c(color: u32) -> Hsla {
    rgb(color).into()
}

/// Text in one of the look's type steps, at its own weight.
pub fn text(step: Step, color: Hsla) -> Div {
    div()
        .font_family(theme::FONT)
        .font_weight(FontWeight(step.weight as f32))
        .text_size(px(step.px))
        .line_height(px(step.line_px()))
        .text_color(color)
}

/// The sidebar's ground: flat, with a 1 px rule on its right edge.
pub fn sidebar_plate(t: &Theme) -> Div {
    div()
        .bg(alpha(t.night.side, 1.))
        .border_r_1()
        .border_color(t.hairline())
}

/// A sidebar row (an instance, a server): its picture and two lines go in as children. The
/// selected one is a flat tint; the others get one while hovered.
pub fn row(id: impl Into<SharedString>, selected_now: bool, t: &Theme) -> Stateful<Div> {
    let id = id.into();
    let d = div()
        .id(id.clone())
        .relative()
        .flex()
        .items_center()
        .gap(px(10.))
        .h(px(44.))
        .px(px(8.))
        .rounded(px(theme::RADIUS))
        .cursor_pointer();
    let d = if selected_now {
        d.bg(t.selected_fill())
    } else {
        d.child(hovered(&id, t))
    };
    keyboard(track_hover(d, id), t)
}

/// The same shape, fading in while the pointer is over `id`.
fn hovered(id: &SharedString, t: &Theme) -> gpui::AnyElement {
    let fill = t.night.card_hi;
    let layer = move || {
        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .rounded(px(theme::RADIUS))
            .bg(alpha(fill, 0.6))
    };
    hover_fade(id, layer, |d, a| d.opacity(a))
}

/// A row's two lines: the name, and its state after a small square in `state`'s colour when it
/// has one ("Ready · 26.3" in green, "Needs preparing" in amber).
pub fn row_text(name: impl Into<SharedString>, sub: String, state: Option<Hsla>, t: &Theme) -> Div {
    let n = t.night;
    div()
        .relative()
        .flex()
        .flex_col()
        .min_w_0()
        .gap(px(1.))
        .child(
            text(night::UI, c(n.ink))
                .font_weight(FontWeight::MEDIUM)
                .truncate()
                .child(name.into()),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(5.))
                .min_w_0()
                .children(state.map(|s| div().flex_none().size(px(5.)).rounded_full().bg(s)))
                .child(text(night::CAPTION, c(n.muted)).truncate().child(sub)),
        )
}

/// A heading over a group of rows ("Instances", a group's name, "Servers"), with room for a
/// button on its right.
pub fn heading(label: impl Into<SharedString>, t: &Theme) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .h(px(28.))
        .px(px(8.))
        .mt(px(12.))
        .child(
            text(night::CAPTION, c(t.night.muted))
                .font_weight(FontWeight::MEDIUM)
                .child(label.into()),
        )
}

/// A page's tab: text only, with a 1 px underline in the ink colour while it is the page shown.
pub fn tab(id: impl Into<SharedString>, label: &str, active: bool, t: &Theme) -> Stateful<Div> {
    let id = id.into();
    let n = t.night;
    let d = div()
        .id(id.clone())
        .relative()
        .flex()
        .items_center()
        .h(px(theme::TOPBAR))
        .px(px(12.))
        .cursor_pointer()
        .child(hovered(&id, t));
    keyboard(
        track_hover(
            d.child(
                text(night::UI, c(if active { n.ink } else { n.muted }))
                    .font_weight(if active {
                        FontWeight::MEDIUM
                    } else {
                        FontWeight::NORMAL
                    })
                    .relative()
                    .child(label.to_string()),
            )
            .when(active, |d| {
                d.child(
                    div()
                        .absolute()
                        .bottom_0()
                        .left(px(12.))
                        .right(px(12.))
                        .h(px(1.))
                        .bg(c(n.ink)),
                )
            }),
            id,
        ),
        t,
    )
}

/// A square button with one icon (Settings, the sidebar's `+`).
pub fn icon_button(
    id: impl Into<SharedString>,
    icon: &'static str,
    size: f32,
    active: bool,
    t: &Theme,
) -> Stateful<Div> {
    let id = id.into();
    let n = t.night;
    let d = div()
        .id(id.clone())
        .relative()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(px(size))
        .rounded(px(theme::RADIUS))
        .cursor_pointer();
    let d = if active {
        d.bg(t.selected_fill())
    } else {
        d.child(hovered(&id, t))
    };
    keyboard(
        track_hover(
            d.child(div().relative().child(crate::blocks::icon(
                icon,
                (size * 0.46).round(),
                c(if active { n.ink } else { n.muted }),
            ))),
            id,
        ),
        t,
    )
}

/// The account chip's plate: a fill and a hairline, for the picture and two lines.
pub fn chip(id: impl Into<SharedString>, t: &Theme) -> Stateful<Div> {
    let id = id.into();
    let d = div()
        .id(id.clone())
        .relative()
        .flex()
        .min_w_0()
        .items_center()
        .gap(px(8.))
        .h(px(32.))
        .pl(px(4.))
        .pr(px(10.))
        .rounded(px(theme::RADIUS))
        .border_1()
        .border_color(t.hairline())
        .cursor_pointer()
        .child(hovered(&id, t));
    keyboard(track_hover(d, id), t)
}

/// What a quiet button is: the one main action (an accent fill), a raised one (a fill and a
/// hairline), or only its label until the pointer is over it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Main,
    Raised,
    Quiet,
}

/// A quiet button, its label in the UI step: [`Action::Main`] is [`night::PLAY`] tall, the others
/// [`theme::CONTROL`]. Icons and the label go in as children.
pub fn action(id: impl Into<SharedString>, kind: Action, t: &Theme) -> Stateful<Div> {
    let id = id.into();
    let n = t.night;
    let d = div()
        .id(id.clone())
        .relative()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .gap(px(8.))
        .px(px(14.))
        .rounded(px(theme::RADIUS))
        .font_family(theme::FONT)
        .font_weight(FontWeight::MEDIUM)
        .text_size(px(night::UI.px))
        .cursor_pointer();
    let fill = |d: Div| {
        d.absolute()
            .top_0()
            .left_0()
            .size_full()
            .rounded(px(theme::RADIUS))
    };
    let d = match kind {
        Action::Main => d
            .h(px(night::PLAY))
            .px(px(20.))
            .text_size(px(night::TITLE.px))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(c(n.on_accent))
            .bg(alpha(n.accent, 1.))
            .child(hover_fade(
                &id,
                move || fill(div()).bg(c(n.on_accent).opacity(0.10)),
                |d, a| d.opacity(a),
            )),
        Action::Raised => d
            .h(px(theme::CONTROL))
            .bg(alpha(n.card_hi, 1.))
            .border_1()
            .border_color(alpha(n.stroke, n.stroke_alpha * 1.4))
            .text_color(c(n.ink))
            .child(hover_fade(
                &id,
                move || fill(div()).bg(c(n.ink).opacity(0.05)),
                |d, a| d.opacity(a),
            )),
        Action::Quiet => d
            .h(px(theme::CONTROL))
            .text_color(c(n.ink_soft))
            .child(hover_fade(
                &id,
                move || fill(div()).bg(alpha(n.card_hi, 0.7)),
                |d, a| d.opacity(a),
            )),
    };
    keyboard(track_hover(d, id), t).active(|s| s.opacity(theme::PRESSED))
}

/// A label for an [`action`], over its hover layer.
pub fn label(text_: impl Into<SharedString>) -> Div {
    div().relative().child(text_.into())
}

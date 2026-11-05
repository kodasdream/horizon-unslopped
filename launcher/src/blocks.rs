//! The quiet look's smaller parts (`docs/design-system.md`): icons, text in the type scale,
//! flat panels, badges, the tab strip, and the slots that hold icons downloaded at run time.
//! The old blocky look's outlines, bevels and pixel type are gone.
use crate::theme::{self, Theme};
use gpui::{
    div, img, prelude::*, px, svg, AnyElement, App, ClickEvent, Div, FontWeight, Hsla, ImageSource,
    ObjectFit, SharedString, Svg, Window,
};
use std::borrow::Cow;

/// The icons, served to `svg()` as `icons/<name>.svg`.
pub struct Assets;

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        let name = path
            .strip_prefix("icons/")
            .and_then(|p| p.strip_suffix(".svg"));
        Ok(name.and_then(horizon_theme::icons::get).map(Cow::Borrowed))
    }

    fn list(&self, _path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(horizon_theme::icons::ALL
            .iter()
            .map(|(name, _)| SharedString::from(format!("icons/{name}.svg")))
            .collect())
    }
}

/// A pixel icon (`horizon_theme::icons`) in `color`.
pub fn icon(name: &'static str, size: f32, color: Hsla) -> Svg {
    svg()
        .path(format!("icons/{name}.svg"))
        .flex_none()
        .size(px(size))
        .text_color(color)
}

/// Text at a size and weight in the UI family. Kept under its old name for the call sites; the
/// pixel face it used to draw is gone.
pub fn pixel(size: f32, weight: FontWeight) -> Div {
    div()
        .font_family(theme::FONT)
        .font_weight(weight)
        .text_size(px(size))
        .line_height(px((size * 1.35).round()))
}

/// The old bevel and lip: the quiet look has neither. Kept as an empty slot so a call site that
/// spread it still compiles and draws nothing.
pub fn edges(_top: Hsla, _bottom: Hsla, _lip: f32) -> [AnyElement; 2] {
    [div().into_any_element(), div().into_any_element()]
}

/// A panel: rows, cards, notices, dialogs. A flat fill, a hairline and one radius.
pub fn panel(t: &Theme) -> Div {
    div()
        .relative()
        .rounded(px(theme::RADIUS_PLATE))
        .bg(t.panel)
        .border_1()
        .border_color(t.hairline())
}

/// A small label in a tint of its colour: "Runs", "Partly", a version. A tint rather than an
/// outlined badge, so a list of them stays quiet.
pub fn badge(label: impl Into<SharedString>, color: Hsla, t: &Theme) -> Div {
    let label: SharedString = label.into();
    let label = SharedString::from(crate::i18n::tr(&label).to_string());
    let tint = if t.dark { 0.14 } else { 0.10 };
    div()
        .flex()
        .flex_none()
        .items_center()
        .gap(px(5.))
        .h(px(20.))
        .px(px(7.))
        .rounded(px(4.))
        .bg(color.opacity(tint))
        .child(pixel(11., FontWeight::MEDIUM).text_color(color).child(label))
}

/// A row of tabs over a panel: text only, the open one in ink with a rule under it.
pub fn tabs<T: Copy + PartialEq + 'static>(
    id: &'static str,
    options: &[(T, &'static str, &'static str)],
    current: T,
    t: &Theme,
    on_pick: impl Fn(T, &mut Window, &mut App) + Clone + 'static,
) -> Div {
    let t = *t;
    div()
        .flex()
        .items_center()
        .gap(px(2.))
        .border_b_1()
        .border_color(t.divider())
        .children(options.iter().map(|&(value, label, _icon_name)| {
            let open = current == value;
            let pick = on_pick.clone();
            let tab = div()
                .id(SharedString::from(format!("{id}-{label}")))
                .relative()
                .flex()
                .items_center()
                .h(px(36.))
                .px(px(12.))
                .cursor_pointer()
                .child(
                    pixel(13., FontWeight::MEDIUM)
                        .text_color(if open { t.ink } else { t.muted })
                        .child(crate::i18n::tr(label)),
                )
                .when(!open, |d| d.hover(move |s| s.text_color(t.ink)))
                .when(open, |d| {
                    d.child(
                        div()
                            .absolute()
                            .bottom(px(-1.))
                            .left_0()
                            .right_0()
                            .h(px(1.))
                            .bg(t.ink),
                    )
                })
                .on_click(move |_: &ClickEvent, window, cx| pick(value, window, cx));
            crate::keyboard(tab, &t)
        }))
}

/// A square slot for an icon: the picture at `source` when there is one, else the first letter of
/// `name` on a raised block.
pub fn slot(source: Option<ImageSource>, name: &str, size: f32, t: &Theme) -> Div {
    let frame = div()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(px(size))
        .rounded(px(4.))
        .overflow_hidden()
        .bg(t.raised);
    match source {
        Some(source) => frame.child(img(source).size(px(size)).object_fit(ObjectFit::Cover)),
        None => frame.child(
            pixel((size * 0.42).round(), FontWeight::MEDIUM)
                .text_color(t.muted)
                .child(
                    name.chars()
                        .find(|c| c.is_alphanumeric())
                        .unwrap_or('?')
                        .to_uppercase()
                        .to_string(),
                ),
        ),
    }
}

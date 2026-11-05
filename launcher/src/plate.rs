//! Plates and controls in the quiet look: flat fills with plain rounded corners and a hairline
//! border. The old look cut corners in pixel steps on a canvas; nothing on a screen needs a stepped
//! corner, so a plate is an ordinary box with a 1 px border and a radius.
use crate::theme::night::{self, Night};
use gpui::{div, prelude::*, px, rgba, Div, Rgba};

/// `c` at opacity `a`.
pub fn alpha(c: u32, a: f32) -> Rgba {
    rgba(c << 8 | (a * 255.).round() as u32)
}

/// A plate: the card colour, rounded, with a hairline border. Its content goes in as children.
pub fn plate(n: &Night) -> Div {
    div()
        .relative()
        .rounded(px(night::RADIUS_PLATE))
        .bg(alpha(n.card, n.card_alpha))
        .border_1()
        .border_color(alpha(n.stroke, n.stroke_alpha))
}

/// A box of any size with rounded corners: `fill` inside and a `stroke` of the given colour and
/// width along the edge. `step` and `steps` are the old look's corner geometry and are ignored;
/// the radius is [`night::RADIUS_CONTROL`].
pub fn cut(_step: f32, _steps: usize, fill: Option<Rgba>, stroke: Option<(Rgba, f32)>) -> Div {
    let mut d = div()
        .relative()
        .rounded(px(night::RADIUS_CONTROL))
        .overflow_hidden();
    if let Some(fill) = fill {
        d = d.bg(fill);
    }
    if let Some((color, width)) = stroke {
        d = d.border(px(width)).border_color(color);
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_packs_the_colour_the_way_rgba_reads_it() {
        assert_eq!(alpha(0x112233, 1.), rgba(0x112233ff));
        assert_eq!(alpha(0x000000, 0.5), rgba(0x00000080));
    }
}

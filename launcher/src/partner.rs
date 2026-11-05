//! Sculk, a server host that sponsors Horizon (Ludovic, 2026-10-26): its wide banner on the Servers
//! page, and a card made for the sidebar on the other pages, where the banner's baked-in text was too
//! small to read. The pictures are in the binary, never fetched; a click opens Sculk's page in the
//! browser. A "Partner" label above each says it is an ad. After `SEEN_BEFORE_HIDE` seconds on screen
//! a "Hide" appears on either, and hiding one hides both for good.
use crate::theme::Theme;
use gpui::{div, img, prelude::*, px, ClickEvent, Div, Image, ImageFormat, Stateful};
use std::sync::{Arc, LazyLock};

pub const URL: &str = "https://my.sculkhosting.com/ref/horizon";

/// How long the sidebar's banner stays on screen, counted over every start, before a "Hide" shows
/// beside its label: long enough to be seen, then the player may put it away for good.
pub const SEEN_BEFORE_HIDE: u64 = 180;

/// The small "Hide" beside a banner's or card's label, top right of whatever holds it (which must be
/// `relative`). The caller adds the click: it sets `Settings::partner_hidden`, which hides every
/// partner banner for good.
pub fn hide_button(id: &'static str, t: &Theme) -> Stateful<Div> {
    let t = *t;
    let hide = div()
        .id(id)
        .absolute()
        .top(px(0.))
        .right(px(0.))
        .cursor_pointer()
        .child(
            crate::text(crate::theme::MICRO)
                .text_color(t.muted)
                .hover(move |s| s.text_color(t.ink))
                .child(horizon_launcher::i18n::tr("HIDE")),
        );
    crate::keyboard(hide, &t)
}

/// 1200×441: twice the widest it is shown at, for Retina screens. Its transparent edges let the
/// characters stand out of the card.
#[cfg(test)]
const WIDTH: f32 = 1200.;
#[cfg(test)]
const HEIGHT: f32 = 441.;
/// The Servers page's sponsor: one line, the same as the sidebar's, wide. The banner picture was
/// the loudest thing in the launcher and read as an ad break in the middle of a tool, so the page
/// shows a link under a Sponsor label instead.
pub fn banner(id: &'static str, width: f32, t: &Theme) -> Stateful<Div> {
    card(id, width.min(420.), true, t)
}

/// Sculk's mark and name, cropped from its banner, for the one line the sidebar gives it.
const LOGO_WIDTH: f32 = 191.;
const LOGO_HEIGHT: f32 = 36.;
static LOGO: LazyLock<Arc<Image>> = LazyLock::new(|| {
    Arc::new(Image::from_bytes(
        ImageFormat::Webp,
        include_bytes!("../assets/sculk-logo.webp").to_vec(),
    ))
});

/// The promo code: Sculk's, never translated.
const CODE: &str = "HORIZON15";

/// The sidebar's one line for a sponsor: a label, Sculk's mark, and the offer, on the quiet
/// surface. No picture filling half the sidebar and no second colour: a sponsorship is a line, not
/// a poster, and the launcher's UI is not the place for a brand's own palette.
pub fn card(id: &'static str, width: f32, _compact: bool, t: &Theme) -> Stateful<Div> {
    let t = *t;
    let el = div()
        .id(id)
        .flex()
        .flex_col()
        .gap(px(4.))
        .w(px(width))
        .rounded(px(6.))
        .cursor_pointer()
        .child(
            crate::text(crate::theme::CAPTION)
                .text_color(t.muted)
                .child(horizon_launcher::i18n::tr("SPONSOR")),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(8.))
                .child(
                    img(LOGO.clone())
                        .flex_none()
                        .h(px(12.))
                        .w(px(12. * LOGO_WIDTH / LOGO_HEIGHT))
                        .opacity(0.9),
                )
                .child(
                    crate::text(crate::theme::CAPTION)
                        .text_color(t.ink_soft)
                        .truncate()
                        .child(horizon_launcher::i18n::tr("Minecraft hosting, 24/7")),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(6.))
                .child(
                    crate::text(crate::theme::CAPTION)
                        .font_family(crate::theme::MONO)
                        .text_color(t.muted)
                        .child(CODE),
                )
                .child(
                    crate::text(crate::theme::CAPTION)
                        .text_color(t.accent)
                        .child(horizon_launcher::i18n::tr("15% off")),
                ),
        )
        .on_click(|_: &ClickEvent, _, cx| cx.open_url(URL));
    crate::keyboard(el, &t)
}

#[cfg(test)]
mod tests {
    use super::{HEIGHT, LOGO_HEIGHT, LOGO_WIDTH, WIDTH};

    /// The bundled pictures decode as WebPs of the size the code lays them out at.
    #[test]
    fn the_card_picture_decodes() {
        let pic = image::load_from_memory_with_format(
            &include_bytes!("../assets/sculk-logo.webp")[..],
            image::ImageFormat::WebP,
        )
        .expect("a WebP");
        assert_eq!(
            (pic.width(), pic.height()),
            (LOGO_WIDTH as u32, LOGO_HEIGHT as u32)
        );
    }

    #[test]
    fn the_bundled_banner_decodes() {
        let bytes = include_bytes!("../assets/sculk-banner.webp");
        let banner = image::load_from_memory_with_format(bytes, image::ImageFormat::WebP)
            .expect("the banner is a WebP");
        assert_eq!(
            (banner.width(), banner.height()),
            (WIDTH as u32, HEIGHT as u32)
        );
    }
}

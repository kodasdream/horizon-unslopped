//! The launcher's quiet look ("Quiet" in `docs/design-system.md`): one flat surface, hairlines, and
//! text doing the work, with a single low-chroma accent for the one main action. There is no
//! gradient, no sky, and no second hue: the palette is a neutral grey ramp and the accent is a
//! desaturated slate blue, so a screen reads as a tool, not as a poster.
//!
//! The token names are the ones the launcher already used (`night`), kept so the screens that read
//! them did not all have to change at once; [`NIGHT`] is the dark appearance and [`DAWN`] the light
//! one. Type is Geist and Geist Mono, both already in [`crate::fonts`].
use crate::{step, Blocks, Step, Web};

/// The UI family. The display face is the same family: a launcher has no display type.
pub const FONT: &str = crate::FONT;
pub const DISPLAY_FONT: &str = crate::FONT;

/// Page and instance names. Deliberately smaller than the old display cut: a 56 px name over a
/// picture was the loudest part of the old look.
pub const DISPLAY: Step = step("night-display", 26., Web::Fluid(22., 30.), 1.2, -0.02, 600, false);
/// Section heads.
pub const TITLE: Step = step("night-title", 15., Web::Fixed, 1.35, -0.005, 600, false);
/// Text, rows, buttons and navigation.
pub const UI: Step = step("night-ui", 13., Web::Fixed, 1.45, 0., 400, false);
/// One-line states and details under a name ("Ready · 26.3").
pub const CAPTION: Step = step("night-caption", 11., Web::Fixed, 1.4, 0., 400, false);
/// A screen uses two or three of these, plus [`crate::MICRO`] for paths and counts.
pub const TYPE: [Step; 4] = [DISPLAY, TITLE, UI, CAPTION];

/// Corners: one radius for controls and plates, and a gentler one for large surfaces. The old look
/// cut corners in pixel steps; these are plain arcs.
pub const RADIUS_CONTROL: f32 = 6.;
pub const RADIUS_PLATE: f32 = 8.;
/// Kept for the call sites that still pass a step count: there are no stepped corners any more.
pub const PIXEL_CORNER: f32 = 6.;
/// The main action (Play) is taller than the other controls.
pub const PLAY: f32 = 44.;

/// No stop of the surface is too bright for text: every stop of the surface is safe bare.
pub const UNCARDED_STOPS: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Night {
    /// The six old sky stops. The surface is flat now, so these are six steps of one neutral ramp:
    /// the first is the page, the last the faintest raised surface.
    pub sky: [u32; 6],
    /// Behind everything, and the sidebar.
    pub page: u32,
    pub side: u32,
    /// Plates (rows, cards, dialogs), with their opacity, and a raised or selected one.
    pub card: u32,
    pub card_alpha: f32,
    pub card_hi: u32,
    pub card_hi_alpha: f32,
    /// Hairlines: a plate's border and the rules between rows, at these opacities.
    pub stroke: u32,
    pub stroke_alpha: f32,
    pub rule_alpha: f32,
    pub ink: u32,
    pub ink_soft: u32,
    pub muted: u32,
    /// The one main action's fill and progress; its label is `on_accent`.
    pub accent: u32,
    /// The accent as text and as the focus ring, readable on every plate.
    pub accent_hi: u32,
    pub on_accent: u32,
    pub success: u32,
    pub warn: u32,
    pub danger: u32,
    pub dark: bool,
}

pub const NIGHT: Night = Night {
    // One neutral ramp, page to raised: no hue shift between them.
    sky: [0x0c0c0e, 0x101013, 0x141418, 0x18181d, 0x1c1c22, 0x202027],
    page: 0x0c0c0e,
    side: 0x0f0f12,
    card: 0x131317,
    card_alpha: 1.,
    card_hi: 0x1b1b21,
    card_hi_alpha: 1.,
    stroke: 0xffffff,
    stroke_alpha: 0.07,
    rule_alpha: 0.06,
    ink: 0xedeef1,
    ink_soft: 0xb9bbc4,
    muted: 0x8f929c,
    // A slate blue, desaturated on purpose: an accent that reads as a control, not as a brand.
    accent: 0x5b7cfa,
    accent_hi: 0x9db2ff,
    on_accent: 0x0c0c0e,
    success: 0x5fbf83,
    warn: 0xd2a24a,
    danger: 0xe0736a,
    dark: true,
};

/// The light appearance: the same neutral ramp, inverted, with the accent darkened to keep its
/// contrast on white.
pub const DAWN: Night = Night {
    sky: [0xf7f7f8, 0xf4f4f6, 0xf0f0f3, 0xebebef, 0xe6e6ea, 0xe0e0e5],
    page: 0xf7f7f8,
    side: 0xf2f2f4,
    card: 0xffffff,
    card_alpha: 1.,
    card_hi: 0xf0f0f3,
    card_hi_alpha: 1.,
    stroke: 0x000000,
    stroke_alpha: 0.10,
    rule_alpha: 0.08,
    ink: 0x17181c,
    ink_soft: 0x4a4c55,
    muted: 0x5f626b,
    accent: 0x3a58c8,
    accent_hi: 0x3149a8,
    on_accent: 0xffffff,
    success: 0x1a6d3d,
    warn: 0x805400,
    danger: 0x9e3428,
    dark: false,
};

/// `top` at `alpha` over `under`, as the GPU blends them.
pub fn over(top: u32, alpha: f32, under: u32) -> u32 {
    [16, 8, 0].iter().fold(0, |out, s| {
        let (a, b) = (((top >> s) & 255) as f32, ((under >> s) & 255) as f32);
        out | ((a * alpha + b * (1. - alpha)).round() as u32) << s
    })
}

impl Night {
    /// These colours in the parts of the Blocks look the launcher still draws: plates come out
    /// opaque over the page, with hairlines instead of dark outlines and bevels.
    pub fn blocks(&self) -> Blocks {
        let shade = |a: f32, under: u32| over(if self.dark { 0 } else { self.stroke }, a, under);
        let panel = over(self.card, self.card_alpha, self.page);
        let raised = over(self.card_hi, self.card_hi_alpha, self.page);
        Blocks {
            page: self.page,
            side: self.side,
            panel,
            raised,
            button: over(self.stroke, 0.06, raised),
            button_bevel: over(self.stroke, 0.10, raised),
            well: if self.dark {
                shade(0.5, self.side)
            } else {
                over(self.card, 0.7, self.side)
            },
            bevel: over(self.stroke, self.stroke_alpha, panel),
            lip: shade(0.0, self.page),
            outline: over(self.stroke, self.stroke_alpha * 1.4, self.page),
            rule: over(self.stroke, self.rule_alpha, panel),
            ink: self.ink,
            ink_soft: self.ink_soft,
            muted: self.muted,
            accent: self.accent_hi,
            face: self.accent,
            face_bevel: self.accent,
            face_lip: self.accent,
            on_face: self.on_accent,
            danger: self.danger,
            success: self.success,
            warn: self.warn,
            dark: self.dark,
        }
    }

    /// Every colour by its CSS name (`--night-<name>`); plates carry their opacity.
    pub fn css(&self) -> String {
        let rgba = |c: u32, a: f32| {
            format!(
                "rgba({}, {}, {}, {})",
                c >> 16 & 255,
                c >> 8 & 255,
                c & 255,
                (a * 100.).round() / 100.
            )
        };
        let mut lines: Vec<(String, String)> = self
            .sky
            .iter()
            .enumerate()
            .map(|(i, c)| (format!("sky-{i}"), format!("#{c:06x}")))
            .collect();
        for (name, c) in [
            ("page", self.page),
            ("side", self.side),
            ("ink", self.ink),
            ("ink-soft", self.ink_soft),
            ("muted", self.muted),
            ("accent", self.accent),
            ("accent-hi", self.accent_hi),
            ("on-accent", self.on_accent),
            ("success", self.success),
            ("warn", self.warn),
            ("danger", self.danger),
        ] {
            lines.push((name.into(), format!("#{c:06x}")));
        }
        lines.push(("card".into(), rgba(self.card, self.card_alpha)));
        lines.push(("card-hi".into(), rgba(self.card_hi, self.card_hi_alpha)));
        lines.push(("stroke".into(), rgba(self.stroke, self.stroke_alpha)));
        lines.push(("rule".into(), rgba(self.stroke, self.rule_alpha)));
        lines
            .into_iter()
            .map(|(name, value)| format!("  --night-{name}: {value};\n"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contrast;

    /// Every text colour reaches WCAG AA (4.5:1) on both plates, the page and the sidebar, and the
    /// focus ring reaches 3:1 (WCAG 1.4.11) on everything it is drawn on.
    #[test]
    fn quiet_text_meets_wcag_aa() {
        for p in [NIGHT, DAWN] {
            let look = if p.dark { "dark" } else { "light" };
            let texts = [
                ("ink", p.ink),
                ("ink_soft", p.ink_soft),
                ("muted", p.muted),
                ("accent_hi", p.accent_hi),
                ("success", p.success),
                ("warn", p.warn),
                ("danger", p.danger),
            ];
            let grounds = [
                ("card", over(p.card, p.card_alpha, p.page)),
                ("card_hi", over(p.card_hi, p.card_hi_alpha, p.page)),
                ("page", p.page),
                ("side", p.side),
            ];
            for (gn, bg) in grounds {
                for (tn, fg) in texts {
                    let ratio = contrast(fg, bg);
                    assert!(ratio >= 4.5, "{look} {tn} on {gn}: {ratio:.2}");
                }
                let ring = contrast(p.accent, bg);
                assert!(ring >= 3.0, "{look} the focus ring on {gn}: {ring:.2}");
            }
            let label = contrast(p.on_accent, p.accent);
            assert!(label >= 4.5, "{look} the main action's label: {label:.2}");
        }
    }

    #[test]
    fn over_blends_like_the_gpu() {
        assert_eq!(over(0xffffff, 1., 0x000000), 0xffffff);
        assert_eq!(over(0xffffff, 0.5, 0x000000), 0x808080);
        assert_eq!(over(0x123456, 0., 0xabcdef), 0xabcdef);
    }
}

//! Feedback: notices (inline, in a tone), toasts (transient, stacked in a
//! corner, fading), empty states, progress bars and spinners.

use egui::{
    Align2, Area, Frame, Id, Margin, Order, Rect, Response, Role, Sense, Shape, Stroke, Ui, Vec2,
    pos2, vec2,
};

use crate::icon::{self, Icon};
use crate::state;
use crate::text::{self, Type};
use crate::theme::{self, colors};
use crate::tokens::{Elevation, Tone, motion, radius, space};

crate::messages! {
    enum Msg {
        Dismiss: "Dismiss", "Schließen";
        Loading: "Loading", "Wird geladen";
    }
}

/// The icon of a tone.
pub fn tone_icon(tone: Tone) -> Icon {
    match tone {
        Tone::Neutral | Tone::Info => Icon::Info,
        Tone::Success => Icon::Success,
        Tone::Warning => Icon::Warning,
        Tone::Danger => Icon::Error,
    }
}

/// A notice across the available width: the tone's icon and text on its
/// tint; an optional bold lead ("Found:").
pub fn notice(ui: &mut Ui, tone: Tone, lead: &str, text_: &str) -> Response {
    let c = colors(ui);
    let t = c.tone(tone);
    let r = Frame::new()
        .fill(t.tint)
        .corner_radius(radius::MENU)
        .inner_margin(Margin::symmetric(space::XL as i8, space::LG as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = space::LG;
                let (ir, _) = ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
                icon::paint(ui.painter(), ir, tone_icon(tone), t.solid);
                let mut job = egui::text::LayoutJob::default();
                if !lead.is_empty() {
                    job.append(lead, 0.0, Type::BodyStrong.format(t.text));
                    job.append(" ", 0.0, Type::Body.format(t.text));
                }
                job.append(text_, 0.0, Type::Body.format(t.text));
                job.wrap.max_width = ui.available_width();
                let g = ui.painter().layout_job(job);
                let (gr, _) = ui.allocate_exact_size(g.size(), Sense::hover());
                ui.painter().galley(gr.min, g, t.text);
            });
        });
    state::a11y(&r.response, Role::Alert, text_);
    r.response
}

/// A transient message.
#[derive(Debug, Clone)]
pub struct Toast {
    pub tone: Tone,
    pub text: String,
    /// Seconds on screen (`None`: until dismissed).
    pub duration: Option<f64>,
    shown_at: Option<f64>,
    id: u64,
}

/// The toast stack of a window (bottom right).
#[derive(Debug, Default)]
pub struct Toasts {
    items: Vec<Toast>,
    next: u64,
}

impl Toasts {
    pub fn push(&mut self, tone: Tone, text: impl Into<String>, duration: Option<f64>) {
        self.next += 1;
        self.items.push(Toast {
            tone,
            text: text.into(),
            duration,
            shown_at: None,
            id: self.next,
        });
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Shows the toasts; call once per frame.
    pub fn show(&mut self, ctx: &egui::Context) {
        let now = ctx.input(|i| i.time);
        let dark = ctx.global_style().visuals.dark_mode;
        let mut y = -space::X3L;
        let mut gone = Vec::new();
        for t in self.items.iter_mut().rev() {
            let shown = *t.shown_at.get_or_insert(now);
            let age = now - shown;
            let fade_in = ((age as f32) / motion::SLOW).clamp(0.0, 1.0);
            let fade_out = t
                .duration
                .map_or(1.0, |d| (((d - age) as f32) / motion::SLOW).clamp(0.0, 1.0));
            if t.duration.is_some_and(|d| age > d) {
                gone.push(t.id);
                continue;
            }
            ctx.request_repaint();
            let alpha = fade_in.min(fade_out);
            let id = Id::unique(("opencad_ui_toast", t.id));
            let resp = Area::new(id)
                .order(Order::Tooltip)
                .anchor(
                    Align2::RIGHT_BOTTOM,
                    vec2(-space::X3L, y + 8.0 * (1.0 - fade_in)),
                )
                .show(ctx, |ui| {
                    ui.set_style(theme::style(dark));
                    ui.multiply_opacity(alpha);
                    toast_body(ui, t.tone, &t.text)
                });
            if resp.inner {
                gone.push(t.id);
            }
            y -= resp.response.rect.height() + space::MD;
        }
        self.items.retain(|t| !gone.contains(&t.id));
    }
}

/// One toast's surface (also drawn inline by the gallery). `true` when
/// its close button was clicked.
pub fn toast_body(ui: &mut Ui, tone: Tone, text_: &str) -> bool {
    let c = colors(ui);
    let t = c.tone(tone);
    let mut close = false;
    Frame::new()
        .fill(c.surface_raised)
        .stroke(Stroke::new(1.0, c.ring))
        .corner_radius(radius::CARD)
        .inner_margin(Margin::symmetric(space::XL as i8, space::LG as i8))
        .shadow(Elevation::Popover.shadow(c.dark))
        .show(ui, |ui| {
            ui.set_max_width(340.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = space::LG;
                let (ir, _) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::hover());
                icon::paint(ui.painter(), ir, tone_icon(tone), t.solid);
                ui.add(egui::Label::new(text::rich(text_, Type::Body, c.text)).wrap());
                let x = ui.add(
                    super::button::IconButton::new(Icon::Close, crate::i18n::tr(Msg::Dismiss))
                        .size(Vec2::splat(20.0))
                        .icon_size(12.0),
                );
                close = x.clicked();
            });
        });
    close
}

/// An empty state: an icon in a tinted circle, a title, an explanation and
/// an optional action button (returned when clicked).
pub fn empty_state(
    ui: &mut Ui,
    icon_: Icon,
    title: &str,
    text_: &str,
    action: Option<&str>,
) -> Option<Response> {
    let c = colors(ui);
    let mut out = None;
    ui.vertical_centered(|ui| {
        ui.spacing_mut().item_spacing.y = space::MD;
        ui.add_space(space::X4L);
        let (r, _) = ui.allocate_exact_size(Vec2::splat(56.0), Sense::hover());
        ui.painter().circle_filled(r.center(), 28.0, c.accent_tint);
        icon::paint(
            ui.painter(),
            Rect::from_center_size(r.center(), Vec2::splat(26.0)),
            icon_,
            c.accent,
        );
        ui.add_space(space::XS);
        ui.label(text::rich(title, Type::Title, c.text));
        ui.add(egui::Label::new(text::rich(text_, Type::Body, c.text_secondary)).wrap());
        if let Some(a) = action {
            ui.add_space(space::XS);
            out = Some(ui.add(super::button::Button::primary(a)));
        }
        ui.add_space(space::X4L);
    });
    out
}

/// A progress bar `width` wide: `Some(0..1)` determinate, `None`
/// indeterminate (a sliding segment).
pub fn progress(ui: &mut Ui, value: Option<f32>, width: f32, tone: Tone) -> Response {
    let c = colors(ui);
    let (rect, resp) =
        ui.allocate_exact_size(vec2(width.min(ui.available_width()), 6.0), Sense::hover());
    resp.widget_info(|| {
        let mut info =
            egui::WidgetInfo::labeled(Role::ProgressIndicator, true, crate::i18n::tr(Msg::Loading));
        info.value = value.map(|v| f64::from(v * 100.0));
        info
    });
    let p = ui.painter();
    p.rect_filled(rect, radius::PILL, c.track_strong);
    let fill = match tone {
        Tone::Neutral | Tone::Info => c.accent_fill,
        other => c.tone(other).solid,
    };
    match value {
        Some(v) => {
            let v = ui
                .ctx()
                .animate_value_with_time(resp.id, v.clamp(0.0, 1.0), motion::SLOW);
            if v > 0.0 {
                let r = Rect::from_min_max(
                    rect.min,
                    pos2(rect.left() + rect.width() * v.max(0.02), rect.bottom()),
                );
                p.rect_filled(r, radius::PILL, fill);
            }
        }
        None => {
            ui.ctx().request_repaint();
            let t = (ui.input(|i| i.time) as f32 / (motion::SPIN * 1.4)).fract();
            let seg = rect.width() * 0.3;
            let x0 = rect.left() - seg + (rect.width() + seg) * motion::ease_in_out(t);
            let r = Rect::from_min_max(
                pos2(x0.max(rect.left()), rect.top()),
                pos2((x0 + seg).min(rect.right()), rect.bottom()),
            );
            if r.width() > 0.0 {
                p.rect_filled(r, radius::PILL, fill);
            }
        }
    }
    resp
}

/// A spinner (an arc turning once per [`motion::SPIN`]).
pub fn spinner(ui: &mut Ui, size: f32) -> Response {
    let c = colors(ui);
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(size), Sense::hover());
    state::a11y(
        &resp,
        Role::ProgressIndicator,
        crate::i18n::tr(Msg::Loading),
    );
    ui.ctx().request_repaint();
    let time = ui.input(|i| i.time) as f32;
    let start = time / motion::SPIN * std::f32::consts::TAU;
    let r = size / 2.0 - 1.5;
    let width = (size / 9.0).clamp(1.5, 3.0);
    let p = ui.painter();
    p.circle_stroke(rect.center(), r, Stroke::new(width, c.track_strong));
    let n = 24;
    let pts: Vec<_> = (0..=n)
        .map(|i| {
            let a = start + i as f32 / n as f32 * std::f32::consts::TAU * 0.28;
            rect.center() + vec2(a.cos(), a.sin()) * r
        })
        .collect();
    p.add(Shape::line(pts, Stroke::new(width, c.accent_fill)));
    resp
}

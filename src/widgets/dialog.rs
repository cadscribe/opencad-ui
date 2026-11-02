//! Dialogs: the modal dialog (centred, dimmed backdrop), the sheet
//! (attached under the title bar), their inline surface for previews,
//! the footer with buttons, and wizard steps.

use egui::{
    Align, Align2, Area, Frame, Id, Layout, Margin, Order, Rect, Response, Role, Sense, Stroke, Ui,
    Vec2, pos2, vec2,
};

use crate::icon::{self, Icon};
use crate::text::{self, Type};
use crate::theme::{self, colors};
use crate::tokens::{Colors, Elevation, Tone, radius, space};

/// The frame of dialogs.
pub fn dialog_frame(c: &Colors) -> Frame {
    Frame::new()
        .fill(c.surface_raised)
        .stroke(Stroke::new(1.0, c.ring))
        .corner_radius(radius::OVERLAY)
        .inner_margin(Margin::symmetric(28, 22))
        .shadow(Elevation::Dialog.shadow(c.dark))
}

/// The dialog surface drawn inline (gallery, previews), `width` wide.
pub fn surface<R>(ui: &mut Ui, width: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    let c = colors(ui);
    dialog_frame(c)
        .show(ui, |ui| {
            ui.set_width(width);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = space::X3L;
                add(ui)
            })
            .inner
        })
        .inner
}

/// The title and the explanation at the top of a dialog.
pub fn heading(ui: &mut Ui, title: &str, text_: &str) {
    let c = colors(ui);
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = space::SM;
        ui.add(egui::Label::new(text::rich(title, Type::Headline, c.text)).wrap());
        if !text_.is_empty() {
            ui.add(egui::Label::new(text::rich(text_, Type::Label, c.text_secondary)).wrap());
        }
    });
}

/// The footer: a divider, `left` (a tertiary "Cancel") and `right` (the
/// actions, primary last) aligned to the edges.
pub fn footer(ui: &mut Ui, left: impl FnOnce(&mut Ui), right: impl FnOnce(&mut Ui)) {
    super::card::divider(ui);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = space::LG;
        left(ui);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            // Right to left: the caller adds the primary button first.
            right(ui);
        });
    });
}

/// What happened in a modal dialog this frame.
#[derive(Debug)]
pub struct ModalOutput<R> {
    /// The content's value.
    pub inner: R,
    /// The user asked to close it: Escape, or a click on the backdrop
    /// (unless switched off).
    pub close: bool,
    /// Enter was pressed while no widget had the focus: the default
    /// (primary) button's action.
    pub confirm: bool,
}

/// A modal dialog: centred in the safe area ([`crate::window::safe_area`],
/// never under the title bar), over a dimmed backdrop, in the dialog
/// frame; Escape closes, Enter confirms.
#[derive(Debug, Clone, Copy)]
pub struct Dialog {
    id: Id,
    width: f32,
    backdrop_closes: bool,
    padding: Option<Margin>,
}

impl Dialog {
    pub fn new(id_salt: impl std::hash::Hash + std::fmt::Debug) -> Self {
        Dialog {
            id: Id::unique(("opencad_ui_modal", id_salt)),
            width: 380.0,
            backdrop_closes: true,
            padding: None,
        }
    }

    /// The content width.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Whether a click on the backdrop closes it (off for dialogs that
    /// would lose input, such as wizards).
    pub fn backdrop_closes(mut self, on: bool) -> Self {
        self.backdrop_closes = on;
        self
    }

    /// The inner margin instead of the dialog's (zero for a wizard, whose
    /// sidebar reaches the frame's edges).
    pub fn padding(mut self, padding: impl Into<Margin>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    pub fn show<R>(self, ctx: &egui::Context, add: impl FnOnce(&mut Ui) -> R) -> ModalOutput<R> {
        let dark = ctx.global_style().visuals.dark_mode;
        let c = Colors::of(dark);
        let safe = crate::window::safe_area(ctx);
        let area = egui::Modal::default_area(self.id).constrain_to(safe);
        let width = self.width.min(safe.width() - 64.0).max(160.0);
        let mut frame = dialog_frame(c);
        if let Some(m) = self.padding {
            frame = frame.inner_margin(m);
        }
        let resp = egui::Modal::new(self.id)
            .area(area)
            .frame(frame)
            .backdrop_color(c.scrim)
            .show(ctx, |ui| {
                ui.set_width(width);
                ui.spacing_mut().item_spacing.y = space::X3L;
                add(ui)
            });
        let top = resp.is_top_modal && !resp.any_popup_open;
        let escape =
            top && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let backdrop = self.backdrop_closes && resp.backdrop_response.clicked();
        let confirm = top
            && ctx.memory(|m| m.focused().is_none())
            && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
        ModalOutput {
            inner: resp.inner,
            close: escape || backdrop || resp.response.should_close(),
            confirm,
        }
    }
}

/// A modal dialog `width` wide ([`Dialog`] with the defaults).
pub fn modal<R>(
    ctx: &egui::Context,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    width: f32,
    add: impl FnOnce(&mut Ui) -> R,
) -> ModalOutput<R> {
    Dialog::new(id_salt).width(width).show(ctx, add)
}

/// A sheet: slides down from under the title bar (`top`, at least the
/// top of the safe area), over a dimmed window. `true` in the second value when the user asked to close it.
pub fn sheet<R>(
    ctx: &egui::Context,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    top: f32,
    width: f32,
    add: impl FnOnce(&mut Ui) -> R,
) -> (R, bool) {
    let dark = ctx.global_style().visuals.dark_mode;
    let c = Colors::of(dark);
    let top = top.max(crate::window::safe_area(ctx).top());
    let id = Id::unique(("opencad_ui_sheet", id_salt));
    let t = ctx.animate_bool_with_time_and_easing(
        id.with("in"),
        true,
        crate::tokens::motion::SLOW,
        crate::tokens::motion::ease_out,
    );
    let screen = ctx.content_rect();
    let backdrop = ctx.layer_painter(egui::LayerId::new(Order::Middle, id.with("backdrop")));
    backdrop.rect_filled(screen, 0.0, c.scrim.gamma_multiply(t));
    let frame = Frame::new()
        .fill(c.surface_raised)
        .stroke(Stroke::new(1.0, c.ring))
        .corner_radius(egui::CornerRadius {
            nw: 0,
            ne: 0,
            sw: radius::OVERLAY,
            se: radius::OVERLAY,
        })
        .inner_margin(Margin::symmetric(24, 20))
        .shadow(Elevation::Dialog.shadow(dark));
    let inner = Area::new(id)
        .order(Order::Foreground)
        .anchor(Align2::CENTER_TOP, vec2(0.0, top - 12.0 * (1.0 - t)))
        .show(ctx, |ui| {
            ui.set_style(theme::style(dark));
            frame
                .show(ui, |ui| {
                    ui.set_width(width);
                    ui.spacing_mut().item_spacing.y = space::XXL;
                    add(ui)
                })
                .inner
        });
    let close = ctx.input(|i| i.key_pressed(egui::Key::Escape));
    (inner.inner, close)
}

/// The state of a wizard step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepState {
    Done,
    Current,
    Upcoming,
}

/// The step list of a wizard: a numbered circle per step (a check when
/// done), the current step raised. Returns a clicked done step.
pub fn wizard_steps(ui: &mut Ui, steps: &[&str], current: usize) -> Option<usize> {
    let c = colors(ui);
    let mut clicked = None;
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = space::XS;
        for (i, name) in steps.iter().enumerate() {
            let st = if i < current {
                StepState::Done
            } else if i == current {
                StepState::Current
            } else {
                StepState::Upcoming
            };
            let sense = if st == StepState::Done {
                Sense::click()
            } else {
                Sense::hover()
            };
            let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 36.0), sense);
            let enabled = resp.enabled();
            resp.widget_info(|| {
                let mut info = egui::WidgetInfo::labeled(Role::ListItem, enabled, *name);
                info.selected = Some(st == StepState::Current);
                info
            });
            let p = ui.painter();
            if st == StepState::Current {
                p.add(
                    Elevation::Raised
                        .shadow(c.dark)
                        .as_shape(rect, radius::TILE),
                );
                p.rect_filled(rect, radius::TILE, c.surface_raised);
            } else if resp.hovered() {
                p.rect_filled(rect, radius::TILE, c.hover);
            }
            let cc = pos2(rect.left() + space::LG + 10.0, rect.center().y);
            match st {
                StepState::Done => {
                    p.circle_filled(cc, 10.0, c.success.solid);
                    icon::paint(
                        p,
                        Rect::from_center_size(cc, Vec2::splat(13.0)),
                        Icon::Check,
                        egui::Color32::WHITE,
                    );
                }
                StepState::Current => {
                    p.circle_filled(cc, 10.0, c.accent_fill);
                    let g = text::galley(p, &(i + 1).to_string(), Type::CaptionStrong, c.on_accent);
                    text::paint_in(
                        p,
                        Rect::from_center_size(cc, Vec2::splat(20.0)),
                        g,
                        Align::Center,
                        c.on_accent,
                    );
                }
                StepState::Upcoming => {
                    p.circle_stroke(cc, 9.25, Stroke::new(1.5, c.border_mixed));
                    let g = text::galley(p, &(i + 1).to_string(), Type::Caption, c.text_secondary);
                    text::paint_in(
                        p,
                        Rect::from_center_size(cc, Vec2::splat(20.0)),
                        g,
                        Align::Center,
                        c.text_secondary,
                    );
                }
            }
            let (t, fg) = if st == StepState::Current {
                (Type::LabelStrong, c.text)
            } else {
                (Type::Label, c.text_secondary)
            };
            let tr = Rect::from_min_max(pos2(cc.x + 20.0, rect.top()), rect.max);
            let g = text::truncated(p, name, t, fg, tr.width() - space::MD);
            text::paint_in(p, tr, g, Align::Min, fg);
            if resp.clicked() {
                clicked = Some(i);
            }
        }
    });
    clicked
}

/// The wizard layout inside a dialog surface: a sunken sidebar with the
/// title, the steps and a note; the page on the right.
pub fn wizard<R>(
    ui: &mut Ui,
    icon_: Icon,
    title: &str,
    steps: &[&str],
    current: usize,
    note: &str,
    page: impl FnOnce(&mut Ui) -> R,
) -> (R, Option<usize>) {
    let c = colors(ui);
    let mut clicked = None;
    // The sidebar's fill is painted once the page's height is known, so
    // that it reaches the bottom of the dialog.
    let bg = ui.painter().add(egui::Shape::Noop);
    let mut side = Rect::NOTHING;
    let r = ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        side = Frame::new()
            .inner_margin(Margin::symmetric(space::X3L as i8, 22))
            .corner_radius(egui::CornerRadius {
                nw: radius::OVERLAY,
                sw: radius::OVERLAY,
                ne: 0,
                se: 0,
            })
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.set_width(188.0);
                    ui.set_min_height(380.0);
                    ui.horizontal(|ui| {
                        let (sq, _) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover());
                        ui.painter().rect_filled(sq, radius::TILE, c.assistant);
                        icon::paint(ui.painter(), sq.shrink(7.0), icon_, egui::Color32::WHITE);
                        ui.add_space(space::LG - ui.spacing().item_spacing.x);
                        ui.label(text::rich(title, Type::Title, c.text));
                    });
                    ui.add_space(space::XL);
                    clicked = wizard_steps(ui, steps, current);
                    ui.add_space(space::X4L);
                    ui.add(
                        egui::Label::new(text::rich(note, Type::Caption, c.text_tertiary)).wrap(),
                    );
                })
            })
            .response
            .rect;
        Frame::new()
            .inner_margin(Margin {
                left: 28,
                right: 28,
                top: 26,
                bottom: 20,
            })
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = space::X3L;
                    page(ui)
                })
                .inner
            })
            .inner
    });
    let full = Rect::from_min_max(side.min, egui::pos2(side.max.x, r.response.rect.bottom()));
    ui.painter().set(
        bg,
        egui::Shape::rect_filled(
            full,
            egui::CornerRadius {
                nw: radius::OVERLAY,
                sw: radius::OVERLAY,
                ne: 0,
                se: 0,
            },
            c.surface_sunken,
        ),
    );
    (r.inner, clicked)
}

/// A status block of a dialog: a dot with a halo (pulsing while `busy`),
/// a title, a detail line and an optional link on the right.
pub fn status_block(
    ui: &mut Ui,
    tone: Tone,
    title: &str,
    detail: &str,
    busy: bool,
    link: Option<&str>,
) -> Option<Response> {
    let c = colors(ui);
    let t = c.tone(tone);
    let mut link_resp = None;
    Frame::new()
        .fill(c.field)
        .stroke(Stroke::new(1.0, c.divider))
        .corner_radius(radius::CARD)
        .inner_margin(Margin::symmetric(space::XXL as i8, space::XL as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                let (dot, _) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::hover());
                let pulse = if busy {
                    ui.ctx().request_repaint();
                    let time = ui.input(|i| i.time) as f32;
                    0.5 + 0.5 * (time * 3.0).sin()
                } else {
                    1.0
                };
                ui.painter()
                    .circle_filled(dot.center(), 5.0 + 4.0 * pulse, t.tint);
                ui.painter().circle_filled(dot.center(), 5.0, t.solid);
                ui.add_space(space::XS);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    ui.label(text::rich(title, Type::LabelStrong, c.text));
                    ui.label(text::rich(detail, Type::Body, c.text_secondary));
                });
                if let Some(l) = link {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        link_resp = Some(super::button::link(ui, l));
                    });
                }
            });
        });
    link_resp
}

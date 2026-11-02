//! Windows (ADR 0019 "Windows and dialogs"): the safe area, tool
//! windows and floating panels. Modal dialogs are in
//! [`crate::widgets::dialog`].
//!
//! * The **safe area** is the part of a viewport that in-canvas windows,
//!   dialogs and sheets may use: the content rect without the app's
//!   title / toolbar strip (on macOS the full-size content view puts the
//!   native traffic lights there). The app sets it once per frame
//!   ([`set_safe_area`]); everything that floats in the canvas clamps to
//!   [`safe_area`].
//! * A [`ToolWindow`] is a long-lived window (UI gallery, settings,
//!   shortcuts). Natively it is a real OS window (an egui immediate
//!   viewport): moved, resized and closed by the OS, with its own title
//!   bar, and its size and position remembered per id. Where egui has no
//!   multiple viewports (the web) it is a floating in-canvas window in
//!   the safe area, dragged by its header and resizable.
//! * A [`Floating`] panel is a small non-modal in-canvas window (filter
//!   assistant, assistant prompts), clamped to the safe area, optionally
//!   anchored to one of its corners.

use egui::{
    Align2, CentralPanel, Context, Frame, Id, Key, KeyboardShortcut, Margin, Modifiers, Pos2, Rect,
    Stroke, Ui, Vec2, ViewportBuilder, ViewportClass, ViewportId, WidgetText, pos2, vec2,
};

use crate::tokens::{Colors, Elevation, radius, space};

fn safe_area_id(viewport: ViewportId) -> Id {
    Id::unique(("opencad_ui_safe_area", viewport))
}

/// Sets the safe area of the current viewport for this and the following
/// frames: usually the content rect below the title bar and toolbar.
pub fn set_safe_area(ctx: &Context, rect: Rect) {
    let id = safe_area_id(ctx.viewport_id());
    ctx.data_mut(|d| d.insert_temp(id, rect));
}

/// The safe area of the current viewport: where in-canvas windows,
/// dialogs and sheets may be. The whole content rect if the app did not
/// set one.
pub fn safe_area(ctx: &Context) -> Rect {
    let screen = ctx.content_rect();
    let id = safe_area_id(ctx.viewport_id());
    match ctx.data(|d| d.get_temp::<Rect>(id)) {
        Some(r) if r.intersects(screen) => r.intersect(screen),
        _ => screen,
    }
}

/// Moves `rect` (keeping its size) into `safe`. A rect larger than the
/// safe area is aligned to its top left, so its top (the header) is never
/// above the safe area.
pub fn clamp_rect(rect: Rect, safe: Rect) -> Rect {
    let axis = |min: f32, len: f32, lo: f32, hi: f32| {
        if len >= hi - lo {
            lo
        } else {
            min.clamp(lo, hi - len)
        }
    };
    let x = axis(rect.left(), rect.width(), safe.left(), safe.right());
    let y = axis(rect.top(), rect.height(), safe.top(), safe.bottom());
    Rect::from_min_size(pos2(x, y), rect.size())
}

/// A rect of `size` centred in `safe` (clamped to it).
pub fn centred_in(safe: Rect, size: Vec2) -> Rect {
    clamp_rect(Rect::from_center_size(safe.center(), size), safe)
}

/// The frame of in-canvas windows: raised surface, hairline ring, the
/// overlay radius and the dialog shadow.
pub fn window_frame(c: &Colors) -> Frame {
    Frame::new()
        .fill(c.surface_raised)
        .stroke(Stroke::new(1.0, c.ring))
        .corner_radius(radius::OVERLAY)
        .inner_margin(Margin::same(space::XXL as i8))
        .shadow(Elevation::Dialog.shadow(c.dark))
}

/// The frame of the content of a tool window that is an OS window: the
/// window background, no border (the OS draws the chrome).
pub fn os_window_frame(c: &Colors) -> Frame {
    Frame::new()
        .fill(c.bg)
        .inner_margin(Margin::same(space::X3L as i8))
}

fn colors_of(ctx: &Context) -> &'static Colors {
    Colors::of(ctx.global_style().visuals.dark_mode)
}

/// A long-lived window: a real OS window natively, a floating in-canvas
/// window on the web. Call [`ToolWindow::show`] every frame, also while
/// closed (it forgets the window's opening geometry then).
#[derive(Debug, Clone)]
pub struct ToolWindow {
    key: String,
    title: String,
    default_size: Vec2,
    min_size: Vec2,
    resizable: bool,
    bare: bool,
}

impl ToolWindow {
    /// `key` is stable (not translated): it names the viewport and the
    /// remembered geometry; `title` is shown in the title bar.
    pub fn new(key: impl Into<String>, title: impl Into<String>) -> Self {
        ToolWindow {
            key: key.into(),
            title: title.into(),
            default_size: vec2(480.0, 360.0),
            min_size: vec2(240.0, 160.0),
            resizable: true,
            bare: false,
        }
    }

    /// The content fills the window edge to edge (a floating viewport),
    /// without the window's padding.
    pub fn bare(mut self) -> Self {
        self.bare = true;
        self
    }

    /// The size when first opened.
    pub fn default_size(mut self, size: impl Into<Vec2>) -> Self {
        self.default_size = size.into();
        self
    }

    pub fn min_size(mut self, size: impl Into<Vec2>) -> Self {
        self.min_size = size.into();
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    fn id(&self) -> Id {
        Id::unique(("opencad_ui_tool_window", &self.key))
    }

    /// The viewport id of the OS window.
    pub fn viewport_id(&self) -> ViewportId {
        ViewportId::from_hash_of(self.id())
    }

    /// Shows the window while `*open`; the user closing it sets `*open`
    /// to `false`. `add` draws the content.
    pub fn show(self, ctx: &Context, open: &mut bool, add: impl FnMut(&mut Ui)) {
        let id = self.id();
        if !*open {
            ctx.data_mut(|d| d.remove::<ViewportBuilder>(id.with("builder")));
            return;
        }
        if ctx.embed_viewports() {
            self.show_floating(ctx, open, add);
        } else {
            self.show_os_window(ctx, open, add);
        }
    }

    /// The builder of the OS window, made once when it opens (a builder
    /// that changes every frame would move the window back).
    fn builder(&self, ctx: &Context) -> ViewportBuilder {
        let mut b = ViewportBuilder::default()
            .with_title(self.title.clone())
            .with_inner_size(self.default_size)
            .with_min_inner_size(self.min_size)
            .with_resizable(self.resizable);
        let stored = ctx.data_mut(|d| d.get_persisted::<[f32; 4]>(self.id().with("geometry")));
        let monitor = ctx.input_for(ViewportId::ROOT, |i| i.viewport().monitor_size);
        let main = ctx.input_for(ViewportId::ROOT, |i| i.viewport().outer_rect);
        match stored {
            Some([x, y, w, h]) => {
                let size = vec2(w, h).max(self.min_size);
                b = b.with_inner_size(size);
                // Only where it is still on the (main) screen; displays
                // change.
                let on_screen = monitor.is_none_or(|m| {
                    Rect::from_min_size(Pos2::ZERO, m)
                        .intersects(Rect::from_min_size(pos2(x, y), size))
                });
                if on_screen {
                    b = b.with_position(pos2(x, y));
                }
            }
            None => {
                if let Some(main) = main {
                    b = b.with_position(main.center() - self.default_size / 2.0);
                }
            }
        }
        b
    }

    fn show_os_window(self, ctx: &Context, open: &mut bool, mut add: impl FnMut(&mut Ui)) {
        let id = self.id();
        // (Not built inside `data_mut`: `builder` reads the context too.)
        let key = id.with("builder");
        let builder = match ctx.data(|d| d.get_temp::<ViewportBuilder>(key)) {
            Some(b) => b,
            None => {
                let b = self.builder(ctx);
                ctx.data_mut(|d| d.insert_temp(key, b.clone()));
                b
            }
        };
        // The title follows the language.
        let builder = builder.with_title(self.title.clone());
        let closed = ctx.show_viewport_immediate(self.viewport_id(), builder, |ui, class| {
            let ctx = ui.ctx().clone();
            let c = colors_of(&ctx);
            if class == ViewportClass::EmbeddedWindow {
                // A backend without multiple viewports: egui already put
                // us in a window.
                add(ui);
                return false;
            }
            let (outer, inner) = ctx.input(|i| (i.viewport().outer_rect, i.viewport().inner_rect));
            if let (Some(o), Some(i)) = (outer, inner) {
                ctx.data_mut(|d| {
                    d.insert_persisted(
                        id.with("geometry"),
                        [o.left(), o.top(), i.width(), i.height()],
                    );
                });
            }
            let frame = if self.bare {
                os_window_frame(c).inner_margin(0)
            } else {
                os_window_frame(c)
            };
            let bare = self.bare;
            CentralPanel::default()
                .frame(frame)
                .show(ui, |ui| scroll_unless_bare(ui, bare, &mut add));
            let close_key = KeyboardShortcut::new(Modifiers::COMMAND, Key::W);
            ctx.input(|i| i.viewport().close_requested())
                || ctx.input_mut(|i| i.consume_shortcut(&close_key))
        });
        if closed {
            *open = false;
        }
    }

    fn show_floating(self, ctx: &Context, open: &mut bool, add: impl FnMut(&mut Ui)) {
        Floating::new(self.id(), self.title)
            .default_size(self.default_size)
            .min_size(self.min_size)
            .resizable(self.resizable)
            .bare(self.bare)
            .show(ctx, Some(open), add);
    }
}

/// A non-modal in-canvas window in the safe area: dragged by its header
/// (title bar), optionally resizable, anchored or centred.
#[derive(Debug, Clone)]
pub struct Floating {
    id: Id,
    title: WidgetText,
    default_size: Option<Vec2>,
    min_size: Vec2,
    resizable: bool,
    anchor: Option<(Align2, Vec2)>,
    default_pos: Option<Pos2>,
    bare: bool,
}

impl Floating {
    pub fn new(id: Id, title: impl Into<WidgetText>) -> Self {
        Floating {
            id,
            title: title.into(),
            default_size: None,
            min_size: vec2(160.0, 80.0),
            resizable: false,
            anchor: None,
            default_pos: None,
            bare: false,
        }
    }

    /// The content goes nearly edge to edge (a floating viewport).
    pub fn bare(mut self, bare: bool) -> Self {
        self.bare = bare;
        self
    }

    /// The size when first shown (else the content's size).
    pub fn default_size(mut self, size: impl Into<Vec2>) -> Self {
        self.default_size = Some(size.into());
        self
    }

    pub fn min_size(mut self, size: impl Into<Vec2>) -> Self {
        self.min_size = size.into();
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// The top-left corner when first shown (else centred in the safe
    /// area).
    pub fn default_pos(mut self, pos: impl Into<Pos2>) -> Self {
        self.default_pos = Some(pos.into());
        self
    }

    /// Pins the window to a corner of the safe area (`offset` moves it
    /// inwards); it cannot be dragged then.
    pub fn anchor(mut self, align: Align2, offset: impl Into<Vec2>) -> Self {
        self.anchor = Some((align, offset.into()));
        self
    }

    /// Shows the window; `open` adds a close button. Returns the window's
    /// rect and the content's value (`None` while hidden).
    pub fn show<R>(
        self,
        ctx: &Context,
        open: Option<&mut bool>,
        mut add: impl FnMut(&mut Ui) -> R,
    ) -> Option<(Rect, Option<R>)> {
        let safe = safe_area(ctx);
        let c = colors_of(ctx);
        let mut w = egui::Window::new(self.title)
            .id(self.id)
            .collapsible(false)
            .resizable(self.resizable)
            .min_size(self.min_size)
            .frame(if self.bare {
                window_frame(c).inner_margin(space::XS)
            } else {
                window_frame(c)
            })
            .constrain_to(safe);
        match (self.anchor, self.default_pos, self.default_size) {
            (Some((align, offset)), _, size) => {
                w = w.anchor(align, offset);
                if let Some(size) = size {
                    w = w.default_size(size);
                }
            }
            (None, Some(pos), Some(size)) => {
                w = w.default_rect(clamp_rect(Rect::from_min_size(pos, size), safe));
            }
            (None, Some(pos), None) => {
                let r = clamp_rect(Rect::from_min_size(pos, self.min_size), safe);
                w = w.default_pos(r.min);
            }
            (None, None, Some(size)) => w = w.default_rect(centred_in(safe, size)),
            (None, None, None) => w = w.pivot(Align2::CENTER_CENTER).default_pos(safe.center()),
        }
        if let Some(open) = open {
            w = w.open(open);
        }
        let bare = self.bare;
        w.show(ctx, |ui| scroll_unless_bare(ui, bare, &mut add))
            .map(|r| (r.response.rect, r.inner))
    }
}

/// Window content: a bare window (a viewport) fills the window itself;
/// any other content scrolls vertically when the window is shorter than
/// it.
fn scroll_unless_bare<R>(ui: &mut Ui, bare: bool, add: &mut impl FnMut(&mut Ui) -> R) -> R {
    if bare {
        return add(ui);
    }
    egui::ScrollArea::vertical()
        .auto_shrink([false, true])
        .show(ui, |ui| add(ui))
        .inner
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A safe area below a 52 pt title / toolbar strip.
    fn safe() -> Rect {
        Rect::from_min_max(pos2(0.0, 52.0), pos2(1280.0, 800.0))
    }

    #[test]
    fn windows_never_go_above_the_title_bar_inset() {
        let s = safe();
        for (pos, size) in [
            (pos2(100.0, -40.0), vec2(300.0, 200.0)),
            (pos2(100.0, 10.0), vec2(300.0, 200.0)),
            (pos2(-500.0, 900.0), vec2(300.0, 200.0)),
            (pos2(10.0, 10.0), vec2(2000.0, 2000.0)),
            (pos2(1200.0, 700.0), vec2(400.0, 300.0)),
        ] {
            let r = clamp_rect(Rect::from_min_size(pos, size), s);
            assert!(r.top() >= s.top(), "{r:?} above the inset");
            assert!(r.left() >= s.left(), "{r:?} left of the window");
            assert_eq!(r.size(), size);
            if size.x <= s.width() && size.y <= s.height() {
                assert!(s.contains_rect(r), "{r:?} not inside {s:?}");
            }
        }
        // Inside already: unchanged.
        let inside = Rect::from_min_size(pos2(200.0, 100.0), vec2(300.0, 200.0));
        assert_eq!(clamp_rect(inside, s), inside);
    }

    #[test]
    fn centred_windows_sit_in_the_safe_area() {
        let s = safe();
        let r = centred_in(s, vec2(400.0, 300.0));
        assert_eq!(r.center(), s.center());
        let tall = centred_in(s, vec2(400.0, 5000.0));
        assert!((tall.top() - s.top()).abs() < 1e-3);
    }

    #[test]
    fn safe_area_defaults_to_the_content_rect_and_follows_the_app() {
        let ctx = Context::default();
        let screen = Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 800.0));
        let input = || egui::RawInput {
            screen_rect: Some(screen),
            ..Default::default()
        };
        let mut out = ctx.run_ui(input(), |ui| {
            assert_eq!(safe_area(ui.ctx()), screen);
            set_safe_area(ui.ctx(), safe());
        });
        out.textures_delta.clear();
        let mut out = ctx.run_ui(input(), |ui| assert_eq!(safe_area(ui.ctx()), safe()));
        out.textures_delta.clear();
    }

    /// The in-canvas fallback: a window placed (or dragged) under the
    /// title bar is pushed below it.
    #[test]
    fn floating_windows_stay_below_the_title_bar() {
        let ctx = Context::default();
        let screen = Rect::from_min_size(Pos2::ZERO, vec2(1280.0, 800.0));
        let mut rect = Rect::NOTHING;
        for _ in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            };
            let mut out = ctx.run_ui(input, |ui| {
                set_safe_area(ui.ctx(), safe());
                let id = Id::unique("test_floating");
                let shown = Floating::new(id, "Test")
                    .default_size(vec2(300.0, 200.0))
                    .show(ui.ctx(), None, |ui| ui.label("x"));
                rect = shown.expect("shown").0;
                // Try to move it into the title bar strip.
                if let Some(mut a) = egui::AreaState::load(ui.ctx(), id) {
                    a.set_left_top_pos(pos2(40.0, 0.0));
                    let layer = egui::LayerId::new(egui::Order::Middle, id);
                    ui.ctx().memory_mut(|m| m.areas_mut().set_state(layer, a));
                }
            });
            out.textures_delta.clear();
        }
        assert!(rect.top() >= safe().top(), "{rect:?}");
    }
}

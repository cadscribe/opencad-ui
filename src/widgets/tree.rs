//! Tree rows: indentation guides, an expand chevron (turning when
//! opened), an optional drawing-file state control, an icon or colour
//! swatch, the label (one line, cut with "…"), a secondary note, a
//! right-aligned trailing value in tabular figures (a storey's level)
//! and state toggles on the right (visible, locked, …). ←/→ close and
//! open a focused row.

use egui::{
    Align, Color32, Painter, Rect, Response, Role, Sense, Stroke, StrokeKind, Ui, Vec2, pos2, vec2,
};

use crate::i18n::tr;
use crate::icon::{self, Icon};
use crate::state;
use crate::text::{self, Type};
use crate::theme::colors;
use crate::tokens::{Colors, motion, radius, size, space};

crate::messages! {
    enum Msg {
        Active: "Active", "Aktiv";
        Editable: "Editable", "Aktiv im Hintergrund";
        Reference: "Reference", "Passiv";
        Hidden: "Hidden", "Ausgeblendet";
        ActiveTip: "Drawn into; one file at a time", "Hier wird gezeichnet; immer nur eines";
        EditableTip: "Shown and editable", "Sichtbar und bearbeitbar";
        ReferenceTip: "Shown grey, not editable", "Grau sichtbar, nicht bearbeitbar";
        HiddenTip: "Not shown", "Nicht sichtbar";
        CycleTip: "Click: next state · right click: all states", "Klick: nächster Zustand · Rechtsklick: alle Zustände";
    }
}

/// The state of a drawing file (Allplan's "Teilbild" states).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileState {
    /// The file new elements go into; one at a time.
    Active,
    /// Shown and editable ("active in the background").
    Editable,
    /// Shown grey, not editable ("passive").
    Reference,
    Hidden,
}

impl FileState {
    pub const ALL: [FileState; 4] = [
        FileState::Active,
        FileState::Editable,
        FileState::Reference,
        FileState::Hidden,
    ];

    /// The state after a click on the state control. The active file is
    /// not reached by cycling (it is chosen by clicking the row); it goes
    /// to the background, hidden files come back editable.
    pub fn next(self) -> FileState {
        match self {
            FileState::Active | FileState::Hidden => FileState::Editable,
            FileState::Editable => FileState::Reference,
            FileState::Reference => FileState::Hidden,
        }
    }

    pub fn label(self) -> &'static str {
        tr(match self {
            FileState::Active => Msg::Active,
            FileState::Editable => Msg::Editable,
            FileState::Reference => Msg::Reference,
            FileState::Hidden => Msg::Hidden,
        })
    }

    pub fn description(self) -> &'static str {
        tr(match self {
            FileState::Active => Msg::ActiveTip,
            FileState::Editable => Msg::EditableTip,
            FileState::Reference => Msg::ReferenceTip,
            FileState::Hidden => Msg::HiddenTip,
        })
    }
}

/// Paints the glyph of `state` centred in `rect`: a filled accent disc
/// (active), an accent ring with a dot (editable), a grey ring
/// (reference), a faint struck-through ring (hidden).
pub fn paint_file_state(p: &Painter, rect: Rect, state: FileState, c: &Colors) {
    let ctr = rect.center();
    let r = (rect.width().min(rect.height()) * 0.3).max(3.0);
    match state {
        FileState::Active => {
            p.circle_filled(ctr, r + 0.5, c.accent_fill);
        }
        FileState::Editable => {
            p.circle_stroke(ctr, r, Stroke::new(1.5, c.accent));
            p.circle_filled(ctr, r * 0.4, c.accent);
        }
        FileState::Reference => {
            p.circle_stroke(ctr, r, Stroke::new(1.3, c.text_secondary));
        }
        FileState::Hidden => {
            let s = Stroke::new(1.2, c.text_disabled);
            p.circle_stroke(ctr, r, s);
            let d = r * std::f32::consts::FRAC_1_SQRT_2 + 1.0;
            p.line_segment([ctr + vec2(-d, d), ctr + vec2(d, -d)], s);
        }
    }
}

/// The legend of the file states (the state control's tooltip): every
/// state's glyph, name and meaning, the current one in bold.
pub fn file_state_legend(ui: &mut Ui, current: FileState) {
    let c = colors(ui);
    ui.spacing_mut().item_spacing.y = space::XS;
    for s in FileState::ALL {
        ui.horizontal(|ui| {
            let (r, _) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::hover());
            paint_file_state(ui.painter(), r, s, c);
            let t = if s == current {
                Type::BodyStrong
            } else {
                Type::Body
            };
            text::label(ui, s.label(), t, c.text);
            text::label(ui, s.description(), Type::Caption, c.text_secondary);
        });
    }
    ui.add_space(space::XXS);
    text::label(ui, tr(Msg::CycleTip), Type::Caption, c.text_tertiary);
}

/// Minimum width the label keeps before a trailing value gives way.
pub const MIN_LABEL_W: f32 = 56.0;

/// How the label and a trailing value share `avail` points: the
/// label's maximum width and whether the trailing value is shown. The
/// trailing value (a level, a count) is dropped before the label is cut
/// below [`MIN_LABEL_W`] (or below its own width, if it is shorter).
pub fn fit_trailing(avail: f32, label_w: f32, trailing_w: f32) -> (f32, bool) {
    if trailing_w <= 0.0 {
        return (avail.max(0.0), false);
    }
    let rest = avail - trailing_w - space::MD;
    if rest >= label_w.min(MIN_LABEL_W) {
        (rest, true)
    } else {
        (avail.max(0.0), false)
    }
}

/// A state toggle on the right of a tree row.
#[derive(Debug)]
pub struct Toggle<'a> {
    pub on: &'a mut bool,
    pub icon_on: Icon,
    pub icon_off: Icon,
    pub tip: &'a str,
}

/// What happened to a tree row.
#[derive(Debug)]
pub struct TreeResponse {
    pub response: Response,
    /// The open state changed.
    pub toggled_open: bool,
    /// The index of a state toggle that was switched.
    pub toggled: Option<usize>,
    /// The file state control (when the row has one).
    pub state: Option<Response>,
    /// The label was cut; show the full text in the tooltip.
    pub truncated: bool,
}

impl TreeResponse {
    /// The file state control was clicked.
    pub fn state_clicked(&self) -> bool {
        self.state.as_ref().is_some_and(Response::clicked)
    }
}

/// Indentation per tree level.
pub const INDENT: f32 = 16.0;

/// A tree row.
#[derive(Debug)]
#[must_use = "call `.show(ui)`"]
pub struct TreeRow<'a> {
    depth: usize,
    label: &'a str,
    note: &'a str,
    trailing: &'a str,
    icon: Option<Icon>,
    icon_color: Option<Color32>,
    swatch: Option<Color32>,
    file_state: Option<FileState>,
    open: Option<&'a mut bool>,
    selected: bool,
    toggles: Vec<Toggle<'a>>,
    muted: bool,
    guides: bool,
}

impl<'a> TreeRow<'a> {
    pub fn new(label: &'a str, depth: usize) -> Self {
        TreeRow {
            depth,
            label,
            note: "",
            trailing: "",
            icon: None,
            icon_color: None,
            swatch: None,
            file_state: None,
            open: None,
            selected: false,
            toggles: Vec::new(),
            muted: false,
            guides: true,
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// The icon in another colour (an assistant's mark).
    pub fn icon_color(mut self, color: Color32) -> Self {
        self.icon_color = Some(color);
        self
    }

    /// A colour swatch instead of an icon (layers).
    pub fn swatch(mut self, color: Color32) -> Self {
        self.swatch = Some(color);
        self
    }

    /// A secondary note after the label (scale, count).
    pub fn note(mut self, note: &'a str) -> Self {
        self.note = note;
        self
    }

    /// A value right-aligned in tabular figures, secondary colour (a
    /// storey's level); dropped first when the row is narrow.
    pub fn trailing(mut self, trailing: &'a str) -> Self {
        self.trailing = trailing;
        self
    }

    /// The drawing-file state control before the icon.
    pub fn file_state(mut self, state: FileState) -> Self {
        self.file_state = Some(state);
        self
    }

    /// The row has children; `open` is their visibility.
    pub fn expandable(mut self, open: &'a mut bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Greyed (hidden or passive items).
    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    /// Indentation guides (on by default).
    pub fn guides(mut self, on: bool) -> Self {
        self.guides = on;
        self
    }

    pub fn toggle(mut self, toggle: Toggle<'a>) -> Self {
        self.toggles.push(toggle);
        self
    }

    pub fn show(mut self, ui: &mut Ui) -> TreeResponse {
        let c = colors(ui);
        let (rect, resp) =
            ui.allocate_exact_size(vec2(ui.available_width(), size::ROW_H), Sense::click());
        let enabled = resp.enabled();
        let (selected, label) = (self.selected, self.label);
        resp.widget_info(|| {
            let mut info = egui::WidgetInfo::selected(Role::TreeItem, enabled, selected, label);
            info.selected = Some(selected);
            info
        });
        let s = state::state(ui, &resp);
        let p = ui.painter().clone();
        if self.selected {
            p.rect_filled(rect, radius::ROW, c.selection);
        } else {
            let h = state::hover_fill(ui, resp.id, s, c);
            if h.a() > 0 {
                p.rect_filled(rect, radius::ROW, h);
            }
        }
        let left = rect.left() + space::SM;
        if self.guides {
            for d in 0..self.depth {
                let gx = (left + d as f32 * INDENT + 7.0).round() + 0.5;
                p.vline(gx, rect.y_range(), Stroke::new(1.0, c.divider));
            }
        }
        let mut toggled_open = false;
        let mut x = left + self.depth as f32 * INDENT;
        if let Some(open) = self.open.as_deref_mut() {
            let cr = Rect::from_center_size(pos2(x + 7.0, rect.center().y), Vec2::splat(16.0));
            let cresp = ui.interact(cr, resp.id.with("chevron"), Sense::click());
            let keys = if s.focused {
                ui.input(|i| {
                    (
                        i.key_pressed(egui::Key::ArrowRight),
                        i.key_pressed(egui::Key::ArrowLeft),
                    )
                })
            } else {
                (false, false)
            };
            if cresp.clicked() || resp.double_clicked() || (keys.0 && !*open) || (keys.1 && *open) {
                *open = !*open;
                toggled_open = true;
            }
            let t = state::animate(ui, resp.id.with("open"), *open, motion::NORMAL);
            let rot = egui::emath::Rot2::from_angle(std::f32::consts::FRAC_PI_2 * t);
            let pts: Vec<egui::Pos2> = [vec2(-1.8, -3.6), vec2(1.8, 0.0), vec2(-1.8, 3.6)]
                .iter()
                .map(|v| cr.center() + rot * *v)
                .collect();
            let col = if cresp.hovered() {
                c.text
            } else {
                c.text_secondary
            };
            p.add(egui::Shape::line(pts, Stroke::new(1.4, col)));
        }
        x += 16.0;
        let fg = if self.muted || s.disabled {
            c.text_disabled
        } else {
            match self.file_state {
                Some(FileState::Reference) => c.text_secondary,
                Some(FileState::Hidden) => c.text_tertiary,
                _ => c.text,
            }
        };
        let mut state_resp = None;
        if let Some(fs) = self.file_state {
            let sr = Rect::from_center_size(pos2(x + 9.0, rect.center().y), vec2(20.0, 20.0));
            let sresp = ui.interact(sr, resp.id.with("file_state"), Sense::click());
            state::a11y(&sresp, Role::Button, fs.label());
            if sresp.hovered() {
                p.rect_filled(sr, radius::ROW, c.hover);
            }
            paint_file_state(&p, sr, fs, c);
            let sresp = sresp.on_hover_ui(|ui| file_state_legend(ui, fs));
            state_resp = Some(sresp);
            x += 20.0 + space::XS;
        }
        if let Some(i) = self.icon {
            let ic = if self.muted || self.file_state == Some(FileState::Hidden) {
                c.text_disabled
            } else if let Some(col) = self.icon_color {
                col
            } else if self.selected {
                c.accent
            } else {
                c.icon
            };
            icon::paint(
                &p,
                Rect::from_center_size(pos2(x + 7.0, rect.center().y), Vec2::splat(14.0)),
                i,
                ic,
            );
            x += 14.0 + space::SM;
        } else if let Some(col) = self.swatch {
            let sw = Rect::from_center_size(pos2(x + 6.0, rect.center().y), Vec2::splat(11.0));
            p.rect_filled(sw, radius::XS, col);
            p.rect_stroke(
                sw,
                radius::XS,
                Stroke::new(1.0, c.border),
                StrokeKind::Inside,
            );
            x += 12.0 + space::SM;
        }
        // Toggles from the right.
        let mut right = rect.right() - space::XS;
        let mut toggled = None;
        for (i, t) in self.toggles.iter_mut().enumerate().rev() {
            let tr = Rect::from_center_size(pos2(right - 11.0, rect.center().y), vec2(22.0, 20.0));
            right -= 22.0;
            let tresp = ui.interact(tr, resp.id.with(("toggle", i)), Sense::click());
            state::a11y_selected(&tresp, Role::Button, *t.on, t.tip);
            if tresp.hovered() {
                p.rect_filled(tr, radius::ROW, c.hover);
            }
            let ic = if *t.on { c.icon } else { c.text_disabled };
            icon::paint(
                &p,
                Rect::from_center_size(tr.center(), Vec2::splat(14.0)),
                if *t.on { t.icon_on } else { t.icon_off },
                ic,
            );
            if tresp.clicked() {
                *t.on = !*t.on;
                toggled = Some(i);
            }
            let _ = tresp.on_hover_text(t.tip);
        }
        let ty = if self.selected {
            Type::BodyStrong
        } else {
            Type::Body
        };
        let avail = right - x - space::SM;
        let full_w = text::galley(&p, self.label, ty, fg).size().x;
        let trail_w = if self.trailing.is_empty() {
            0.0
        } else {
            text::tabular_width(&p, self.trailing, Type::Body)
        };
        let (label_w, show_trailing) = fit_trailing(avail, full_w, trail_w);
        let g = text::truncated(&p, self.label, ty, fg, label_w);
        let truncated = g.elided;
        let gw = g.size().x;
        text::paint_in(
            &p,
            Rect::from_min_max(pos2(x, rect.top()), pos2(x + gw, rect.bottom())),
            g,
            Align::Min,
            fg,
        );
        if show_trailing {
            text::paint_tabular(
                &p,
                Rect::from_min_max(
                    pos2(right - space::XS - trail_w, rect.top()),
                    pos2(right - space::XS, rect.bottom()),
                ),
                self.trailing,
                Type::Body,
                c.text_secondary,
                Align::Max,
            );
        }
        let note_right = if show_trailing {
            right - trail_w - space::MD
        } else {
            right
        };
        if !self.note.is_empty() && x + gw + 20.0 < note_right {
            let ng = text::truncated(
                &p,
                self.note,
                Type::Body,
                c.text_tertiary,
                note_right - x - gw - space::SM,
            );
            text::paint_in(
                &p,
                Rect::from_min_max(
                    pos2(x + gw + space::SM, rect.top()),
                    pos2(note_right, rect.bottom()),
                ),
                ng,
                Align::Min,
                c.text_tertiary,
            );
        }
        if s.focused {
            state::focus_ring(ui, rect, radius::ROW, c);
        }
        TreeResponse {
            response: resp,
            toggled_open,
            toggled,
            state: state_resp,
            truncated,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_cycle_skips_active_and_returns_to_editable() {
        assert_eq!(FileState::Active.next(), FileState::Editable);
        assert_eq!(FileState::Editable.next(), FileState::Reference);
        assert_eq!(FileState::Reference.next(), FileState::Hidden);
        assert_eq!(FileState::Hidden.next(), FileState::Editable);
        // Cycling never makes a file active.
        let mut s = FileState::Hidden;
        for _ in 0..8 {
            s = s.next();
            assert_ne!(s, FileState::Active);
        }
    }

    #[test]
    fn trailing_value_gives_way_before_the_label() {
        // Room for both.
        assert_eq!(
            fit_trailing(200.0, 80.0, 40.0),
            (200.0 - 40.0 - space::MD, true)
        );
        // Narrow: the level is dropped, the label keeps the full width.
        assert_eq!(fit_trailing(80.0, 120.0, 40.0), (80.0, false));
        // A short label fits next to the level even below MIN_LABEL_W.
        assert_eq!(
            fit_trailing(80.0, 20.0, 40.0),
            (80.0 - 40.0 - space::MD, true)
        );
        // No trailing value.
        assert_eq!(fit_trailing(100.0, 300.0, 0.0), (100.0, false));
        assert_eq!(fit_trailing(-5.0, 10.0, 0.0), (0.0, false));
    }
}

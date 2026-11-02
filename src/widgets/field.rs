//! Input fields: text fields (prefix, suffix, clear button, error),
//! search fields, number and length fields (units, arithmetic, the mixed
//! "–" state, tabular figures) and the select (combo box).

use egui::text::CCursorRange;
use egui::{
    Align, Color32, Frame, Id, Key, Margin, Rect, Response, Role, Sense, Stroke, StrokeKind,
    TextEdit, Ui, UiBuilder, Vec2, pos2, vec2,
};

use crate::i18n::{Lang, lang, tr};
use crate::icon::{self, Icon};
use crate::length::{self, InputError, Unit};
use crate::state::{self, State};
use crate::text::{self, Type};
use crate::theme::colors;
use crate::tokens::{Colors, radius, size, space};

crate::messages! {
    enum Msg {
        Clear: "Clear", "Leeren";
        Invalid: "Not a valid number", "Keine gültige Zahl";
        DivisionByZero: "Division by zero", "Division durch null";
        LengthTimesLength: "Lengths cannot be multiplied", "Längen lassen sich nicht multiplizieren";
        NoUnits: "No units here", "Hier ohne Einheit";
        Mixed: "Differing values", "Unterschiedliche Werte";
    }
}

/// The look of a field frame in a state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldLook {
    /// Filled with a border (fields in cards).
    Bordered,
    /// Sunken fill without a border (search in popovers).
    Filled,
}

/// Paints a field background: fill, border, error, mixed, focus ring.
pub fn paint_frame(
    ui: &Ui,
    rect: Rect,
    s: State,
    look: FieldLook,
    error: bool,
    mixed: bool,
    c: &Colors,
) {
    let p = ui.painter();
    let r = radius::FIELD;
    let fill = match (look, s.focused, s.disabled) {
        (_, _, true) => c.surface_sunken,
        (FieldLook::Bordered, true, _) => c.field_focus,
        (FieldLook::Bordered, false, _) if mixed => c.surface_raised,
        (FieldLook::Bordered, false, _) => c.field,
        (FieldLook::Filled, _, _) => c.surface,
    };
    p.rect_filled(rect, r, fill);
    if s.focused {
        if error {
            p.rect_stroke(
                rect.expand(0.5),
                r + 1,
                Stroke::new(size::FOCUS_RING, c.danger.tint),
                StrokeKind::Outside,
            );
            p.rect_stroke(
                rect,
                r,
                Stroke::new(1.0, c.danger.solid),
                StrokeKind::Inside,
            );
        } else {
            state::focus_ring(ui, rect, r, c);
        }
        return;
    }
    if error {
        p.rect_stroke(
            rect,
            r,
            Stroke::new(1.0, c.danger.solid),
            StrokeKind::Inside,
        );
    } else if mixed {
        let rr = rect.shrink(0.5);
        let pts = [
            rr.left_top(),
            rr.right_top(),
            rr.right_bottom(),
            rr.left_bottom(),
            rr.left_top(),
        ];
        p.extend(egui::Shape::dashed_line(
            &pts,
            Stroke::new(1.0, c.border_mixed),
            3.0,
            2.0,
        ));
    } else if look == FieldLook::Bordered {
        let border = if s.hovered { c.border_mixed } else { c.border };
        p.rect_stroke(rect, r, Stroke::new(1.0, border), StrokeKind::Inside);
    }
}

/// What sits before or after the text.
#[derive(Debug, Clone, Copy)]
pub enum Affix<'a> {
    Icon(Icon),
    Text(&'a str),
}

/// A single-line text field.
#[derive(Debug)]
#[must_use = "call `.show(ui)`"]
pub struct TextField<'a> {
    text: &'a mut String,
    id_salt: Option<Id>,
    hint: &'a str,
    prefix: Option<Affix<'a>>,
    suffix: Option<Affix<'a>>,
    clearable: bool,
    error: Option<&'a str>,
    width: Option<f32>,
    look: FieldLook,
    height: f32,
    align: Align,
    label: &'a str,
    password: bool,
}

impl<'a> TextField<'a> {
    pub fn new(text: &'a mut String) -> Self {
        TextField {
            text,
            id_salt: None,
            hint: "",
            prefix: None,
            suffix: None,
            clearable: false,
            error: None,
            width: None,
            look: FieldLook::Bordered,
            height: size::FIELD_H,
            align: Align::Min,
            label: "",
            password: false,
        }
    }

    pub fn id_salt(mut self, salt: impl std::hash::Hash + std::fmt::Debug) -> Self {
        self.id_salt = Some(Id::unique(salt));
        self
    }

    /// Placeholder text.
    pub fn hint(mut self, hint: &'a str) -> Self {
        self.hint = hint;
        self
    }

    /// The accessible name (the label next to the field).
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn prefix(mut self, prefix: Affix<'a>) -> Self {
        self.prefix = Some(prefix);
        self
    }

    pub fn suffix(mut self, suffix: Affix<'a>) -> Self {
        self.suffix = Some(suffix);
        self
    }

    /// A clear button while there is text.
    pub fn clearable(mut self) -> Self {
        self.clearable = true;
        self
    }

    /// An error: red outline and the message under the field.
    pub fn error(mut self, error: Option<&'a str>) -> Self {
        self.error = error;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn look(mut self, look: FieldLook) -> Self {
        self.look = look;
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    /// Hides the text (tokens); pasting still works.
    pub fn password(mut self, on: bool) -> Self {
        self.password = on;
        self
    }

    /// Text alignment (numbers: right).
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let c = colors(ui);
        let w = self
            .width
            .unwrap_or(ui.available_width())
            .min(ui.available_width())
            .max(40.0);
        let (rect, outer) = ui.allocate_exact_size(vec2(w, self.height), Sense::hover());
        let id = self.id_salt.unwrap_or_else(|| outer.id.with("edit"));
        let focused = ui.memory(|m| m.has_focus(id));
        let mut s = state::state(ui, &outer);
        s.focused |= focused;
        if state::forced(ui) == Some(state::Forced::Focus) {
            s.focused = true;
        }
        paint_frame(ui, rect, s, self.look, self.error.is_some(), false, c);
        let mut inner = rect.shrink2(vec2(space::SM, 0.0));
        let affix = |ui: &mut Ui, a: Affix<'_>, at_left: bool, inner: &mut Rect| {
            let fg = c.text_tertiary;
            match a {
                Affix::Icon(i) => {
                    let x = if at_left {
                        inner.left() + 7.0
                    } else {
                        inner.right() - 7.0
                    };
                    icon::paint(
                        ui.painter(),
                        Rect::from_center_size(pos2(x, inner.center().y), Vec2::splat(13.0)),
                        i,
                        fg,
                    );
                    if at_left {
                        inner.min.x += 14.0 + space::XS;
                    } else {
                        inner.max.x -= 14.0 + space::XS;
                    }
                }
                Affix::Text(t) => {
                    let g = text::galley(ui.painter(), t, Type::Body, fg);
                    let gw = g.size().x;
                    let r = if at_left {
                        Rect::from_min_max(inner.min, pos2(inner.left() + gw, inner.bottom()))
                    } else {
                        Rect::from_min_max(pos2(inner.right() - gw, inner.top()), inner.max)
                    };
                    text::paint_in(ui.painter(), r, g, Align::Min, fg);
                    if at_left {
                        inner.min.x += gw + space::XS;
                    } else {
                        inner.max.x -= gw + space::XS;
                    }
                }
            }
        };
        if let Some(a) = self.prefix {
            affix(ui, a, true, &mut inner);
        }
        let mut cleared = false;
        if self.clearable && !self.text.is_empty() {
            let xr = Rect::from_center_size(
                pos2(inner.right() - 7.0, inner.center().y),
                Vec2::splat(16.0),
            );
            let xresp = ui.interact(xr, id.with("clear"), Sense::click());
            state::a11y(&xresp, Role::Button, tr(Msg::Clear));
            let fg = if xresp.hovered() {
                c.text_secondary
            } else {
                c.text_tertiary
            };
            icon::paint(ui.painter(), xr.shrink(1.5), Icon::Clear, fg);
            if xresp.clicked() {
                self.text.clear();
                cleared = true;
                ui.memory_mut(|m| m.request_focus(id));
            }
            let _ = xresp.on_hover_text(tr(Msg::Clear));
            inner.max.x -= 16.0 + space::XS;
        }
        if let Some(a) = self.suffix {
            affix(ui, a, false, &mut inner);
        }
        let mut child = ui.new_child(
            UiBuilder::new()
                .max_rect(inner)
                .layout(egui::Layout::left_to_right(Align::Center)),
        );
        let fg = state::text_color(s, c.text, c);
        let edit = TextEdit::singleline(self.text)
            .id(id)
            .font(Type::Body.font())
            .text_color(fg)
            .frame(Frame::NONE)
            .margin(Margin::ZERO)
            .align(egui::Align2([self.align, Align::Center]))
            .password(self.password)
            .desired_width(inner.width());
        let mut resp = child.add(edit);
        let show_hint = self.text.is_empty() && !self.hint.is_empty();
        if show_hint {
            let g = text::truncated(
                ui.painter(),
                self.hint,
                Type::Body,
                c.text_tertiary,
                inner.width(),
            );
            let pos = egui::pos2(
                match self.align {
                    Align::Max => inner.right() - g.size().x,
                    Align::Center => inner.center().x - g.size().x / 2.0,
                    Align::Min => inner.left(),
                },
                inner.center().y - g.size().y / 2.0,
            );
            ui.painter().galley(pos, g, c.text_tertiary);
        }
        if !self.label.is_empty() {
            let label = self.label;
            let txt = self.text.clone();
            resp.widget_info(|| egui::WidgetInfo::text_edit(true, txt.clone(), label, ""));
        }
        if cleared {
            resp.mark_changed();
        }
        if let Some(e) = self.error {
            let g = text::wrapped(ui.painter(), e, Type::Caption, c.danger.text, w);
            let (er, _) = ui.allocate_exact_size(g.size(), Sense::hover());
            ui.painter().galley(er.min, g, c.danger.text);
        }
        resp
    }
}

/// A search field: magnifier, placeholder, clear button. `filled` uses
/// the sunken look of popovers (type picker), else the bordered one
/// (ribbon).
pub fn search(ui: &mut Ui, text: &mut String, hint: &str, filled: bool) -> Response {
    TextField::new(text)
        .hint(hint)
        .label(hint)
        .prefix(Affix::Icon(Icon::Search))
        .clearable()
        .look(if filled {
            FieldLook::Filled
        } else {
            FieldLook::Bordered
        })
        .height(if filled { 30.0 } else { 26.0 })
        .show(ui)
}

fn error_text(e: InputError) -> &'static str {
    match e {
        InputError::DivisionByZero => tr(Msg::DivisionByZero),
        InputError::LengthTimesLength => tr(Msg::LengthTimesLength),
        InputError::UnitNotAllowed => tr(Msg::NoUnits),
        InputError::Empty | InputError::Unexpected(_) => tr(Msg::Invalid),
    }
}

/// A number or length field: shows the formatted value with tabular
/// figures; while editing it takes arithmetic and (for lengths) units;
/// Enter or leaving the field commits, Escape cancels, ↑/↓ step.
#[derive(Debug)]
#[must_use = "call `.show(ui)`"]
pub struct NumberField<'a> {
    value: &'a mut f64,
    id_salt: Id,
    unit: Option<Unit>,
    suffix: &'a str,
    decimals: usize,
    step: f64,
    range: std::ops::RangeInclusive<f64>,
    mixed: bool,
    width: f32,
    label: &'a str,
}

impl<'a> NumberField<'a> {
    /// A plain number field.
    pub fn new(id_salt: impl std::hash::Hash + std::fmt::Debug, value: &'a mut f64) -> Self {
        NumberField {
            value,
            id_salt: Id::unique(("number_field", id_salt)),
            unit: None,
            suffix: "",
            decimals: 2,
            step: 1.0,
            range: f64::NEG_INFINITY..=f64::INFINITY,
            mixed: false,
            width: size::NUMBER_W,
            label: "",
        }
    }

    /// A length field: `value` in millimetres, shown and typed in `unit`.
    pub fn length(
        id_salt: impl std::hash::Hash + std::fmt::Debug,
        mm: &'a mut f64,
        unit: Unit,
    ) -> Self {
        let mut f = Self::new(id_salt, mm);
        f.unit = Some(unit);
        f.decimals = unit.decimals();
        f.step = unit.mm() * if unit == Unit::M { 0.01 } else { 1.0 };
        f
    }

    pub fn decimals(mut self, d: usize) -> Self {
        self.decimals = d;
        self
    }

    /// A unit text after a plain number ("°", "W/(m²K)").
    pub fn suffix(mut self, s: &'a str) -> Self {
        self.suffix = s;
        self
    }

    /// The ↑/↓ step (in the stored unit: millimetres for lengths).
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    pub fn range(mut self, range: std::ops::RangeInclusive<f64>) -> Self {
        self.range = range;
        self
    }

    /// Values differ in a multi-selection: shows "–" in a dashed frame.
    pub fn mixed(mut self, mixed: bool) -> Self {
        self.mixed = mixed;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    /// The accessible name.
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    fn display(&self, v: f64, lang: Lang) -> String {
        match self.unit {
            Some(u) => length::format_length(v, u, lang),
            None if self.suffix.is_empty() => length::format_number(v, self.decimals, lang),
            None => format!(
                "{} {}",
                length::format_number(v, self.decimals, lang),
                self.suffix
            ),
        }
    }

    fn parse(&self, s: &str) -> Result<f64, InputError> {
        let s = s.trim().trim_end_matches(self.suffix).trim();
        match self.unit {
            Some(u) => length::parse_length(s, u),
            None => length::parse_number(s),
        }
    }

    /// Shows the field; `changed()` when a new value was committed.
    pub fn show(self, ui: &mut Ui) -> Response {
        let c = colors(ui);
        let id = self.id_salt;
        let buf_id = id.with("buffer");
        let err_id = id.with("error");
        let w = self.width.min(ui.available_width()).max(40.0);
        let (rect, outer) = ui.allocate_exact_size(vec2(w, size::FIELD_H), Sense::hover());
        let editing = ui.memory(|m| m.has_focus(id));
        let lang = lang();
        let shown = if self.mixed {
            "–".to_owned()
        } else {
            self.display(*self.value, lang)
        };
        let mut buf: String = if editing {
            ui.data(|d| d.get_temp::<String>(buf_id))
                .unwrap_or_else(|| shown.clone())
        } else {
            shown.clone()
        };
        let error: Option<InputError> = ui.data(|d| d.get_temp(err_id));
        let mut s = state::state(ui, &outer);
        s.focused |= editing;
        if state::forced(ui) == Some(state::Forced::Focus) {
            s.focused = true;
        }
        paint_frame(
            ui,
            rect,
            s,
            FieldLook::Bordered,
            error.is_some(),
            self.mixed && !editing,
            c,
        );
        let inner = rect.shrink2(vec2(7.0, 0.0));
        let mut child = ui.new_child(
            UiBuilder::new()
                .max_rect(inner)
                .layout(egui::Layout::right_to_left(Align::Center)),
        );
        // Outside editing the value is painted with tabular figures; the
        // text edit underneath stays transparent (it keeps the caret and
        // click behaviour).
        let fg = state::text_color(s, c.text, c);
        let edit_color = if editing { fg } else { Color32::TRANSPARENT };
        let mut resp = child.add(
            TextEdit::singleline(&mut buf)
                .id(id)
                .font(Type::Body.font())
                .text_color(edit_color)
                .frame(Frame::NONE)
                .margin(Margin::ZERO)
                .align(egui::Align2::RIGHT_CENTER)
                .desired_width(inner.width()),
        );
        if !editing {
            let col = if self.mixed { c.text_secondary } else { fg };
            text::paint_tabular(ui.painter(), inner, &shown, Type::Body, col, Align::Max);
        }
        let label = if self.label.is_empty() {
            tr(Msg::Mixed)
        } else {
            self.label
        };
        let value = *self.value;
        resp.widget_info(|| {
            let mut info = egui::WidgetInfo::drag_value(true, value);
            info.label = Some(label.to_owned());
            info
        });
        let mut commit: Option<f64> = None;
        if resp.gained_focus() {
            // Start with the value (empty for mixed), all selected.
            buf = if self.mixed {
                String::new()
            } else {
                shown.clone()
            };
            if let Some(mut st) = TextEdit::load_state(ui.ctx(), id) {
                st.cursor
                    .set_char_range(Some(CCursorRange::select_all_str(&buf)));
                st.store(ui.ctx(), id);
            }
        }
        if resp.has_focus() {
            let (up, down) = ui.input_mut(|i| {
                (
                    i.consume_key(egui::Modifiers::NONE, Key::ArrowUp),
                    i.consume_key(egui::Modifiers::NONE, Key::ArrowDown),
                )
            });
            if up || down {
                let base = self.parse(&buf).unwrap_or(*self.value);
                let v = base + if up { self.step } else { -self.step };
                commit = Some(v);
                buf = self.display(v.clamp(*self.range.start(), *self.range.end()), lang);
            }
            ui.data_mut(|d| d.insert_temp(buf_id, buf.clone()));
        }
        if resp.lost_focus() {
            let cancelled = ui.input(|i| i.key_pressed(Key::Escape));
            ui.data_mut(|d| d.remove::<String>(buf_id));
            if cancelled || (self.mixed && buf.trim().is_empty()) || buf == shown {
                ui.data_mut(|d| d.remove::<InputError>(err_id));
            } else {
                match self.parse(&buf) {
                    Ok(v) => {
                        commit = Some(v);
                        ui.data_mut(|d| d.remove::<InputError>(err_id));
                    }
                    Err(e) => {
                        ui.data_mut(|d| d.insert_temp(err_id, e));
                    }
                }
            }
        }
        if let Some(v) = commit {
            let v = v.clamp(*self.range.start(), *self.range.end());
            if (v - *self.value).abs() > 1e-12 || self.mixed {
                *self.value = v;
                resp.mark_changed();
            }
        }
        if let Some(e) = error {
            resp = resp.on_hover_text(error_text(e));
        } else if self.mixed && !editing {
            resp = resp.on_hover_text(tr(Msg::Mixed));
        }
        resp
    }
}

/// A select (combo box): a field-like button with the chosen option and
/// a chevron; the options open in a menu with a checkmark on the chosen
/// one. ↑/↓ change the value while it has focus.
pub fn select<T: PartialEq + Copy>(
    ui: &mut Ui,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    value: &mut T,
    options: &[(T, &str)],
    width: f32,
) -> Response {
    let c = colors(ui);
    let w = width.min(ui.available_width()).max(60.0);
    let (rect, _) = ui.allocate_exact_size(vec2(w, size::FIELD_H), Sense::hover());
    let mut resp = ui.interact(
        rect,
        Id::unique(("opencad_ui_select", id_salt)),
        Sense::click(),
    );
    let current = options.iter().position(|(v, _)| v == value);
    let label = current.map_or("–", |i| options[i].1);
    state::a11y(&resp, Role::ComboBox, label);
    let s = state::state(ui, &resp);
    paint_frame(
        ui,
        rect,
        s,
        FieldLook::Bordered,
        false,
        current.is_none(),
        c,
    );
    let fg = state::text_color(s, c.text, c);
    let inner = rect.shrink2(vec2(space::MD, 0.0));
    let g = text::truncated(ui.painter(), label, Type::Body, fg, inner.width() - 16.0);
    text::paint_in(ui.painter(), inner, g, Align::Min, fg);
    icon::paint(
        ui.painter(),
        Rect::from_center_size(
            pos2(inner.right() - 6.0, inner.center().y),
            Vec2::splat(12.0),
        ),
        Icon::ChevronDown,
        if s.disabled {
            c.text_disabled
        } else {
            c.text_secondary
        },
    );
    if resp.has_focus() {
        let (up, down) = ui.input_mut(|i| {
            (
                i.consume_key(egui::Modifiers::NONE, Key::ArrowUp),
                i.consume_key(egui::Modifiers::NONE, Key::ArrowDown),
            )
        });
        let i = current.unwrap_or(0);
        let to = if up && i > 0 {
            Some(i - 1)
        } else if down && i + 1 < options.len() {
            Some(i + 1)
        } else {
            None
        };
        if let Some(j) = to {
            *value = options[j].0;
            resp.mark_changed();
        }
    }
    let mut picked = None;
    super::menu::menu_popup(ui, &resp).width(w).show(|ui| {
        ui.set_min_width(w - 10.0);
        for (i, (_, l)) in options.iter().enumerate() {
            if super::menu::MenuItem::new(l)
                .checked(Some(i) == current)
                .show(ui)
                .clicked()
            {
                picked = Some(i);
            }
        }
    });
    if let Some(i) = picked
        && Some(i) != current
    {
        *value = options[i].0;
        resp.mark_changed();
    }
    resp
}

#[cfg(test)]
mod tests {
    use egui::{CentralPanel, Context, Event, Id, RawInput};

    use super::TextField;

    /// A paste (what eframe makes of ⌘V / Ctrl+V with the system
    /// clipboard) lands in a focused field, also in a password field.
    #[test]
    fn paste_into_a_password_field() {
        let ctx = Context::default();
        let mut token = String::new();
        let frame = |ctx: &Context, events: Vec<Event>, token: &mut String| {
            let input = RawInput {
                events,
                ..RawInput::default()
            };
            let mut out = ctx.run_ui(input, |ui| {
                CentralPanel::default().show(ui, |ui| {
                    TextField::new(token)
                        .id_salt("token")
                        .password(true)
                        .show(ui);
                });
            });
            out.textures_delta.clear();
        };
        frame(&ctx, Vec::new(), &mut token);
        ctx.memory_mut(|m| m.request_focus(Id::unique("token")));
        frame(&ctx, Vec::new(), &mut token);
        frame(&ctx, vec![Event::Paste("vbp_secret".into())], &mut token);
        assert_eq!(token, "vbp_secret");
    }
}

//! A table: a header with sortable columns, optional zebra rows, a
//! selected row, fixed and flexible column widths, scrolling with a
//! fixed header; only visible rows are built.

use egui::{
    Align, Id, Layout, Rect, Response, Role, ScrollArea, Sense, Stroke, Ui, UiBuilder, Vec2, pos2,
    vec2,
};

use crate::icon::{self, Icon};
use crate::state;
use crate::text::{self, Type};
use crate::theme::colors;
use crate::tokens::{over, radius, size, space};

/// The width of a column.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Width {
    /// Points.
    Fixed(f32),
    /// A share of the remaining width.
    Fill(f32),
}

/// A column.
#[derive(Debug, Clone, Copy)]
pub struct Column<'a> {
    pub title: &'a str,
    pub width: Width,
    pub align: Align,
    pub sortable: bool,
}

impl<'a> Column<'a> {
    pub fn new(title: &'a str, width: Width) -> Self {
        Column {
            title,
            width,
            align: Align::Min,
            sortable: true,
        }
    }

    /// Right-aligned (numbers).
    pub fn numeric(mut self) -> Self {
        self.align = Align::Max;
        self
    }

    pub fn unsortable(mut self) -> Self {
        self.sortable = false;
        self
    }
}

/// Which column sorts the rows, and the direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Sort {
    pub column: usize,
    pub ascending: bool,
}

/// What happened in a table this frame.
#[derive(Debug, Clone, Default)]
pub struct TableResponse {
    /// A row was clicked.
    pub clicked: Option<usize>,
    /// The hovered row and its response (for a tooltip with details).
    pub hovered: Option<(usize, Response)>,
    /// The sort changed (re-sort the rows).
    pub sort_changed: bool,
}

/// A table.
#[derive(Debug)]
#[must_use = "call `.show(ui, rows, cell)`"]
pub struct Table<'a> {
    id: Id,
    columns: &'a [Column<'a>],
    sort: Option<&'a mut Sort>,
    zebra: bool,
    selected: Option<usize>,
    max_height: f32,
    row_height: f32,
}

impl<'a> Table<'a> {
    pub fn new(id_salt: impl std::hash::Hash + std::fmt::Debug, columns: &'a [Column<'a>]) -> Self {
        Table {
            id: Id::unique(("opencad_ui_table", id_salt)),
            columns,
            sort: None,
            zebra: false,
            selected: None,
            max_height: 320.0,
            row_height: size::ROW_H,
        }
    }

    pub fn sort(mut self, sort: &'a mut Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn zebra(mut self, on: bool) -> Self {
        self.zebra = on;
        self
    }

    pub fn selected(mut self, row: Option<usize>) -> Self {
        self.selected = row;
        self
    }

    pub fn max_height(mut self, h: f32) -> Self {
        self.max_height = h;
        self
    }

    fn widths(&self, total: f32) -> Vec<f32> {
        let fixed: f32 = self
            .columns
            .iter()
            .map(|c| match c.width {
                Width::Fixed(w) => w,
                Width::Fill(_) => 0.0,
            })
            .sum();
        let shares: f32 = self
            .columns
            .iter()
            .map(|c| match c.width {
                Width::Fill(s) => s,
                Width::Fixed(_) => 0.0,
            })
            .sum();
        let free = (total - fixed - space::XXL * 2.0).max(0.0);
        self.columns
            .iter()
            .map(|c| match c.width {
                Width::Fixed(w) => w,
                Width::Fill(s) if shares > 0.0 => free * s / shares,
                Width::Fill(_) => 0.0,
            })
            .collect()
    }

    /// Shows `rows` rows; `cell(ui, row, column)` draws a cell (left- or
    /// right-aligned per column, vertically centred).
    pub fn show(
        mut self,
        ui: &mut Ui,
        rows: usize,
        mut cell: impl FnMut(&mut Ui, usize, usize),
    ) -> TableResponse {
        let c = colors(ui);
        let mut out = TableResponse::default();
        let width = ui.available_width();
        let spacing = ui.spacing().item_spacing.y;
        ui.spacing_mut().item_spacing.y = 0.0;
        let widths = self.widths(width);
        // Header.
        let (hr, _) = ui.allocate_exact_size(vec2(width, 26.0), Sense::hover());
        let p = ui.painter().clone();
        p.hline(hr.x_range(), hr.bottom() - 0.5, Stroke::new(1.0, c.divider));
        let mut x = hr.left() + space::XXL;
        for (i, (col, w)) in self.columns.iter().zip(&widths).enumerate() {
            let r = Rect::from_min_size(pos2(x, hr.top()), vec2(*w, hr.height()));
            x += w;
            let sense = if col.sortable && self.sort.is_some() {
                Sense::click()
            } else {
                Sense::hover()
            };
            let resp = ui.interact(r, self.id.with(("head", i)), sense);
            state::a11y(&resp, Role::ColumnHeader, col.title);
            let active = self.sort.as_ref().is_some_and(|s| s.column == i);
            let fg = if active { c.text } else { c.text_secondary };
            let t = if active {
                Type::CaptionStrong
            } else {
                Type::Caption
            };
            let arrow_w = if active { 14.0 } else { 0.0 };
            let inner = r.shrink2(vec2(space::XS, 0.0));
            let g = text::truncated(&p, col.title, t, fg, inner.width() - arrow_w);
            let gw = g.size().x;
            let tx = match col.align {
                Align::Max => inner.right() - gw - arrow_w,
                _ => inner.left(),
            };
            text::paint_in(
                &p,
                Rect::from_min_max(pos2(tx, r.top()), pos2(tx + gw, r.bottom())),
                g,
                Align::Min,
                fg,
            );
            if active {
                let asc = self.sort.as_ref().is_some_and(|s| s.ascending);
                icon::paint(
                    &p,
                    Rect::from_center_size(pos2(tx + gw + 8.0, r.center().y), Vec2::splat(12.0)),
                    if asc {
                        Icon::ChevronUp
                    } else {
                        Icon::ChevronDown
                    },
                    fg,
                );
            } else if resp.hovered() && sense == Sense::click() {
                p.rect_filled(r, radius::ROW, c.hover);
            }
            if resp.clicked()
                && let Some(s) = self.sort.as_deref_mut()
            {
                if s.column == i {
                    s.ascending = !s.ascending;
                } else {
                    *s = Sort {
                        column: i,
                        ascending: true,
                    };
                }
                out.sort_changed = true;
            }
        }
        // Body.
        let rh = self.row_height;
        ScrollArea::vertical()
            .id_salt(self.id.with("body"))
            .max_height(self.max_height)
            .auto_shrink([false, true])
            .show_rows(ui, rh, rows, |ui, range| {
                for row in range {
                    let (rr, resp) = ui.allocate_exact_size(vec2(width, rh), Sense::click());
                    let p = ui.painter();
                    let selected = self.selected == Some(row);
                    if selected {
                        p.rect_filled(rr, radius::ROW, c.selection);
                    } else if resp.hovered() {
                        p.rect_filled(rr, radius::ROW, c.hover);
                    } else if self.zebra && row % 2 == 1 {
                        p.rect_filled(rr, 0.0, over(c.surface_raised, c.hover.gamma_multiply(0.5)));
                    }
                    if resp.clicked() {
                        out.clicked = Some(row);
                    }
                    if resp.hovered() {
                        out.hovered = Some((row, resp.clone()));
                    }
                    let mut x = rr.left() + space::XXL;
                    for (ci, (col, w)) in self.columns.iter().zip(&widths).enumerate() {
                        let r = Rect::from_min_size(pos2(x, rr.top()), vec2(*w, rh))
                            .shrink2(vec2(space::XS, 0.0));
                        x += w;
                        let layout = match col.align {
                            Align::Max => Layout::right_to_left(Align::Center),
                            _ => Layout::left_to_right(Align::Center),
                        };
                        let mut child = ui.new_child(UiBuilder::new().max_rect(r).layout(layout));
                        child.set_clip_rect(r.intersect(ui.clip_rect()));
                        cell(&mut child, row, ci);
                    }
                }
            });
        ui.spacing_mut().item_spacing.y = spacing;
        out
    }
}

/// A text cell (truncated, tooltip with the full text).
pub fn cell_text(ui: &mut Ui, text: &str) -> Response {
    let c = colors(ui);
    text::label(ui, text, Type::Body, c.text)
}

/// A number cell with tabular figures.
pub fn cell_number(ui: &mut Ui, text: &str) -> Response {
    let c = colors(ui);
    text::tabular(ui, text, Type::Body, c.text, None)
}

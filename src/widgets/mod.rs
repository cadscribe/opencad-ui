//! The widget set. Every widget reads its colours from the theme of the
//! `Ui` ([`crate::theme::colors`]), shows hover, pressed, focus and
//! disabled states ([`crate::state`]), reports its role and label to
//! AccessKit and works with the keyboard (Tab focus, Enter / Space to
//! activate, arrows where a widget has a value to step).

pub mod button;
pub mod card;
pub mod chip;
pub mod dialog;
pub mod dock;
pub mod feedback;
pub mod field;
pub mod floatbar;
pub mod list;
pub mod menu;
pub mod scroll;
pub mod segmented;
pub mod splitter;
pub mod table;
pub mod tabs;
pub mod toggle;
pub mod toolbar;
pub mod tooltip;
pub mod tree;

pub use button::{Button, ButtonSize, IconButton, Variant};
pub use card::{Card, collapsible, divider, form_row, inset, section};
pub use field::{NumberField, TextField, search, select};
pub use floatbar::{Dock, FloatBar};
pub use list::ListRow;
pub use menu::MenuItem;
pub use segmented::{Segment, Segmented};
pub use table::{Column, Table};
pub use toggle::{checkbox, radio, switch};
pub use toolbar::{RibbonTool, ToolButton};
pub use tree::TreeRow;

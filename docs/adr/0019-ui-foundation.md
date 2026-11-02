# ADR 0019 — UI foundation (`opencad-ui`)

- Status: accepted
- Date: 2026-10-29
- Supersedes: ADR 0004, "Update (UI styling)" (tokens and widgets)

> Copied from OpenCAD's `docs/adr/` when the crate moved into this
> repository (2026-11-02). "The app" is the OpenCAD app; other ADR
> numbers refer to OpenCAD's ADRs. Since then the crate lives in its own
> repository (`cadscribe/opencad-ui`) and OpenCAD and OpenSolve depend on
> it by git tag.

## Context

The UI technology spike (OpenCAD's `docs/research/ui-tech-spike.md`)
compared egui with a UI foundation (A) against HTML/CSS in Tauri with the
engine in wasm (B). The decision was to **stay with egui** and first build
one foundation — tokens, typography, icons, widgets — and then move every
screen onto it. Until now the styling lived in the app: `theme::PanelColours`
and `ui_kit.rs` for the palette, `style.rs` for the native look, a
hand-drawn icon set in `icons.rs`, and per-screen sizes and colours
elsewhere. The approved mockups (palette, dark palette, window, type
picker, ribbon, ribbon pages, workspace, wizard, chat) define the target
look.

## Decision

### A crate of its own

`opencad-ui` depends on `egui` only (the version eframe brings,
`default_fonts`; no new crates in the lock file). It has no app or model
types, builds for `wasm32-unknown-unknown` (it is in the wasm list of CI
and the Makefile) and is used by the app.

| Module | Content |
|---|---|
| `tokens` | `Colors` (light and dark, semantic), `Tone` and `ToneColors` (neutral, info, success, warning, danger: tint, text, solid), `space`, `radius`, `size`, `Elevation`, `motion`, colour helpers (`over`, `mix`, `contrast`) |
| `text` | the type scale `Type`, truncation and wrapping helpers, tabular figures |
| `fonts` | the UI font per platform, one font family per `Weight` |
| `theme` | egui `Visuals` and spacing from the tokens (both themes), `colors(ui)`, `scope_theme` |
| `state` | hover / pressed / focus / disabled state, focus ring, animations, AccessKit labels, forced states for the gallery |
| `svg`, `icon` | the SVG path parser and the `Icon` set |
| `length` | number and length input (arithmetic, units) and formatting |
| `widgets` | the widget set (below) |
| `i18n` | language, `Message`, `messages!` (moved here from the app, ADR 0011) |
| `gallery` | every token and widget, light and dark side by side |

### Tokens

- **Colours are semantic**: `bg`, `chrome`, `surface`, `surface_raised`,
  `surface_sunken`, `field`, `track`, `thumb`, `border`,
  `border_strong`, `border_mixed`, `divider`, `text`,
  `text_secondary`, `text_tertiary`, `text_disabled`, `icon`, `accent`
  (text and outlines), `accent_fill` (+ hover, pressed; filled
  controls), `on_accent`, `accent_tint`, `accent_text`, `danger_fill`,
  `highlight` (the hovered menu row, white text), `selection` (soft),
  `text_selection`, `focus` (the ring), `hover` and `pressed`
  (translucent overlays), `glass`, `scrim`, `shadow`, `ring`, code
  colours, `assistant`, and the five tones. Light values are the
  mockups'; dark values the dark artboard's. A test keeps text ≥ 7:1,
  secondary ≥ 4.5:1, tertiary ≥ 3:1 on every surface and ≥ 4.5:1 for
  text on accent, highlight, tints and tones.
- **Spacing** 2, 4, 6, 8, 10, 12, 14, 16, 20, 24, 32 (`space::XXS` …
  `X6L`). **Radii** 3 (swatches), 5 (rows), 6 (fields), 7 (buttons),
  8 (tiles), 9 (menus), 10 (cards, panes), 12 (dialogs, popovers,
  floating toolbars), pill. **Sizes**: fields and small buttons 24,
  buttons 28 / 32, menu rows 24, list rows 26 / 44, icons 13 / 16 / 20 /
  26, label column 92, focus ring 3.
- **Elevation**: flat, card, pane (with a hairline ring), raised (the
  segment thumb), popover, dialog; dark shadows are stronger.
- **Motion**: 0.08 s (hover), 0.14 s (switches, thumbs, chevrons),
  0.22 s (collapsing, fades, toasts), ease-out for state changes,
  ease-in-out for movement; 1 s per spinner turn.

### Typography

The scale: caption 11, caption-strong 11 medium, overline 11 semibold
in capitals with 0.45 pt tracking, body 12, body-medium, body-strong 12
semibold, label 13 (dialogs, menus), label-strong, title 14 semibold,
headline 20 semibold, display 26 semibold, mono 12.

**Fonts.** egui 0.37 renders variable fonts (`skrifa`), so each weight is
registered as its own font family with a fixed `wght` coordinate
(`Weight::family`); egui's own widgets can then use any weight through a
plain `FontId`. The font per platform:

- macOS: **SF Pro** (`SFNS.ttf`, variable; 400 / 510 / 590 / 700) and SF
  Mono, read from the system at runtime — Apple's licence does not allow
  bundling them.
- Windows 11: **Segoe UI Variable** (`SegUIVar.ttf`); Windows 10: Segoe UI
  and Segoe UI Semibold (static); Consolas.
- Linux and the web: **Inter 4.1** (variable), bundled.

**Decision on Inter.** Bundle it. Linux has no common UI font and the web
build had egui's default font, which looks nothing like the mockups.
Inter is the closest open match to SF Pro (metrics and shapes), is
designed for UI sizes and is licensed under the **SIL Open Font License
1.1**, which allows bundling and embedding in software of any licence;
only selling the font by itself is excluded. It is a data asset, not a
code dependency: `assets/fonts/InterVariable.ttf`
(880 KB, from the official v4.1 release of 2024-11-15) with its licence
`assets/fonts/Inter-OFL.txt`. It is `include_bytes!`-ed only on targets
other than macOS and Windows, so desktop binaries there do not grow; the
web build grows by about 0.9 MB (≈ 0.45 MB compressed). The font is not
modified (no subsetting), so the OFL's renaming rule for modified
versions does not apply. The licence file must ship with source
distributions (it does, it is in the crate) and binaries that bundle it
should mention it in their credits.

**Tabular figures.** egui has no OpenType features (`tnum`). Numbers that
must line up (tables, coordinates, values outside editing, tags) are
painted by `text::paint_tabular`: every digit centred in a cell as wide
as the widest digit. Number fields show their value this way and switch
to plain text while editing.

### Icons

One `Icon` enum (144 icons): the tool icons of `RibbonPages.dc.html` on
a 24 grid, and the interface icons of the workspace, ribbon, chat and
wizard mockups (or drawn to match them) on a 16 grid. Each icon is a list
of SVG path layers (`M L H V A C S Q T Z`, absolute and relative):
stroked, with a width factor, dashed, or filled. Paths are parsed once
and cached in view-box units; painting transforms the points and strokes
them as egui shapes with round caps and joins (discs at sharp corners and
open ends), so icons are crisp at any size and at 1× and 2×; the origin is
snapped to physical pixels. The stroke width follows the drawn size
(≈ 1.25 pt at 16, 1.6 pt at 26), like optical sizes. Filled paths are
triangulated (ear clipping) when concave. A test parses every path and
checks it stays inside its view box. No SVG rasteriser and no texture
cache are needed.

### Widgets

Every widget takes its colours from the theme of the `Ui` it is drawn
in (`theme::colors(ui)`), shows hover, pressed, focus (the accent outline
with the soft ring) and disabled states, reports its role and label to
AccessKit (`state::a11y`), and works with the keyboard: Tab focus, Enter
/ Space activate, arrows where a value can be stepped (segmented
controls, selects, number fields, splitters, tree rows). Short
animations: hover fades, the switch knob, the check mark drawing in, the
sliding segment thumb and ribbon underline, chevrons turning, collapsing
sections, toasts fading.

- **Buttons**: `Button` (primary, secondary, tertiary, destructive; small
  / medium / large; leading icon, trailing chevron, fill width),
  `IconButton` (selected, framed), `link`.
- **Toggles**: `switch`, `checkbox`, `checkbox_tri` (mixed), `radio`,
  `radio_value`.
- **Segmented control** (text, icon or both; fill or compact width).
- **Chips**: status chip, value tag, filter chip (selectable, removable),
  anchor chip with a menu, value chip (an editable list value: tabular
  figures, muted for a default, a remove button fading in on hover or
  focus, Delete / Backspace removes) and the dashed "+" add chip
  (`chip::value`, `chip::add`; the project's snap values use them).
- **Fields**: `TextField` (prefix / suffix text or icon, clear button,
  error message, bordered or filled), `search`, `NumberField` (plain or
  length with a unit; arithmetic `1 m + 24 cm`, `(2,5 + 1) × 2`; decimal
  comma or point; Enter / leaving commits, Esc cancels, ↑/↓ step; the
  mixed "–" state with a dashed frame; errors on hover), `select`.
- **Menus**: `menu_popup`, `context_menu`, `popover`, `panel` (inline),
  `MenuItem` (checkmark column, icon column, right-aligned shortcut,
  disabled, destructive), `submenu`, `separator`, `section_header`.
  A menu is as wide as its longest row: in the popup's sizing pass a
  row asks only for its content (separators and headers for nothing),
  so no menu takes egui's 600 pt default area width; `size::MENU_W` /
  `MENU_W_SM` are minimums. Menus of buttons at the window's right edge
  open right-aligned (`RectAlign::BOTTOM_END`).
- **Tooltips**: plain and rich (title, shortcut, description).
- **Surfaces**: `Card`, `section`, `collapsible`, `inset`, `form_row`,
  `divider`, `header`.
- **Collections**: `ListRow` (leading icon, swatch or painted thumbnail,
  secondary line, meta, tag; strong or soft selection), `group_header`,
  `Table` (fixed and fill columns, sortable header, zebra, selection,
  scrolling with a fixed header, only visible rows built), `TreeRow`
  (indent, expand chevron, icon, note, state toggles such as visible and
  locked).
- **Navigation**: attached tabs, underline tabs, document tabs (close,
  drag to reorder, add), `toolbar`, `separator`, `ToolButton`, the
  floating toolbar pill, `ribbon_group`, `ribbon_column`, `RibbonTool`
  (large and small).
- **Dialogs**: `Dialog` / `modal` (egui `Modal` with the dialog frame
  and scrim, in the safe area; Escape closes, Enter confirms),
  `sheet`, `surface` (inline), `heading`, `footer`, `wizard` and
  `wizard_steps`, `status_block`.
- **Feedback**: `notice`, `Toasts`, `empty_state`, `progress`
  (determinate and indeterminate), `spinner`.
- **Layout**: `Splitter` (resizable two panes, remembered per id),
  `scroll::edges` (the cue at scrolled edges), `scroll::side_fades`
  (the same for horizontal areas, the ribbon body).
- **Windows** (`window`): the safe area, `ToolWindow`, `Floating` (see
  "Windows and dialogs").
- **Docking pieces**: `pane` (card with the tab strip), `drop_cross`
  and `cross_zone`, `drop_preview`, `pane_button`; `tabs::doc_tab_strip`
  (document tabs with their responses, a tab dragged off the strip is
  `Detach`ed), `splitter::divider` (a divider whose fraction the caller
  keeps), `toolbar::overlay` (the floating pill over a viewport,
  horizontal or vertical, taking the pointer only inside its bounds).
  The app's viewports use them (`viewports.rs`, `viewport_ui.rs`): tab
  groups split left/right/top/bottom, tabs dragged to reorder, onto a
  group's drop cross, strip or edge to dock or split, anywhere else to
  float in a `ToolWindow` (`bare`: no padding); "Dock" brings a floating
  view back. The layout is kept in the app settings.

### Gallery

`gallery::Gallery` draws nine pages (foundations, icons, buttons and
toggles, inputs, menus and overlays, lists / tables / trees, navigation,
feedback, surfaces and docking) with the light and the dark theme side by
side; rows of widgets are repeated in each state (rest, hover, pressed,
focus, disabled — forced with `state::with_forced`). Its language can
follow the app or be English or German. It is shown in the app under
**Help ▸ UI Gallery** (`Command::UiGallery`) and by
`cargo run --example gallery`;
`OPENCAD_UI_SHOT_DIR=<dir>` makes the example save every page in both
languages as PNG (long pages in several parts) and quit;
`OPENCAD_UI_SIZE=<w>x<h>` sets the window size, and every page whose
content runs past its column is reported (`Gallery::overflow`). Pages are
checked at 1280 and 1600 pt wide. The gallery is
what reviews the foundation; a change to a token or widget should be
checked there in both themes.

### Windows and dialogs

Until now every window was a plain `egui::Window` drawn into the main
window's canvas. On macOS the app uses a full-size content view, so such
a window could be placed into the title-bar strip under the traffic
lights, where the native drag area took its title bar and it could no
longer be moved; and egui's scroll-edge fade painted a grey haze over the
bottom of its content and frame, next to the resize grip. The foundation
now has one abstraction per kind of window (`opencad_ui::window`,
`widgets::dialog`), and every window of the app uses one of them.

- **Safe area.** `window::set_safe_area(ctx, rect)` is called by the app
  once per frame with the content rect below the title bar, menu and
  ribbon (on macOS the strip with the traffic lights);
  `window::safe_area(ctx)` returns it (per viewport; the whole content
  rect if nothing was set). Everything that floats in the canvas — tool
  windows on the web, floating panels, modal dialogs, sheets — is
  constrained to it, so a window's header can never go under the title
  bar. `window::clamp_rect` and `centred_in` are the pure rules (tested).
- **Tool windows** (`window::ToolWindow`): long-lived windows the user
  keeps open next to the work — UI Gallery, Settings, Keyboard Shortcuts,
  AI connection. Natively they are **real OS windows** (egui immediate
  viewports with a `ViewportBuilder`: title, inner and minimum size,
  resizable): moved, resized, minimised and closed by the OS, with their
  own title bar and traffic lights; ⌘W / Ctrl+W closes them too. The
  builder is made once when the window opens (a builder that changed
  every frame would move the window back); size and position are stored
  per window key in egui's persisted memory (eframe saves it with the app
  settings) and restored if the position is still on the screen; the
  first time the window is centred over the main window. Immediate
  viewports were chosen over deferred ones because the content borrows
  the app state (`&mut self`); the cost is that both windows repaint
  together. On the **web** (no multiple viewports, `ctx.embed_viewports()`)
  and with backends without them, a tool window is a floating in-canvas
  window (`Floating`) in the safe area: dragged by its header, resizable,
  closed with its × button, with the overlay radius, the dialog shadow
  and a hairline ring.
- **Settings** is a tool window, not a modal dialog: on macOS (and in
  Windows 11's settings) preferences are a separate, non-modal window
  whose changes apply at once — there is nothing to confirm or cancel,
  so the "Done" button is gone, and the user can keep it open while
  trying a theme or language.
- **Modal dialogs** (`dialog::Dialog`, `dialog::modal`): About, Getting
  Started, Export, the Allplan import report and the AI set-up wizard.
  egui's `Modal` with the dialog frame and the `scrim` backdrop, centred
  in the safe area. `ModalOutput` reports `close` (Escape, or a click on
  the backdrop unless `backdrop_closes(false)`, as for the wizard, which
  would lose its steps) and `confirm` (Enter while no widget has the
  focus: the default button's action). The app's `dialogs::footer` puts
  the tertiary Cancel at the left and the primary button at the right.
- **Floating panels** (`window::Floating`): small non-modal in-canvas
  windows that belong to the canvas — the filter assistant (placed near
  the top right of the safe area, draggable) and the assistant's
  permission prompts (anchored to the bottom right). The prompts stay
  non-modal as ADR 0013 decided: an assistant may ask several things at
  once, the prompts stack and time out, and the user keeps working; a
  modal would block the canvas for a question the user can ignore.
- **Scroll edges.** egui's built-in edge fade is switched off in both
  styles (`spacing.scroll.fade.strength = 0`): it took its colour from
  the nearest frame up the `Ui` stack (a card, the window chrome) and ran
  to the bottom of the area, over the resize grip. `widgets::scroll::edges`
  replaces it, called after `ScrollArea::show`: the cue is clipped to the
  viewport, leaves the scroll bar free, appears only towards content that
  is scrolled out of view (top and bottom, ramping in over the first
  14 pt), and is either a 14 pt fade into the colour of the area's own
  background (`Edge::Fade`) or a hairline with a 4 pt shadow, as on macOS
  (`Edge::Hairline`, for areas over mixed surfaces such as the gallery's
  light and dark columns).

Caveats: multiple native windows need a backend with multi-viewport
support (eframe's wgpu and glow on Windows, macOS and X11 have it; with
wgpu each window has its own surface on the shared device). On Wayland
the compositor places new windows and ignores the stored position. Key
equivalents of the native menu bar (macOS) are handled by the app for
the main window, so a shortcut typed in a tool window may act on the
project. On macOS the app's Window menu is NSApp's windows menu, so
AppKit lists the tool windows there by title. eframe takes screenshots
(`ViewportCommand::Screenshot`) of the root and deferred viewports
only, so the script step `screenshot` shows the main window; tool
windows are captured with the OS (`screencapture -l <window id>`).

### Language

The language state, `Message` and `messages!` moved into `opencad-ui`
(`i18n`), so foundation widgets ("Clear", "Close tab") and the app share
one language; the app's `i18n` module re-exports them and adds
`system_lang` (OS or browser locale). Every user-visible string of the
crate, the gallery included, is in a `messages!` table with English and
German. Sample names in the gallery stand for model data and are not
translated (ADR 0011).

### Ribbon: task area and tabs

The ribbon's tab row holds two levels of choice: the task area
(Architecture, Draft, Engineering, Surroundings) picks the set of tabs,
the tabs pick the tools. Drawn as a segmented control right beside the
underline tabs, in the same size and weight, both read as one row of
equal options. The task area is therefore a framed menu button (area
icon, name, chevron; accessible name "Task area: …") followed by a
divider, as in Allplan's and Fusion's workspace switch, and only the
tabs are frameless underline tabs. The area changes rarely, so the menu's
extra click costs little, and the narrower control leaves the tabs more
room. The pinned "General" group at the left of the body sits on a chrome
tint with a full-height border, apart from the tab's groups, which
scroll and fade at the edge that has more.

### Rules for app code

1. **No literal colours or sizes** in screens: colours come from
   `opencad_ui::colors(ui)` (or `Colors::of(dark)`), sizes from `space`,
   `radius`, `size`, text from `Type`. Drawing colours of the model (pens,
   materials, hatches) are data, not tokens.
2. **Use the widgets.** A screen composes foundation widgets; if one is
   missing or needs a variant, add it to `opencad-ui` (with a gallery
   entry) instead of styling it in the screen.
3. **Icons** are `opencad_ui::Icon`; new icons are SVG path strings in
   `icon_data.rs`, on the 16 or 24 grid, in the style of the set.
4. **States and accessibility**: a custom widget uses `state::state` for
   its look, `state::focus_ring` when focused and `state::a11y` for its
   role and label.
5. **Strings** go through `messages!` tables (ADR 0011).
6. **Both themes**: check new UI in the gallery or the app in light and
   dark.
7. **Windows**: no plain `egui::Window` in screens; a window is a
   `ToolWindow`, a `Dialog` or a `Floating` panel, and anything else that
   floats in the canvas clamps to `window::safe_area`.
8. **Ids**: a widget derives the ids of its parts from its own response
   (an automatic id) or an explicit salt, never from `ui.scope_id()`
   alone: sibling `Ui`s share a scope, so two instances side by side
   would share state and the last one drawn would take every click.

### Migration

Screens are not migrated yet; that is the next step, after the gallery
review. For now there is one source of tokens: `theme::PanelColours` is
built from `opencad_ui::Colors`, `ui_kit`'s size constants are aliases of
the foundation's tokens, `ui_kit::chip`, `segmented` / `segmented_tips`
and `toggle` are the foundation's widgets, `style::over` is the
foundation's, and the app's fonts come from `opencad_ui::fonts` (plus the
plan font). Visible effects: card body text is 12 pt (was 12.5), header
titles 14 (was 15), menu-row and segment corners 5 (were 4), Linux and
the web use Inter, Windows 11 Segoe UI Variable. `style.rs` (the native
egui look of the remaining egui widgets) and `icons.rs` (the hand-drawn
icons) stay until their screens move; the order of the migration is
palette and component editors, catalogue and type picker, ribbon and
status bar, panels, dialogs, chat and wizard, 3D panel.

## Consequences

- One place decides the look; a screen becomes composition.
- The foundation is a few thousand lines to own and test (keyboard, IME,
  AccessKit coverage of custom widgets). egui's limits remain: no
  backdrop blur (menus are opaque), no OpenType features (tabular figures
  by painting), single-pass layout (widgets size themselves; right-to-left
  containers reverse order), egui's text rasteriser.
- Popups opened from a `Ui` in another theme (the gallery's dark column)
  get that theme through a style modifier; tooltips use the context's
  theme.
- The web build grows by the bundled font (≈ 0.9 MB raw).

## Open questions

- Snapshot tests of the gallery (egui_kittest) once a renderer for CI is
  agreed on.
- Real docking (egui_tiles or our own) on top of the docking pieces.
- Whether `style.rs`'s platform palettes (macOS / Windows look of plain
  egui widgets) stay or give way entirely to the tokens.

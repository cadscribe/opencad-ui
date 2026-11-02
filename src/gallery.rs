//! The UI gallery: every token and widget, the light and the dark theme
//! side by side, with their states; in English or German. Shown by the
//! app (Help ▸ UI Gallery) and by `cargo run -p opencad-ui --example
//! gallery`.
//!
//! Material, type and room names in the samples stand for model data
//! (never translated, ADR 0011); everything else comes from the message
//! table below.

use egui::{
    Align, Color32, Frame, Layout, Margin, Rect, ScrollArea, Sense, Stroke, StrokeKind, Ui, Vec2,
    pos2, vec2,
};

use crate::i18n::{Lang, lang, tr, trf, with_lang};
use crate::icon::{self, Icon};
use crate::length::Unit;
use crate::state::{Forced, with_forced};
use crate::text::{self, Type};
use crate::theme::{colors, scope_theme};
use crate::tokens::{Colors, Elevation, Tone, motion, radius, space};
use crate::widgets::button::{Button, ButtonSize, IconButton, Variant};
use crate::widgets::dialog;
use crate::widgets::dock::{self, DropZone};
use crate::widgets::feedback::{self, Toasts};
use crate::widgets::field::{self, Affix, NumberField, TextField};
use crate::widgets::list::{self, ListRow, Selection};
use crate::widgets::menu::{self, MenuItem};
use crate::widgets::scroll;
use crate::widgets::segmented::{Segment, Segmented};
use crate::widgets::splitter::{Axis, Splitter};
use crate::widgets::table::{self, Column, Sort, Table, Width};
use crate::widgets::tabs::{self, DocTab, TabEvent};
use crate::widgets::toggle::{self, Check};
use crate::widgets::toolbar::{self, RibbonTool, ToolButton};
use crate::widgets::tooltip;
use crate::widgets::tree::{Toggle, TreeRow};
use crate::widgets::{card, chip};

crate::messages! {
    enum Msg {
        Title: "UI Gallery", "UI-Galerie";
        Subtitle: "Tokens and widgets of opencad-ui (ADR 0019)", "Tokens und Widgets von opencad-ui (ADR 0019)";
        Light: "Light", "Hell";
        Dark: "Dark", "Dunkel";
        Language: "Language", "Sprache";
        AppLanguage: "App", "App";
        PageFoundations: "Foundations", "Grundlagen";
        PageIcons: "Icons", "Symbole";
        PageButtons: "Buttons & toggles", "Schaltflächen & Schalter";
        PageInputs: "Inputs", "Eingaben";
        PageMenus: "Menus & overlays", "Menüs & Overlays";
        PageCollections: "Lists, tables, trees", "Listen, Tabellen, Bäume";
        PageNavigation: "Navigation", "Navigation";
        PageFeedback: "Feedback", "Rückmeldung";
        PageLayout: "Surfaces & docking", "Flächen & Andocken";
        StateRest: "Rest", "Normal";
        StateHover: "Hover", "Hover";
        StatePressed: "Pressed", "Gedrückt";
        StateFocus: "Focus", "Fokus";
        StateDisabled: "Disabled", "Deaktiviert";
        Colours: "Colours", "Farben";
        Tones: "Status tones", "Statustöne";
        Typography: "Typography", "Typografie";
        FontSource: "UI font: {0}", "UI-Schrift: {0}";
        TypeSample: "Wall 10.240 m · Layers", "Wand 10,240 m · Schichten";
        Tabular: "Tabular figures", "Tabellenziffern";
        Proportional: "Proportional", "Proportional";
        Spacing: "Spacing", "Abstände";
        Radii: "Radii", "Radien";
        Elevation: "Elevation", "Erhebung";
        Motion: "Motion", "Bewegung";
        MotionText: "fast {0} s · normal {1} s · slow {2} s · ease-out", "schnell {0} s · normal {1} s · langsam {2} s · ease-out";
        IconSizes: "Sizes 13 / 16 / 20 / 26 / 40", "Größen 13 / 16 / 20 / 26 / 40";
        AllIcons: "All icons ({0})", "Alle Symbole ({0})";
        Buttons: "Buttons", "Schaltflächen";
        Primary: "Primary", "Primär";
        Secondary: "Secondary", "Sekundär";
        Tertiary: "Tertiary", "Tertiär";
        Destructive: "Destructive", "Destruktiv";
        Sizes: "Sizes", "Größen";
        Small: "Small", "Klein";
        Medium: "Medium", "Mittel";
        Large: "Large", "Groß";
        Save: "Save", "Speichern";
        Cancel: "Cancel", "Abbrechen";
        Back: "Back", "Zurück";
        Next: "Next", "Weiter";
        Delete: "Delete", "Löschen";
        ResetToType: "Reset to type", "Auf Typ zurücksetzen";
        AllLinked: "All linked (6)", "Alle verknüpften (6)";
        AddLayer: "+ Layer", "+ Schicht";
        IconButtons: "Icon buttons", "Symbolschaltflächen";
        Toggles: "Switches, checkboxes, radios", "Schalter, Kontrollkästchen, Optionen";
        ShowRooms: "Show rooms in 3D", "Räume in 3D zeigen";
        StartAtLogin: "Start at login", "Bei der Anmeldung starten";
        LinkToType: "Linked to type", "Mit Typ verknüpft";
        Mixed: "Mixed", "Gemischt";
        Segmented: "Segmented controls", "Segmentierte Steuerelemente";
        Axis: "Axis", "Achse";
        Left: "Left", "Links";
        Centre: "Centre", "Mitte";
        Right: "Right", "Rechts";
        OneView: "One view", "Ein Fenster";
        TwoViews: "Two views", "Zwei Fenster";
        ThreeViews: "Three views", "Drei Fenster";
        Chips: "Chips and tags", "Chips und Tags";
        MatchesType: "matches type", "entspricht Typ";
        Modified: "modified", "geändert";
        Info: "imported", "importiert";
        Invalid: "invalid", "ungültig";
        Neutral: "passive", "passiv";
        Exterior: "Exterior wall", "Außenwand";
        Interior: "Interior wall", "Innenwand";
        Basement: "Basement", "Keller";
        Drywall: "Drywall", "Trockenbau";
        SlabAbove: "Structural slab above", "Rohdecke darüber";
        TextFields: "Text fields", "Textfelder";
        Name: "Name", "Name";
        NameHint: "Name of the drawing file", "Name des Teilbilds";
        Url: "Address", "Adresse";
        ErrorEmpty: "A name is required", "Ein Name ist erforderlich";
        SearchFields: "Search fields", "Suchfelder";
        SearchType: "Search types", "Typ suchen";
        SearchTool: "Search tools …  ⌘K", "Werkzeug suchen …  ⌘K";
        NumberFields: "Number and length fields", "Zahlen- und Längenfelder";
        NumberHelp: "Type 1 m + 24 cm, (2,5 + 1) × 2, ↑/↓ to step; Esc cancels", "Eingabe z. B. 1 m + 24 cm, (2,5 + 1) × 2, ↑/↓ schrittweise; Esc bricht ab";
        Length: "Length", "Länge";
        Thickness: "Thickness", "Dicke";
        Angle: "Angle", "Winkel";
        Distance: "Distance", "Abstand";
        Selects: "Selects", "Auswahlfelder";
        Scale: "Scale", "Maßstab";
        Form: "Form in a card", "Formular in einer Karte";
        PositionHeights: "Position & heights", "Lage & Höhen";
        TopEdge: "Top", "Oberkante";
        BottomEdge: "Bottom", "Unterkante";
        Menu: "Menu", "Menü";
        Undo: "Undo: layer thickness", "Rückgängig: Schichtdicke";
        Redo: "Redo", "Wiederholen";
        Cut: "Cut", "Ausschneiden";
        Copy: "Copy", "Kopieren";
        Paste: "Paste", "Einfügen";
        TransferFormat: "Transfer format …", "Format übertragen …";
        AssignType: "Assign type …", "Typ zuweisen …";
        Arrange: "Arrange", "Anordnen";
        ToFront: "Bring to front", "Nach vorne";
        ToBack: "Send to back", "Nach hinten";
        OpenMenu: "Open menu", "Menü öffnen";
        ContextHint: "Right-click here for a context menu", "Rechtsklick hier für ein Kontextmenü";
        Popover: "Popover", "Popover";
        Tooltips: "Tooltips", "Tooltips";
        WallTool: "Wall", "Wand";
        WallToolText: "Draws walls along an axis; the layers come from the type.", "Zeichnet Wände entlang einer Achse; die Schichten kommen vom Typ.";
        Dialog: "Dialog", "Dialog";
        DeleteTitle: "Delete drawing file 200?", "Teilbild 200 löschen?";
        DeleteText: "Its 412 elements will be removed. You can undo this.", "Seine 412 Elemente werden entfernt. Das lässt sich rückgängig machen.";
        OpenDialog: "Open dialog", "Dialog öffnen";
        Wizard: "Wizard", "Assistent";
        SetUpAi: "Set up AI", "KI einrichten";
        StepGoal: "Goal", "Ziel";
        StepStart: "Start ai-connector", "ai-connector starten";
        StepChoose: "Choose AI", "KI auswählen";
        StepTest: "Test", "Testen";
        WizardNote: "Everything stays on this computer.", "Alles bleibt auf diesem Rechner.";
        WizardText: "ai-connector provides the AI models. It runs in the background.", "ai-connector stellt die KI-Modelle bereit. Er läuft im Hintergrund.";
        Found: "Found:", "Gefunden:";
        FoundText: "ai-connector 0.9.4", "ai-connector 0.9.4";
        Waiting: "Waiting for ai-connector …", "Warte auf ai-connector …";
        WaitingText: "Continues as soon as it runs.", "Sobald er läuft, geht es automatisch weiter.";
        Help: "Help", "Hilfe";
        Lists: "List rows", "Listenzeilen";
        Office: "Office", "Büro";
        Supplied: "Supplied", "Mitgeliefert";
        Table: "Table (sortable)", "Tabelle (sortierbar)";
        Room: "Room", "Raum";
        Storey: "Storey", "Geschoss";
        Area: "Area m²", "Fläche m²";
        Height: "Height m", "Höhe m";
        Tree: "Tree", "Baum";
        Visible: "Visible", "Sichtbar";
        Locked: "Locked", "Gesperrt";
        AttachedTabs: "Attached tabs", "Angehängte Tabs";
        Properties: "Properties", "Eigenschaften";
        Library: "Library", "Bibliothek";
        RoomBook: "Room book", "Raumbuch";
        UnderlineTabs: "Underline tabs (ribbon)", "Unterstrichene Tabs (Ribbon)";
        Components: "Components", "Bauteile";
        Finishing: "Finishing", "Ausbau";
        Modelling: "Modelling", "Modellieren";
        Annotate: "Annotate", "Beschriften";
        DocTabs: "Document tabs (drag to reorder)", "Dokument-Tabs (zum Sortieren ziehen)";
        PlanEg: "Plan · GF", "Grundriss · EG";
        SectionAA: "Section A–A", "Schnitt A–A";
        View3dTab: "3D · whole building", "3D · ganzes Gebäude";
        Toolbars: "Toolbars", "Werkzeugleisten";
        Floating: "Floating toolbar", "Schwebende Werkzeugleiste";
        Perspective: "Perspective", "Perspektive";
        Edges: "Edges", "Kanten";
        Rooms: "Rooms", "Räume";
        SectionDd: "Section", "Schnitt";
        Content: "Content", "Inhalt";
        ZoomAll: "Zoom all", "Alles zeigen";
        Ribbon: "Ribbon groups", "Ribbon-Gruppen";
        WallsColumns: "Walls & columns", "Wände & Stützen";
        Column_: "Column", "Stütze";
        Beam: "Beam", "Unterzug";
        Pick: "Pick properties", "Eigenschaften übernehmen";
        Openings: "Openings", "Öffnungen";
        Window_: "Window", "Fenster";
        Door: "Door", "Tür";
        LiftSlide: "Lift-and-slide door", "Hebeschiebetür";
        Opening: "Opening", "Öffnung";
        Areas: "Task areas", "Aufgabenbereiche";
        Architecture: "Architecture", "Architektur";
        Draft: "Draft", "Entwurf";
        Engineering: "Engineering", "Ingenieurbau";
        Notices: "Notices", "Hinweise";
        NoticeInfo: "The project was imported from Allplan.", "Das Projekt wurde aus Allplan importiert.";
        NoticeSuccess: "Saved in 120 ms.", "In 120 ms gespeichert.";
        NoticeWarning: "Two walls overlap in the GF.", "Zwei Wände überschneiden sich im EG.";
        NoticeDanger: "The file could not be read.", "Die Datei konnte nicht gelesen werden.";
        Toasts: "Toasts", "Toasts";
        ShowToast: "Show a toast", "Toast zeigen";
        ToastText: "Exported plan.pdf", "plan.pdf exportiert";
        Status: "Status block", "Statusblock";
        Empty: "Empty state", "Leerer Zustand";
        EmptyTitle: "No drawing files yet", "Noch keine Teilbilder";
        EmptyText: "Create a drawing file to start modelling.", "Lege ein Teilbild an, um zu modellieren.";
        EmptyAction: "New drawing file", "Neues Teilbild";
        Progress: "Progress and spinners", "Fortschritt und Ladeanzeigen";
        Cards: "Cards and sections", "Karten und Abschnitte";
        Type_: "Type", "Typ";
        Layers: "Layers", "Schichten";
        Physics: "Building physics", "Bauphysik";
        Collapsible: "Collapsible (click the title)", "Einklappbar (Titel anklicken)";
        Inset: "Inset", "Einsatz";
        Splitter: "Splitter", "Teiler";
        PaneA: "Plan", "Grundriss";
        PaneB: "Palette", "Palette";
        Docking: "Docking pieces", "Andock-Elemente";
        Undock: "Undock", "Abdocken";
        Maximise: "Maximise", "Maximieren";
        Header: "Header card", "Kopfkarte";
        WallSubtitle: "Drawing file 200 · GF model · layer AR_WALLS", "Teilbild 200 · EG Modell · Layer AR_WALLS";
        Total: "Total", "Gesamt";
        Inside: "inside", "innen";
        Outside: "outside", "außen";
    }
}

/// Sample model data (names are never translated).
const MATERIALS: [(&str, u32, f64, bool); 4] = [
    ("Kalkgipsputz", 0xECE7DC, 15.0, false),
    ("Kalksandstein", 0xDCCDB2, 175.0, true),
    ("EPS 032 (WDVS)", 0xF3E07A, 140.0, false),
    ("Oberputz", 0xE2E2E6, 5.0, false),
];

const ROOMS: [(&str, &str, f64, f64); 8] = [
    ("Wohnen", "EG", 32.41, 2.60),
    ("Küche", "EG", 11.80, 2.60),
    ("Bad", "OG", 7.92, 2.45),
    ("Schlafen", "OG", 16.25, 2.45),
    ("Kind 1", "OG", 12.04, 2.45),
    ("Flur", "EG", 8.66, 2.60),
    ("Technik", "KG", 9.10, 2.20),
    ("Hobby", "KG", 18.37, 2.20),
];

/// The gallery's pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    Foundations,
    Icons,
    Buttons,
    Inputs,
    Menus,
    Collections,
    Navigation,
    Feedback,
    Layout,
}

impl Page {
    pub const ALL: [Page; 9] = [
        Page::Foundations,
        Page::Icons,
        Page::Buttons,
        Page::Inputs,
        Page::Menus,
        Page::Collections,
        Page::Navigation,
        Page::Feedback,
        Page::Layout,
    ];

    fn msg(self) -> Msg {
        match self {
            Page::Foundations => Msg::PageFoundations,
            Page::Icons => Msg::PageIcons,
            Page::Buttons => Msg::PageButtons,
            Page::Inputs => Msg::PageInputs,
            Page::Menus => Msg::PageMenus,
            Page::Collections => Msg::PageCollections,
            Page::Navigation => Msg::PageNavigation,
            Page::Feedback => Msg::PageFeedback,
            Page::Layout => Msg::PageLayout,
        }
    }

    /// A short key (screenshot file names).
    pub fn key(self) -> &'static str {
        match self {
            Page::Foundations => "foundations",
            Page::Icons => "icons",
            Page::Buttons => "buttons",
            Page::Inputs => "inputs",
            Page::Menus => "menus",
            Page::Collections => "collections",
            Page::Navigation => "navigation",
            Page::Feedback => "feedback",
            Page::Layout => "layout",
        }
    }
}

/// The values the sample widgets edit (shared by both columns).
#[derive(Debug)]
struct Values {
    switch: bool,
    switch2: bool,
    check: bool,
    check_tri: Check,
    radio: u8,
    axis: u8,
    views: u8,
    area: u8,
    filters: [bool; 4],
    name: String,
    name_err: String,
    url: String,
    search: String,
    search2: String,
    length: f64,
    thickness: f64,
    angle: f64,
    distance: f64,
    scale: u32,
    sort: Sort,
    selected_room: Option<usize>,
    tree_open: [bool; 3],
    visible: [bool; 6],
    locked: [bool; 6],
    palette_tab: usize,
    ribbon_tab: usize,
    doc_tabs: Vec<u8>,
    doc_selected: usize,
    show_dialog: bool,
    toasts: Toasts,
    progress: f32,
}

impl Default for Values {
    fn default() -> Self {
        Values {
            switch: true,
            switch2: false,
            check: true,
            check_tri: Check::Mixed,
            radio: 1,
            axis: 0,
            views: 1,
            area: 0,
            filters: [true, false, false, false],
            name: "EG Modell".into(),
            name_err: String::new(),
            url: "localhost".into(),
            search: "ks".into(),
            search2: String::new(),
            length: 10_240.0,
            thickness: 175.0,
            angle: 0.0,
            distance: 0.0,
            scale: 100,
            sort: Sort {
                column: 0,
                ascending: true,
            },
            selected_room: Some(2),
            tree_open: [true, true, false],
            visible: [true, true, false, true, true, true],
            locked: [false, false, false, true, false, false],
            palette_tab: 0,
            ribbon_tab: 0,
            doc_tabs: vec![0, 1, 2],
            doc_selected: 0,
            show_dialog: false,
            toasts: Toasts::default(),
            progress: 0.35,
        }
    }
}

/// The gallery window's state.
#[derive(Debug)]
pub struct Gallery {
    pub page: Page,
    /// The language of the gallery (`None`: the app's).
    pub lang: Option<Lang>,
    /// Scrolls the page to this offset on the next frame (screenshots).
    pub scroll_to: Option<f32>,
    /// The height of the page content and of its viewport, as last shown.
    pub content_height: f32,
    pub view_height: f32,
    /// How far the widest column's content ran past its width (points,
    /// last frame); 0 when everything fits.
    pub overflow: f32,
    values: Values,
}

impl Default for Gallery {
    fn default() -> Self {
        Gallery {
            page: Page::Foundations,
            lang: None,
            scroll_to: None,
            content_height: 0.0,
            view_height: 0.0,
            overflow: 0.0,
            values: Values::default(),
        }
    }
}

impl Gallery {
    /// The window title in the current language.
    pub fn title() -> &'static str {
        tr(Msg::Title)
    }

    /// Draws the gallery into `ui` (a window or a central panel).
    pub fn ui(&mut self, ui: &mut Ui) {
        let lang = self.lang.unwrap_or_else(lang);
        with_lang(lang, || self.ui_in_lang(ui));
    }

    fn ui_in_lang(&mut self, ui: &mut Ui) {
        let c = colors(ui);
        // Header: title, page picker, language.
        ui.horizontal(|ui| {
            ui.label(text::rich(tr(Msg::Title), Type::Title, c.text));
            ui.label(text::rich(tr(Msg::Subtitle), Type::Body, c.text_secondary));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let mut l = self.lang;
                let segs = [
                    Segment::text(None, tr(Msg::AppLanguage)),
                    Segment::text(Some(Lang::En), "EN"),
                    Segment::text(Some(Lang::De), "DE"),
                ];
                ui.allocate_ui(vec2(150.0, 28.0), |ui| {
                    if Segmented::new(&mut l, &segs).show(ui).changed() {
                        self.lang = l;
                    }
                });
                ui.label(text::rich(tr(Msg::Language), Type::Body, c.text_secondary));
            });
        });
        ui.add_space(space::SM);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = vec2(space::SM, space::SM);
            for p in Page::ALL {
                if chip::filter(ui, tr(p.msg()), self.page == p, false).clicked {
                    self.page = p;
                }
            }
        });
        ui.add_space(space::MD);
        let page = self.page;
        let v = &mut self.values;
        let mut area = ScrollArea::vertical()
            .id_salt(("gallery_scroll", page.key()))
            .auto_shrink([false, false]);
        if let Some(y) = self.scroll_to.take() {
            area = area.vertical_scroll_offset(y);
        }
        let mut overflow = 0.0f32;
        let out = area.show(ui, |ui| {
            ui.columns(2, |cols| {
                for (i, col) in cols.iter_mut().enumerate() {
                    let dark = i == 1;
                    // Each column in its own id scope: the columns are
                    // siblings and would otherwise share the ids of
                    // scope-derived widget state.
                    col.push_id(("gallery_column", i), |col| {
                        scope_theme(col, dark, |ui| {
                            let c = colors(ui);
                            Frame::new()
                                .fill(c.surface)
                                .corner_radius(radius::OVERLAY)
                                .stroke(Stroke::new(1.0, c.border))
                                .inner_margin(Margin::same(space::X3L as i8))
                                .show(ui, |ui| {
                                    let allotted = ui.available_width();
                                    ui.set_width(allotted);
                                    ui.spacing_mut().item_spacing.y = space::XL;
                                    ui.label(text::rich(
                                        tr(if dark { Msg::Dark } else { Msg::Light }),
                                        Type::Overline,
                                        c.text_tertiary,
                                    ));
                                    page_ui(ui, page, v, i);
                                    // Content wider than the column is a layout bug.
                                    overflow = overflow.max(ui.min_rect().width() - allotted);
                                });
                        });
                    });
                }
            });
        });
        // The page runs over cards in both themes: a hairline, not a fade.
        scroll::edges(ui, &out, scroll::Edge::Hairline);
        self.overflow = overflow.max(0.0);
        self.content_height = out.content_size.y;
        self.view_height = out.inner_rect.height();
        let ctx = ui.ctx().clone();
        self.values.toasts.show(&ctx);
        if self.values.show_dialog {
            let out = dialog::modal(&ctx, "gallery_dialog", 380.0, |ui| {
                dialog::heading(ui, tr(Msg::DeleteTitle), tr(Msg::DeleteText));
                let (mut cancel, mut delete) = (false, false);
                dialog::footer(
                    ui,
                    |ui| {
                        cancel = ui.add(Button::tertiary(tr(Msg::Cancel))).clicked();
                    },
                    |ui| {
                        delete = ui.add(Button::destructive(tr(Msg::Delete))).clicked();
                    },
                );
                if cancel || delete {
                    self.values.show_dialog = false;
                }
            });
            if out.close || out.confirm {
                self.values.show_dialog = false;
            }
        }
    }
}

fn page_ui(ui: &mut Ui, page: Page, v: &mut Values, col: usize) {
    match page {
        Page::Foundations => foundations(ui),
        Page::Icons => icons(ui),
        Page::Buttons => buttons(ui, v),
        Page::Inputs => inputs(ui, v, col),
        Page::Menus => menus(ui, v),
        Page::Collections => collections(ui, v, col),
        Page::Navigation => navigation(ui, v, col),
        Page::Feedback => feedback_page(ui, v),
        Page::Layout => layout_page(ui, v, col),
    }
}

fn heading(ui: &mut Ui, m: Msg) {
    let c = colors(ui);
    ui.label(text::rich(tr(m), Type::Title, c.text));
}

fn small(ui: &mut Ui, s: &str) {
    let c = colors(ui);
    ui.label(text::rich(s, Type::Caption, c.text_tertiary));
}

fn state_msg(f: Forced) -> Msg {
    match f {
        Forced::Rest => Msg::StateRest,
        Forced::Hover => Msg::StateHover,
        Forced::Pressed => Msg::StatePressed,
        Forced::Focus => Msg::StateFocus,
        Forced::Disabled => Msg::StateDisabled,
    }
}

/// A row per state: the label column and `add` drawn in that state.
fn states(ui: &mut Ui, label_w: f32, list: &[Forced], mut add: impl FnMut(&mut Ui, Forced)) {
    let c = colors(ui);
    for &f in list {
        ui.horizontal(|ui| {
            let (r, _) = ui.allocate_exact_size(vec2(label_w, 28.0), Sense::hover());
            let g = text::galley(
                ui.painter(),
                tr(state_msg(f)),
                Type::Caption,
                c.text_tertiary,
            );
            text::paint_in(ui.painter(), r, g, Align::Min, c.text_tertiary);
            with_forced(ui, f, |ui| add(ui, f));
        });
    }
}

// ------------------------------------------------------------ foundations

type Pick = fn(&Colors) -> Color32;

const COLOR_TOKENS: [(&str, Pick); 32] = [
    ("bg", |c| c.bg),
    ("chrome", |c| c.chrome),
    ("surface", |c| c.surface),
    ("surface_raised", |c| c.surface_raised),
    ("surface_sunken", |c| c.surface_sunken),
    ("field", |c| c.field),
    ("track", |c| c.track),
    ("thumb", |c| c.thumb),
    ("border", |c| c.border),
    ("border_strong", |c| c.border_strong),
    ("border_mixed", |c| c.border_mixed),
    ("divider", |c| c.divider),
    ("text", |c| c.text),
    ("text_secondary", |c| c.text_secondary),
    ("text_tertiary", |c| c.text_tertiary),
    ("text_disabled", |c| c.text_disabled),
    ("icon", |c| c.icon),
    ("accent", |c| c.accent),
    ("accent_fill", |c| c.accent_fill),
    ("accent_fill_hover", |c| c.accent_fill_hover),
    ("on_accent", |c| c.on_accent),
    ("accent_tint", |c| c.accent_tint),
    ("accent_text", |c| c.accent_text),
    ("highlight", |c| c.highlight),
    ("selection", |c| c.selection),
    ("focus", |c| c.focus),
    ("danger_fill", |c| c.danger_fill),
    ("glass", |c| c.glass),
    ("scrim", |c| c.scrim),
    ("code_bg", |c| c.code_bg),
    ("assistant", |c| c.assistant),
    ("ring", |c| c.ring),
];

fn swatch_cell(ui: &mut Ui, name: &str, col: Color32, w: f32) {
    let c = colors(ui);
    let (r, _) = ui.allocate_exact_size(vec2(w, 22.0), Sense::hover());
    let sw = Rect::from_min_size(r.min + vec2(0.0, 2.0), vec2(28.0, 18.0));
    // A checker under translucent colours.
    if col.a() < 255 {
        ui.painter().rect_filled(sw, radius::XS, Color32::WHITE);
        ui.painter().rect_filled(
            Rect::from_min_size(sw.min, sw.size() / 2.0),
            0.0,
            Color32::from_gray(200),
        );
        ui.painter().rect_filled(
            Rect::from_min_size(sw.center(), sw.size() / 2.0),
            0.0,
            Color32::from_gray(200),
        );
    }
    ui.painter().rect(
        sw,
        radius::XS,
        col,
        Stroke::new(1.0, c.border),
        StrokeKind::Inside,
    );
    let g = text::truncated(
        ui.painter(),
        name,
        Type::Caption,
        c.text_secondary,
        w - 34.0,
    );
    text::paint_in(
        ui.painter(),
        Rect::from_min_max(pos2(sw.right() + 6.0, r.top()), r.max),
        g,
        Align::Min,
        c.text_secondary,
    );
}

fn foundations(ui: &mut Ui) {
    let c = colors(ui);
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::Colours);
        let w = (ui.available_width() / 4.0).floor();
        egui::Grid::new("colors")
            .spacing(vec2(0.0, 2.0))
            .show(ui, |ui| {
                for (i, (name, pick)) in COLOR_TOKENS.iter().enumerate() {
                    swatch_cell(ui, name, pick(c), w);
                    if i % 4 == 3 {
                        ui.end_row();
                    }
                }
            });
        small(ui, tr(Msg::Tones));
        ui.horizontal_wrapped(|ui| {
            for (tone, m) in [
                (Tone::Success, Msg::MatchesType),
                (Tone::Warning, Msg::Modified),
                (Tone::Info, Msg::Info),
                (Tone::Danger, Msg::Invalid),
                (Tone::Neutral, Msg::Neutral),
            ] {
                chip::status(ui, tr(m), tone);
            }
        });
    });
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::Typography);
        small(
            ui,
            &trf(Msg::FontSource, &[&crate::fonts::source(ui.ctx()).name()]),
        );
        ui.spacing_mut().item_spacing.y = space::XS;
        for t in Type::ALL {
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(vec2(118.0, 20.0), Sense::hover());
                let label = format!("{} {}/{:?}", t.name(), t.size(), t.weight());
                let g = text::galley(ui.painter(), &label, Type::Caption, c.text_tertiary);
                text::paint_in(ui.painter(), r, g, Align::Min, c.text_tertiary);
                text::label(ui, tr(Msg::TypeSample), t, c.text);
            });
        }
        ui.add_space(space::XS);
        ui.horizontal_top(|ui| {
            for (m, tabular) in [(Msg::Tabular, true), (Msg::Proportional, false)] {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    small(ui, tr(m));
                    for n in ["1.111,11", "8.888,88", "+2,700", "−0,080"] {
                        if tabular {
                            text::tabular(ui, n, Type::Body, c.text, Some(80.0));
                        } else {
                            ui.allocate_ui_with_layout(
                                vec2(80.0, 17.0),
                                Layout::right_to_left(Align::Center),
                                |ui| {
                                    ui.label(text::rich(n, Type::Body, c.text));
                                },
                            );
                        }
                    }
                });
                ui.add_space(space::X4L);
            }
        });
    });
    ui.horizontal_top(|ui| {
        let w = (ui.available_width() - space::XL) / 2.0;
        ui.allocate_ui(vec2(w, 0.0), |ui| {
            card::Card::new().show(ui, |ui| {
                heading(ui, Msg::Spacing);
                ui.spacing_mut().item_spacing.y = 3.0;
                for (name, s) in crate::tokens::space::SCALE {
                    ui.horizontal(|ui| {
                        let (r, _) = ui.allocate_exact_size(vec2(60.0, 12.0), Sense::hover());
                        let g = text::galley(
                            ui.painter(),
                            &format!("{name} {s}"),
                            Type::Caption,
                            c.text_tertiary,
                        );
                        text::paint_in(ui.painter(), r, g, Align::Min, c.text_tertiary);
                        let (b, _) = ui.allocate_exact_size(vec2(s * 4.0, 10.0), Sense::hover());
                        ui.painter().rect_filled(b, 2, c.accent_tint);
                        ui.painter().rect_stroke(
                            b,
                            2,
                            Stroke::new(1.0, c.accent),
                            StrokeKind::Inside,
                        );
                    });
                }
                heading(ui, Msg::Motion);
                small(
                    ui,
                    &trf(
                        Msg::MotionText,
                        &[&motion::FAST, &motion::NORMAL, &motion::SLOW],
                    ),
                );
                let (r, _) =
                    ui.allocate_exact_size(vec2(ui.available_width(), 48.0), Sense::hover());
                let pts: Vec<_> = (0..=40)
                    .map(|i| {
                        let t = i as f32 / 40.0;
                        pos2(
                            r.left() + t * 80.0,
                            r.bottom() - motion::ease_out(t) * r.height(),
                        )
                    })
                    .collect();
                ui.painter()
                    .add(egui::Shape::line(pts, Stroke::new(1.5, c.accent)));
                let time = ui.input(|i| i.time) as f32;
                let t = (time / 1.6).fract();
                let x = r.left()
                    + 100.0
                    + motion::ease_in_out((t * 2.0).min(2.0 - t * 2.0)) * (r.width() - 120.0);
                ui.painter()
                    .circle_filled(pos2(x, r.center().y), 6.0, c.accent_fill);
                ui.ctx().request_repaint();
            });
        });
        ui.allocate_ui(vec2(w, 0.0), |ui| {
            card::Card::new().show(ui, |ui| {
                heading(ui, Msg::Radii);
                ui.horizontal_wrapped(|ui| {
                    for (name, r) in crate::tokens::radius::SCALE {
                        // A known size, so the wrapping row can wrap it.
                        ui.allocate_ui(vec2(52.0, 50.0), |ui| {
                            let (b, _) = ui.allocate_exact_size(vec2(44.0, 32.0), Sense::hover());
                            ui.painter().rect(
                                b,
                                r,
                                c.surface_sunken,
                                Stroke::new(1.0, c.border_strong),
                                StrokeKind::Inside,
                            );
                            small(ui, &format!("{name} {r}"));
                        });
                    }
                });
                heading(ui, Msg::Elevation);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(space::X3L, space::X3L);
                    for e in Elevation::ALL {
                        let (b, _) = ui.allocate_exact_size(vec2(56.0, 40.0), Sense::hover());
                        let p = ui.painter();
                        p.add(e.shadow(c.dark).as_shape(b, radius::TILE));
                        p.rect_filled(b, radius::TILE, c.surface_raised);
                        if e.ring() {
                            p.rect_stroke(
                                b,
                                radius::TILE,
                                Stroke::new(1.0, c.ring),
                                StrokeKind::Outside,
                            );
                        }
                        let g = text::galley(p, &format!("{e:?}"), Type::Caption, c.text_secondary);
                        text::paint_in(p, b, g, Align::Center, c.text_secondary);
                    }
                });
            });
        });
    });
}

// ------------------------------------------------------------------ icons

fn icons(ui: &mut Ui) {
    let c = colors(ui);
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::IconSizes);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.y = space::MD;
            for i in [
                Icon::Wall,
                Icon::Window,
                Icon::Roof,
                Icon::Search,
                Icon::Layers,
                Icon::Cube,
            ] {
                // One group per icon with a known size, so the row wraps
                // between groups.
                let sizes = [13.0, 16.0, 20.0, 26.0, 40.0];
                let w = sizes.iter().sum::<f32>() + ui.spacing().item_spacing.x * 4.0;
                ui.allocate_ui(vec2(w, 40.0), |ui| {
                    ui.horizontal(|ui| {
                        for s in sizes {
                            icon::icon(ui, i, s, c.icon);
                        }
                    });
                });
                ui.add_space(space::MD);
            }
        });
        ui.horizontal(|ui| {
            for (col, i) in [
                (c.accent, Icon::Wall),
                (c.success.solid, Icon::Success),
                (c.warning.solid, Icon::Warning),
                (c.danger.solid, Icon::Error),
                (c.text_disabled, Icon::Lock),
            ] {
                icon::icon(ui, i, 20.0, col);
            }
        });
    });
    card::Card::new().show(ui, |ui| {
        ui.label(text::rich(
            trf(Msg::AllIcons, &[&Icon::ALL.len()]),
            Type::Title,
            c.text,
        ));
        // As many columns as fit cells of at least 96 pt.
        let avail = ui.available_width();
        let cols = (avail / 96.0).floor().max(1.0);
        let cell = vec2((avail / cols).floor(), 40.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            for i in Icon::ALL {
                let (r, resp) = ui.allocate_exact_size(cell, Sense::hover());
                let p = ui.painter();
                if resp.hovered() {
                    p.rect_filled(r, radius::ROW, c.hover);
                }
                icon::paint(
                    p,
                    Rect::from_center_size(pos2(r.center().x, r.top() + 13.0), Vec2::splat(18.0)),
                    *i,
                    c.icon,
                );
                let g = text::truncated(p, i.name(), Type::Caption, c.text_tertiary, cell.x - 8.0);
                let elided = g.elided;
                text::paint_in(
                    p,
                    Rect::from_min_max(pos2(r.left(), r.top() + 24.0), r.max),
                    g,
                    Align::Center,
                    c.text_tertiary,
                );
                if elided {
                    resp.on_hover_text(i.name());
                }
            }
        });
    });
}

// ---------------------------------------------------------------- buttons

fn buttons(ui: &mut Ui, v: &mut Values) {
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::Buttons);
        ui.horizontal(|ui| {
            ui.add_space(78.0);
            for m in [
                Msg::Primary,
                Msg::Secondary,
                Msg::Tertiary,
                Msg::Destructive,
            ] {
                let (r, _) = ui.allocate_exact_size(vec2(104.0, 14.0), Sense::hover());
                let c = colors(ui);
                let g = text::galley(ui.painter(), tr(m), Type::Caption, c.text_tertiary);
                text::paint_in(ui.painter(), r, g, Align::Min, c.text_tertiary);
            }
        });
        states(ui, 70.0, &Forced::ALL, |ui, _| {
            for (variant, m) in [
                (Variant::Primary, Msg::Save),
                (Variant::Secondary, Msg::Back),
                (Variant::Tertiary, Msg::Cancel),
                (Variant::Destructive, Msg::Delete),
            ] {
                ui.add(Button::new(tr(m)).variant(variant).min_width(96.0));
                ui.add_space(4.0);
            }
        });
        small(ui, tr(Msg::Sizes));
        ui.horizontal(|ui| {
            for (s, m) in [
                (ButtonSize::Small, Msg::Small),
                (ButtonSize::Medium, Msg::Medium),
                (ButtonSize::Large, Msg::Large),
            ] {
                ui.add(Button::primary(tr(m)).size(s));
            }
            ui.add(Button::secondary(tr(Msg::Save)).icon(Icon::Save));
            ui.add(Button::secondary(tr(Msg::Scale)).trailing(Icon::ChevronDown));
            crate::widgets::button::link(ui, tr(Msg::AddLayer));
        });
        ui.horizontal(|ui| {
            let w = (ui.available_width() - space::MD) / 2.0;
            ui.add(Button::secondary(tr(Msg::ResetToType)).min_width(w));
            ui.add(Button::primary(tr(Msg::AllLinked)).fill_width());
        });
    });
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::IconButtons);
        states(ui, 70.0, &Forced::ALL, |ui, _| {
            ui.add(IconButton::new(Icon::Undo, tr(Msg::Undo)));
            ui.add(IconButton::new(Icon::Redo, tr(Msg::Redo)));
            ui.add(IconButton::new(Icon::SidebarLeft, tr(Msg::Properties)).selected(true));
            ui.add(IconButton::new(Icon::More, tr(Msg::Menu)).framed());
            ui.add_space(space::LG);
            ui.add(ToolButton::icon(Icon::Perspective, tr(Msg::Perspective)).selected(true));
            ui.add(ToolButton::icon(Icon::Rooms, tr(Msg::Rooms)));
            ui.add(ToolButton::icon(Icon::Cube, "").label("Iso VL").dropdown());
        });
    });
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::Toggles);
        states(
            ui,
            70.0,
            &[Forced::Rest, Forced::Hover, Forced::Focus, Forced::Disabled],
            |ui, _| {
                let mut a = v.switch;
                toggle::switch(ui, &mut a, "");
                let mut b = false;
                toggle::switch(ui, &mut b, "");
                ui.add_space(space::MD);
                let mut cc = true;
                toggle::checkbox(ui, &mut cc, "");
                let mut d = false;
                toggle::checkbox(ui, &mut d, "");
                let mut m = Check::Mixed;
                toggle::checkbox_tri(ui, &mut m, "");
                ui.add_space(space::MD);
                toggle::radio(ui, true, "");
                toggle::radio(ui, false, "");
            },
        );
        card::divider(ui);
        toggle::switch(ui, &mut v.switch, tr(Msg::ShowRooms));
        toggle::switch(ui, &mut v.switch2, tr(Msg::LinkToType));
        toggle::checkbox(ui, &mut v.check, tr(Msg::StartAtLogin));
        toggle::checkbox_tri(ui, &mut v.check_tri, tr(Msg::Mixed));
        ui.horizontal(|ui| {
            toggle::radio_value(ui, &mut v.radio, 0, tr(Msg::Left));
            toggle::radio_value(ui, &mut v.radio, 1, tr(Msg::Centre));
            toggle::radio_value(ui, &mut v.radio, 2, tr(Msg::Right));
        });
    });
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::Segmented);
        card::form_row(ui, tr(Msg::Axis), |ui| {
            let segs = [
                Segment::text(0u8, tr(Msg::Left)),
                Segment::text(1, tr(Msg::Centre)),
                Segment::text(2, tr(Msg::Right)),
            ];
            Segmented::new(&mut v.axis, &segs).show(ui);
        });
        ui.horizontal(|ui| {
            let segs = [
                Segment::icon(0u8, Icon::ViewSingle, tr(Msg::OneView)),
                Segment::icon(1, Icon::ViewTwo, tr(Msg::TwoViews)),
                Segment::icon(2, Icon::ViewThree, tr(Msg::ThreeViews)),
            ];
            Segmented::new(&mut v.views, &segs)
                .segment_width(30.0)
                .show(ui);
            ui.add_space(space::LG);
            let segs = [
                Segment::text(0u8, tr(Msg::Architecture)).with_icon(Icon::Home),
                Segment::text(1, tr(Msg::Draft)),
                Segment::text(2, tr(Msg::Engineering)),
            ];
            Segmented::new(&mut v.area, &segs).compact().show(ui);
        });
        with_forced(ui, Forced::Disabled, |ui| {
            let mut a = 1u8;
            let segs = [
                Segment::text(0u8, tr(Msg::Left)),
                Segment::text(1, tr(Msg::Centre)),
                Segment::text(2, tr(Msg::Right)),
            ];
            Segmented::new(&mut a, &segs).show(ui);
        });
    });
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::Chips);
        ui.horizontal_wrapped(|ui| {
            chip::status(ui, tr(Msg::MatchesType), Tone::Success);
            chip::status(ui, tr(Msg::Modified), Tone::Warning);
            chip::status(ui, tr(Msg::Invalid), Tone::Danger);
            chip::tag(ui, "U 0,21", Tone::Success, Some(54.0));
            chip::tag(ui, "U 0,36", Tone::Warning, Some(54.0));
            chip::anchor(ui, Some(tr(Msg::SlabAbove)), |ui| {
                let _ = MenuItem::new(tr(Msg::SlabAbove)).checked(true).show(ui);
            });
            chip::anchor(ui, None, |_| {});
        });
        // Editable values of a list: defaults muted, own values framed.
        ui.horizontal_wrapped(|ui| {
            let _ = chip::value(ui, "0,30", true, "");
            let _ = chip::value(ui, "1,05", false, "");
            let _ = chip::value(ui, "1,15", false, "");
            let _ = chip::add(ui, "", "");
        });
        ui.horizontal_wrapped(|ui| {
            for (i, m) in [Msg::Exterior, Msg::Interior, Msg::Basement, Msg::Drywall]
                .into_iter()
                .enumerate()
            {
                let r = chip::filter(ui, tr(m), v.filters[i], i == 3);
                if r.clicked {
                    v.filters[i] = !v.filters[i];
                }
            }
        });
    });
}

// ----------------------------------------------------------------- inputs

fn inputs(ui: &mut Ui, v: &mut Values, col: usize) {
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::TextFields);
        states(ui, 70.0, &Forced::ALL, |ui, f| {
            let mut s = v.name.clone();
            TextField::new(&mut s)
                .id_salt(("tf", col, f))
                .label(tr(Msg::Name))
                .width(200.0)
                .show(ui);
            let mut e = String::new();
            TextField::new(&mut e)
                .id_salt(("tf_hint", col, f))
                .hint(tr(Msg::NameHint))
                .width(220.0)
                .show(ui);
        });
        card::form_row(ui, tr(Msg::Url), |ui| {
            TextField::new(&mut v.url)
                .id_salt(("url", col))
                .prefix(Affix::Text("http://"))
                .suffix(Affix::Text(":8080"))
                .clearable()
                .show(ui);
        });
        card::form_row(ui, tr(Msg::Name), |ui| {
            let err = v.name_err.trim().is_empty().then(|| tr(Msg::ErrorEmpty));
            ui.vertical(|ui| {
                TextField::new(&mut v.name_err)
                    .id_salt(("err", col))
                    .hint(tr(Msg::NameHint))
                    .error(err)
                    .show(ui);
            });
        });
    });
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::SearchFields);
        field::search(ui, &mut v.search, tr(Msg::SearchType), true);
        ui.allocate_ui(vec2(260.0, 26.0), |ui| {
            field::search(ui, &mut v.search2, tr(Msg::SearchTool), false);
        });
    });
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::NumberFields);
        small(ui, tr(Msg::NumberHelp));
        card::form_row(ui, tr(Msg::Length), |ui| {
            NumberField::length(("len", col), &mut v.length, Unit::M)
                .label(tr(Msg::Length))
                .width(110.0)
                .show(ui);
            let c = colors(ui);
            ui.label(text::rich(
                format!("{} 0°", tr(Msg::Angle)),
                Type::Body,
                c.text_secondary,
            ));
        });
        card::form_row(ui, tr(Msg::Thickness), |ui| {
            NumberField::length(("thick", col), &mut v.thickness, Unit::Mm)
                .width(80.0)
                .show(ui);
        });
        card::form_row(ui, tr(Msg::Angle), |ui| {
            NumberField::new(("angle", col), &mut v.angle)
                .suffix("°")
                .decimals(1)
                .width(80.0)
                .show(ui);
        });
        card::form_row(ui, tr(Msg::Distance), |ui| {
            NumberField::length(("dist", col), &mut v.distance, Unit::M)
                .mixed(true)
                .width(80.0)
                .show(ui);
        });
        states(ui, 70.0, &[Forced::Focus, Forced::Disabled], |ui, f| {
            let mut x = 175.0;
            NumberField::length(("st", col, f), &mut x, Unit::Mm)
                .width(80.0)
                .show(ui);
        });
    });
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::Selects);
        card::form_row(ui, tr(Msg::Scale), |ui| {
            field::select(
                ui,
                ("scale", col),
                &mut v.scale,
                &[(50, "1:50"), (100, "1:100"), (200, "1:200"), (500, "1:500")],
                120.0,
            );
        });
        states(
            ui,
            70.0,
            &[Forced::Hover, Forced::Focus, Forced::Disabled],
            |ui, f| {
                let mut s = 100u32;
                field::select(
                    ui,
                    ("scale_state", col, f),
                    &mut s,
                    &[(100, "1:100")],
                    120.0,
                );
            },
        );
    });
    card::section(
        ui,
        tr(Msg::PositionHeights),
        |_| {},
        |ui| {
            card::form_row(ui, tr(Msg::Axis), |ui| {
                let segs = [
                    Segment::text(0u8, tr(Msg::Left)),
                    Segment::text(1, tr(Msg::Centre)),
                    Segment::text(2, tr(Msg::Right)),
                ];
                Segmented::new(&mut v.axis, &segs).show(ui);
            });
            card::inset(ui, |ui| {
                for (m, level) in [(Msg::TopEdge, "+2,700"), (Msg::BottomEdge, "−0,080")] {
                    ui.horizontal(|ui| {
                        let c = colors(ui);
                        let (r, _) = ui.allocate_exact_size(vec2(74.0, 22.0), Sense::hover());
                        let g = text::galley(ui.painter(), tr(m), Type::Body, c.text_secondary);
                        text::paint_in(ui.painter(), r, g, Align::Min, c.text_secondary);
                        chip::anchor(ui, Some(tr(Msg::SlabAbove)), |_| {});
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            text::tabular(ui, level, Type::Body, c.text, None);
                        });
                    });
                }
            });
        },
    );
}

// ------------------------------------------------------------------ menus

fn edit_menu(ui: &mut Ui, highlight: bool) {
    let _ = MenuItem::new(tr(Msg::Undo))
        .icon(Icon::Undo)
        .shortcut("⌘Z")
        .show(ui);
    let _ = MenuItem::new(tr(Msg::Redo))
        .icon(Icon::Redo)
        .shortcut("⇧⌘Z")
        .enabled(false)
        .show(ui);
    menu::separator(ui);
    let _ = MenuItem::new(tr(Msg::Cut))
        .icon_space()
        .shortcut("⌘X")
        .show(ui);
    let _ = MenuItem::new(tr(Msg::Copy))
        .icon(Icon::Copy)
        .shortcut("⌘C")
        .show(ui);
    let _ = MenuItem::new(tr(Msg::Paste))
        .icon_space()
        .shortcut("⌘V")
        .show(ui);
    menu::separator(ui);
    let _ = MenuItem::new(tr(Msg::TransferFormat))
        .icon(Icon::FormatTransfer)
        .shortcut("⌥⌘C")
        .highlighted(highlight)
        .show(ui);
    let _ = MenuItem::new(tr(Msg::AssignType)).icon_space().show(ui);
    menu::submenu(ui, MenuItem::new(tr(Msg::Arrange)).icon_space(), |ui| {
        let _ = MenuItem::new(tr(Msg::ToFront)).show(ui);
        let _ = MenuItem::new(tr(Msg::ToBack)).show(ui);
    });
    menu::separator(ui);
    let _ = MenuItem::new(tr(Msg::Delete))
        .icon(Icon::Delete)
        .shortcut("⌫")
        .destructive()
        .show(ui);
}

fn menus(ui: &mut Ui, v: &mut Values) {
    let c = colors(ui);
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            heading(ui, Msg::Menu);
            menu::panel(ui, 250.0, |ui| edit_menu(ui, true));
            ui.add_space(space::MD);
            menu::panel(ui, 200.0, |ui| {
                menu::section_header(ui, tr(Msg::Scale), Some(3));
                for (s, on) in [("1:50", false), ("1:100", true), ("1:200", false)] {
                    let _ = MenuItem::new(s).checked(on).show(ui);
                }
            });
        });
        ui.add_space(space::X4L);
        ui.vertical(|ui| {
            heading(ui, Msg::OpenMenu);
            let r = ui.add(Button::secondary(tr(Msg::OpenMenu)).trailing(Icon::ChevronDown));
            menu::menu_popup(ui, &r).show(|ui| {
                ui.set_min_width(240.0);
                edit_menu(ui, false);
            });
            let (area, resp) = ui.allocate_exact_size(vec2(220.0, 60.0), Sense::click());
            let p = ui.painter();
            let pts = [
                area.left_top(),
                area.right_top(),
                area.right_bottom(),
                area.left_bottom(),
                area.left_top(),
            ];
            p.extend(egui::Shape::dashed_line(
                &pts,
                Stroke::new(1.0, c.border_mixed),
                4.0,
                3.0,
            ));
            let g = text::wrapped(
                p,
                tr(Msg::ContextHint),
                Type::Caption,
                c.text_tertiary,
                200.0,
            );
            text::paint_in(p, area, g, Align::Center, c.text_tertiary);
            menu::context_menu(ui, &resp, |ui| edit_menu(ui, false));
            heading(ui, Msg::Tooltips);
            tooltip::panel(ui, |ui| {
                ui.label(text::rich(tr(Msg::Undo), Type::Body, c.text));
            });
            tooltip::panel(ui, |ui| {
                tooltip::content(ui, tr(Msg::WallTool), "W", tr(Msg::WallToolText))
            });
            let r = ui.add(RibbonTool::small(Icon::Wall, tr(Msg::WallTool), "W"));
            tooltip::rich(r, tr(Msg::WallTool), "W", tr(Msg::WallToolText));
        });
    });
    heading(ui, Msg::Popover);
    menu::popover_frame(c).show(ui, |ui| {
        ui.set_width(360.0);
        field::search(ui, &mut v.search, tr(Msg::SearchType), true);
        ui.horizontal(|ui| {
            for (i, m) in [Msg::Exterior, Msg::Interior, Msg::Basement]
                .into_iter()
                .enumerate()
            {
                if chip::filter(ui, tr(m), v.filters[i], false).clicked {
                    v.filters[i] = !v.filters[i];
                }
            }
        });
        list::group_header(ui, tr(Msg::Supplied), Some(2));
        let _ = ListRow::new("AW KS 17,5 + WDVS EPS 14")
            .secondary("Kalksandstein · EPS 032")
            .meta("335 mm")
            .tag("U 0,21", Tone::Success)
            .thumb(30.0, layer_thumb)
            .selected(true)
            .show(ui);
        let _ = ListRow::new("AW KS 17,5 + WDVS MW 16")
            .secondary("Kalksandstein · MW 035")
            .meta("355 mm")
            .tag("U 0,20", Tone::Success)
            .thumb(30.0, layer_thumb)
            .show(ui);
    });
    heading(ui, Msg::Dialog);
    ui.horizontal_top(|ui| {
        dialog::surface(ui, 300.0, |ui| {
            dialog::heading(ui, tr(Msg::DeleteTitle), tr(Msg::DeleteText));
            dialog::footer(
                ui,
                |ui| {
                    ui.add(Button::tertiary(tr(Msg::Cancel)));
                },
                |ui| {
                    ui.add(Button::destructive(tr(Msg::Delete)));
                },
            );
        });
        ui.add_space(space::X4L);
        if ui.add(Button::secondary(tr(Msg::OpenDialog))).clicked() {
            v.show_dialog = true;
        }
    });
    heading(ui, Msg::Wizard);
    Frame::new()
        .fill(c.surface_raised)
        .corner_radius(radius::OVERLAY)
        .stroke(Stroke::new(1.0, c.ring))
        .shadow(Elevation::Dialog.shadow(c.dark))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let steps = [
                tr(Msg::StepGoal),
                tr(Msg::StepStart),
                tr(Msg::StepChoose),
                tr(Msg::StepTest),
            ];
            dialog::wizard(
                ui,
                Icon::Sparkle,
                tr(Msg::SetUpAi),
                &steps,
                1,
                tr(Msg::WizardNote),
                |ui| {
                    dialog::heading(ui, tr(Msg::StepStart), tr(Msg::WizardText));
                    feedback::notice(ui, Tone::Info, tr(Msg::Found), tr(Msg::FoundText));
                    dialog::status_block(
                        ui,
                        Tone::Warning,
                        tr(Msg::Waiting),
                        tr(Msg::WaitingText),
                        true,
                        Some(tr(Msg::Help)),
                    );
                    dialog::footer(
                        ui,
                        |ui| {
                            ui.add(Button::tertiary(tr(Msg::Cancel)).size(ButtonSize::Large));
                        },
                        |ui| {
                            with_forced(ui, Forced::Disabled, |ui| {
                                ui.add(Button::primary(tr(Msg::Next)).size(ButtonSize::Large))
                            });
                            ui.add(Button::secondary(tr(Msg::Back)).size(ButtonSize::Large));
                        },
                    );
                },
            );
        });
}

fn layer_thumb(p: &egui::Painter, r: Rect) {
    let total: f64 = MATERIALS.iter().map(|m| m.2).sum();
    let mut y = r.top();
    for (_, rgb, t, hatch) in MATERIALS {
        let h = r.height() * (t / total) as f32;
        let band = Rect::from_min_max(pos2(r.left(), y), pos2(r.right(), y + h));
        p.rect_filled(
            band,
            0.0,
            Color32::from_rgb((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8),
        );
        if hatch {
            let clip = p.with_clip_rect(band);
            let mut x = band.left() - band.height();
            while x < band.right() {
                clip.line_segment(
                    [pos2(x, band.bottom()), pos2(x + band.height(), band.top())],
                    Stroke::new(1.0, Color32::from_rgb(0xB9, 0xA7, 0x88)),
                );
                x += 4.0;
            }
        }
        y += h;
    }
}

// ------------------------------------------------------------ collections

fn collections(ui: &mut Ui, v: &mut Values, col: usize) {
    card::Card::new()
        .padding(Margin::symmetric(0, space::XL as i8))
        .gap(0.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(space::XXL);
                heading(ui, Msg::Lists);
            });
            list::group_header(ui, &format!("{} · haus.occ", tr(Msg::Office)), Some(1));
            let _ = ListRow::new("AW KS 17,5 + WDVS 30")
                .secondary("Kalksandstein · EPS 032")
                .meta("475 mm")
                .tag("U 0,11", Tone::Success)
                .thumb(30.0, layer_thumb)
                .show(ui);
            list::group_header(ui, tr(Msg::Supplied), Some(3));
            let _ = ListRow::new("AW KS 17,5 + WDVS EPS 14")
                .secondary("Kalksandstein · EPS 032")
                .meta("335 mm")
                .tag("U 0,21", Tone::Success)
                .thumb(30.0, layer_thumb)
                .selected(true)
                .show(ui);
            with_forced(ui, Forced::Hover, |ui| {
                let _ = ListRow::new("AW KS 24 + WDVS MW 20")
                    .secondary("Kalksandstein · MW 035")
                    .meta("460 mm")
                    .tag("U 0,16", Tone::Success)
                    .thumb(30.0, layer_thumb)
                    .show(ui);
            });
            let _ = ListRow::new("AW Ziegel 36,5 monolithisch")
                .secondary("Hochlochziegel")
                .meta("395 mm")
                .tag("U 0,24", Tone::Warning)
                .thumb(30.0, layer_thumb)
                .show(ui);
            ui.add_space(space::MD);
            let _ = ListRow::new("EG Modell")
                .icon(Icon::Drawing)
                .meta("200")
                .selected(true)
                .selection(Selection::Soft)
                .show(ui);
            let _ = ListRow::new("OG Modell")
                .icon(Icon::Drawing)
                .meta("300")
                .show(ui);
            let _ = ListRow::new(tr(Msg::Library)).icon(Icon::Library).show(ui);
            with_forced(ui, Forced::Disabled, |ui| {
                let _ = ListRow::new("KG Modell")
                    .icon(Icon::Drawing)
                    .meta("100")
                    .show(ui);
            });
        });
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::Table);
        let cols = [
            Column::new(tr(Msg::Room), Width::Fill(1.0)),
            Column::new(tr(Msg::Storey), Width::Fixed(70.0)),
            Column::new(tr(Msg::Area), Width::Fixed(90.0)).numeric(),
            Column::new(tr(Msg::Height), Width::Fixed(80.0)).numeric(),
        ];
        let mut order: Vec<usize> = (0..ROOMS.len()).collect();
        let s = v.sort;
        order.sort_by(|a, b| {
            let (x, y) = (ROOMS[*a], ROOMS[*b]);
            let o = match s.column {
                0 => x.0.cmp(y.0),
                1 => x.1.cmp(y.1),
                2 => x.2.total_cmp(&y.2),
                _ => x.3.total_cmp(&y.3),
            };
            if s.ascending { o } else { o.reverse() }
        });
        let lang = lang();
        let selected = v
            .selected_room
            .and_then(|r| order.iter().position(|o| *o == r));
        let resp = Table::new(("rooms", col), &cols)
            .sort(&mut v.sort)
            .zebra(true)
            .selected(selected)
            .max_height(200.0)
            .show(ui, ROOMS.len(), |ui, row, c| {
                let r = ROOMS[order[row]];
                match c {
                    0 => {
                        table::cell_text(ui, r.0);
                    }
                    1 => {
                        table::cell_text(ui, r.1);
                    }
                    2 => {
                        table::cell_number(ui, &crate::length::format_number(r.2, 2, lang));
                    }
                    _ => {
                        table::cell_number(ui, &crate::length::format_number(r.3, 2, lang));
                    }
                }
            });
        if let Some(r) = resp.clicked {
            v.selected_room = Some(order[r]);
        }
    });
    card::Card::new().gap(0.0).show(ui, |ui| {
        heading(ui, Msg::Tree);
        ui.add_space(space::SM);
        let [o0, o1, o2] = &mut v.tree_open;
        let (vis, lock) = (&mut v.visible, &mut v.locked);
        let row = |ui: &mut Ui,
                   label: &str,
                   depth: usize,
                   ic: Icon,
                   open: Option<&mut bool>,
                   _index: usize,
                   vis: &mut bool,
                   lock: &mut bool,
                   selected: bool| {
            let mut r = TreeRow::new(label, depth)
                .icon(ic)
                .selected(selected)
                .muted(!*vis);
            if let Some(o) = open {
                r = r.expandable(o);
            }
            r.toggle(Toggle {
                on: lock,
                icon_on: Icon::Lock,
                icon_off: Icon::Unlock,
                tip: tr(Msg::Locked),
            })
            .toggle(Toggle {
                on: vis,
                icon_on: Icon::Eye,
                icon_off: Icon::EyeClosed,
                tip: tr(Msg::Visible),
            })
            .show(ui)
        };
        let [v0, v1, v2, v3, v4, v5] = vis;
        let [l0, l1, l2, l3, l4, l5] = lock;
        row(
            ui,
            "Demo-Haus",
            0,
            Icon::Structure,
            Some(o0),
            0,
            v0,
            l0,
            false,
        );
        if *o0 {
            row(ui, "EG", 1, Icon::Storey, Some(o1), 1, v1, l1, false);
            if *o1 {
                row(ui, "200 EG Modell", 2, Icon::Drawing, None, 2, v2, l2, true);
                row(
                    ui,
                    "210 EG Möblierung",
                    2,
                    Icon::Drawing,
                    None,
                    3,
                    v3,
                    l3,
                    false,
                );
            }
            row(ui, "OG", 1, Icon::Storey, Some(o2), 4, v4, l4, false);
            if *o2 {
                row(
                    ui,
                    "300 OG Modell",
                    2,
                    Icon::Drawing,
                    None,
                    5,
                    v5,
                    l5,
                    false,
                );
            }
        }
    });
}

// ------------------------------------------------------------- navigation

fn navigation(ui: &mut Ui, v: &mut Values, col: usize) {
    let c = colors(ui);
    heading(ui, Msg::AttachedTabs);
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        let items = [
            (Icon::Properties, tr(Msg::Properties)),
            (Icon::Library, tr(Msg::Library)),
            (Icon::RoomBook, tr(Msg::RoomBook)),
        ];
        if let Some(i) = tabs::attached(ui, &items, v.palette_tab) {
            v.palette_tab = i;
        }
        let corners = egui::CornerRadius {
            nw: if v.palette_tab == 0 { 0 } else { radius::CARD },
            ne: if v.palette_tab == 2 { 0 } else { radius::CARD },
            sw: radius::CARD,
            se: radius::CARD,
        };
        card::header(
            ui,
            Icon::Wall,
            tr(Msg::WallTool),
            tr(Msg::WallSubtitle),
            corners,
        );
    });
    heading(ui, Msg::UnderlineTabs);
    Frame::new()
        .fill(c.chrome)
        .corner_radius(radius::CARD)
        .inner_margin(Margin::symmetric(space::XXL as i8, 0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            let segs = [
                Segment::text(0u8, tr(Msg::Architecture)).with_icon(Icon::Home),
                Segment::text(1, tr(Msg::Draft)),
                Segment::text(2, tr(Msg::Engineering)),
            ];
            // Task areas and tabs in one row when there is room (as in the
            // ribbon), stacked in a narrow column.
            let one_row = ui.available_width() >= 720.0;
            let layout = if one_row {
                Layout::left_to_right(Align::Center)
            } else {
                Layout::top_down(Align::Min)
            };
            ui.with_layout(layout, |ui| {
                if !one_row {
                    ui.add_space(space::MD);
                }
                Segmented::new(&mut v.area, &segs)
                    .compact()
                    .height(30.0)
                    .show(ui);
                if one_row {
                    ui.add_space(space::XXL);
                }
                tabs::underline(
                    ui,
                    &[
                        tr(Msg::Components),
                        tr(Msg::Finishing),
                        tr(Msg::Modelling),
                        tr(Msg::Annotate),
                    ],
                    &mut v.ribbon_tab,
                );
            });
        });
    heading(ui, Msg::DocTabs);
    let titles = [
        (Icon::Plan, tr(Msg::PlanEg), "1:100"),
        (Icon::SectionView, tr(Msg::SectionAA), ""),
        (Icon::Cube, tr(Msg::View3dTab), ""),
    ];
    let tabs_: Vec<DocTab<'_>> = v
        .doc_tabs
        .iter()
        .map(|i| {
            let (ic, t, n) = titles[*i as usize];
            DocTab {
                icon: Some(ic),
                title: t,
                note: n,
                closable: true,
            }
        })
        .collect();
    let sel = v.doc_selected;
    let mut events = Vec::new();
    dock::pane(
        ui,
        vec2(ui.available_width(), 120.0),
        |ui| {
            events = tabs::doc_tabs(ui, ("doc", col), &tabs_, sel, true, 60.0);
        },
        |ui| {
            small(ui, tr(Msg::DocTabs));
        },
    );
    for e in events {
        match e {
            TabEvent::Select(i) => v.doc_selected = i,
            TabEvent::Close(i) if v.doc_tabs.len() > 1 => {
                v.doc_tabs.remove(i);
                v.doc_selected = v.doc_selected.min(v.doc_tabs.len() - 1);
            }
            TabEvent::Move(a, b) => {
                tabs::apply_move(&mut v.doc_tabs, a, b);
                v.doc_selected = b;
            }
            TabEvent::Add if v.doc_tabs.len() < 3 => {
                let missing = (0..3u8).find(|i| !v.doc_tabs.contains(i)).unwrap_or(0);
                v.doc_tabs.push(missing);
            }
            _ => {}
        }
    }
    heading(ui, Msg::Toolbars);
    Frame::new()
        .fill(c.chrome)
        .corner_radius(radius::CARD)
        .inner_margin(Margin::same(space::SM as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            toolbar::toolbar(ui, |ui| {
                ui.add(ToolButton::icon(Icon::Save, tr(Msg::Save)));
                ui.add(ToolButton::icon(Icon::Undo, tr(Msg::Undo)));
                with_forced(ui, Forced::Disabled, |ui| {
                    ui.add(ToolButton::icon(Icon::Redo, tr(Msg::Redo)))
                });
                toolbar::separator(ui);
                let segs = [
                    Segment::icon(0u8, Icon::ViewSingle, tr(Msg::OneView)),
                    Segment::icon(1, Icon::ViewTwo, tr(Msg::TwoViews)),
                    Segment::icon(2, Icon::ViewThree, tr(Msg::ThreeViews)),
                ];
                Segmented::new(&mut v.views, &segs)
                    .segment_width(30.0)
                    .show(ui);
                toolbar::separator(ui);
                ui.add(ToolButton::icon(Icon::SidebarLeft, tr(Msg::Properties)).selected(true));
                ui.add(ToolButton::icon(Icon::Theme, "Theme"));
            });
        });
    heading(ui, Msg::Floating);
    Frame::new()
        .fill(c.surface_sunken)
        .corner_radius(radius::CARD)
        .inner_margin(Margin::same(space::X3L as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            toolbar::floating(ui, |ui| {
                ui.add(ToolButton::icon(Icon::Cube, "").label("Iso VL").dropdown());
                toolbar::separator(ui);
                ui.add(ToolButton::icon(Icon::Perspective, tr(Msg::Perspective)).selected(true));
                ui.add(ToolButton::icon(Icon::Edges, tr(Msg::Edges)).selected(true));
                ui.add(ToolButton::icon(Icon::Rooms, tr(Msg::Rooms)));
                toolbar::separator(ui);
                ui.add(
                    ToolButton::icon(Icon::Cut, "")
                        .label(tr(Msg::SectionDd))
                        .dropdown(),
                );
                ui.add(ToolButton::text(tr(Msg::Content)).dropdown());
                toolbar::separator(ui);
                ui.add(ToolButton::icon(Icon::ZoomAll, tr(Msg::ZoomAll)));
            });
        });
    heading(ui, Msg::Ribbon);
    Frame::new()
        .fill(c.surface_sunken)
        .corner_radius(radius::CARD)
        .inner_margin(Margin::symmetric(0, space::MD as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            // A ribbon wider than its pane scrolls sideways.
            egui::ScrollArea::horizontal()
                .id_salt(("ribbon", col))
                .show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        toolbar::ribbon_group(ui, tr(Msg::WallsColumns), |ui| {
                            ui.add(
                                RibbonTool::large(Icon::Wall, tr(Msg::WallTool), "W").active(true),
                            );
                            ui.add(RibbonTool::large(Icon::Column, tr(Msg::Column_), "⇧P"));
                            toolbar::ribbon_column(ui, |ui| {
                                ui.add(RibbonTool::small(Icon::Beam, tr(Msg::Beam), ""));
                                ui.add(RibbonTool::small(Icon::PickProperties, tr(Msg::Pick), "I"));
                            });
                        });
                        toolbar::ribbon_group(ui, tr(Msg::Openings), |ui| {
                            ui.add(RibbonTool::large(Icon::Window, tr(Msg::Window_), "⇧W"));
                            ui.add(RibbonTool::large(Icon::Door, tr(Msg::Door), "U"));
                            toolbar::ribbon_column(ui, |ui| {
                                ui.add(RibbonTool::small(
                                    Icon::LiftSlideDoor,
                                    tr(Msg::LiftSlide),
                                    "⇧U",
                                ));
                                with_forced(ui, Forced::Hover, |ui| {
                                    ui.add(RibbonTool::small(Icon::Opening, tr(Msg::Opening), "⇧O"))
                                });
                            });
                        });
                    })
                });
        });
}

// --------------------------------------------------------------- feedback

fn feedback_page(ui: &mut Ui, v: &mut Values) {
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::Notices);
        feedback::notice(ui, Tone::Info, tr(Msg::Found), tr(Msg::NoticeInfo));
        feedback::notice(ui, Tone::Success, "", tr(Msg::NoticeSuccess));
        feedback::notice(ui, Tone::Warning, "", tr(Msg::NoticeWarning));
        feedback::notice(ui, Tone::Danger, "", tr(Msg::NoticeDanger));
    });
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::Toasts);
        ui.horizontal_top(|ui| {
            feedback::toast_body(ui, Tone::Success, tr(Msg::ToastText));
            ui.add_space(space::LG);
            if ui.add(Button::secondary(tr(Msg::ShowToast))).clicked() {
                v.toasts.push(Tone::Success, tr(Msg::ToastText), Some(3.0));
            }
        });
        heading(ui, Msg::Status);
        dialog::status_block(
            ui,
            Tone::Warning,
            tr(Msg::Waiting),
            tr(Msg::WaitingText),
            true,
            Some(tr(Msg::Help)),
        );
        dialog::status_block(
            ui,
            Tone::Success,
            tr(Msg::NoticeSuccess),
            tr(Msg::FoundText),
            false,
            None,
        );
    });
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::Progress);
        ui.horizontal(|ui| {
            feedback::progress(ui, Some(v.progress), 160.0, Tone::Info);
            if ui
                .add(Button::tertiary("+10 %").size(ButtonSize::Small))
                .clicked()
            {
                v.progress = (v.progress + 0.1).min(1.0);
                if v.progress >= 1.0 {
                    v.progress = 0.0;
                }
            }
        });
        feedback::progress(ui, Some(0.8), 160.0, Tone::Success);
        feedback::progress(ui, None, 160.0, Tone::Info);
        ui.horizontal(|ui| {
            feedback::spinner(ui, 16.0);
            feedback::spinner(ui, 24.0);
            feedback::spinner(ui, 32.0);
        });
    });
    card::Card::new().show(ui, |ui| {
        heading(ui, Msg::Empty);
        feedback::empty_state(
            ui,
            Icon::Drawing,
            tr(Msg::EmptyTitle),
            tr(Msg::EmptyText),
            Some(tr(Msg::EmptyAction)),
        );
    });
}

// ----------------------------------------------------------------- layout

fn layout_page(ui: &mut Ui, v: &mut Values, col: usize) {
    let c = colors(ui);
    heading(ui, Msg::Cards);
    card::section(
        ui,
        tr(Msg::Type_),
        |ui| {
            chip::status(ui, tr(Msg::MatchesType), Tone::Success);
        },
        |ui| {
            let _ = ListRow::new("AW KS 17,5 + WDVS EPS 14")
                .secondary(&format!("{} · 335 mm · U 0,210", tr(Msg::Supplied)))
                .thumb(26.0, layer_thumb)
                .show(ui);
        },
    );
    card::section(
        ui,
        tr(Msg::Layers),
        |_| {},
        |ui| {
            let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 40.0), Sense::hover());
            let total: f64 = MATERIALS.iter().map(|m| m.2).sum();
            let mut x = r.left();
            for (_, rgb, t, _) in MATERIALS {
                let w = r.width() * (t / total) as f32;
                ui.painter().rect_filled(
                    Rect::from_min_max(pos2(x, r.top()), pos2(x + w, r.bottom())),
                    0.0,
                    Color32::from_rgb((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8),
                );
                x += w;
            }
            ui.painter().rect_stroke(
                r,
                radius::FIELD,
                Stroke::new(1.0, c.border_strong),
                StrokeKind::Inside,
            );
            ui.horizontal(|ui| {
                small(ui, tr(Msg::Inside));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    small(ui, tr(Msg::Outside))
                });
            });
            for (name, rgb, t, _) in MATERIALS {
                ui.horizontal(|ui| {
                    let (sw, _) = ui.allocate_exact_size(Vec2::splat(12.0), Sense::hover());
                    ui.painter().rect(
                        sw,
                        radius::XS,
                        Color32::from_rgb((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8),
                        Stroke::new(1.0, c.border_strong),
                        StrokeKind::Inside,
                    );
                    ui.label(text::rich(name, Type::Body, c.text));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        text::tabular(ui, &format!("{t:.0} mm"), Type::Body, c.text, Some(64.0));
                    });
                });
            }
            card::divider(ui);
            ui.horizontal(|ui| {
                crate::widgets::button::link(ui, tr(Msg::AddLayer));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(text::rich("335 mm", Type::BodyStrong, c.text));
                    ui.label(text::rich(tr(Msg::Total), Type::Body, c.text_secondary));
                });
            });
        },
    );
    card::collapsible(ui, ("physics", col), tr(Msg::Physics), true, |ui| {
        ui.horizontal(|ui| {
            ui.label(text::rich("0,210", Type::Display, c.text));
            ui.label(text::rich("W/(m²K)", Type::Body, c.text_secondary));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                chip::status(ui, "GEG ≤ 0,28 ✓", Tone::Success);
            });
        });
    });
    card::collapsible(ui, ("collapsed", col), tr(Msg::Collapsible), false, |ui| {
        small(ui, tr(Msg::Collapsible));
    });
    heading(ui, Msg::Splitter);
    Frame::new()
        .fill(c.surface_raised)
        .corner_radius(radius::CARD)
        .show(ui, |ui| {
            Splitter::new(("split", col), Axis::Horizontal)
                .initial(0.6)
                .height(70.0)
                .show(
                    ui,
                    |ui| {
                        ui.centered_and_justified(|ui| small(ui, tr(Msg::PaneA)));
                    },
                    |ui| {
                        ui.centered_and_justified(|ui| small(ui, tr(Msg::PaneB)));
                    },
                );
        });
    heading(ui, Msg::Docking);
    let tabs_ = [DocTab {
        icon: Some(Icon::SectionView),
        title: tr(Msg::SectionAA),
        note: "",
        closable: false,
    }];
    let w = ui.available_width();
    let r = dock::pane(
        ui,
        vec2(w, 170.0),
        |ui| {
            ui.horizontal(|ui| {
                let _ = tabs::doc_tabs(ui, ("dock", col), &tabs_, 0, false, 70.0);
            });
            let strip = ui.max_rect();
            let mut b = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(Rect::from_min_max(
                        pos2(strip.right() - 62.0, strip.top() + 5.0),
                        pos2(strip.right() - 4.0, strip.bottom() - 5.0),
                    ))
                    .layout(Layout::left_to_right(Align::Center)),
            );
            dock::pane_button(&mut b, Icon::Undock, tr(Msg::Undock));
            dock::pane_button(&mut b, Icon::Maximize, tr(Msg::Maximise));
        },
        |ui| ui.max_rect(),
    );
    let center = r.center();
    let hovered = dock::drop_cross(ui, center, Some(DropZone::Right));
    dock::drop_preview(ui, DropZone::Right.preview(r).shrink(4.0));
    let _ = hovered;
    let _ = v;
}

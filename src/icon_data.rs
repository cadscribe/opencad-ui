// The icon paths. The tools come from the mockup RibbonPages.dc.html (24 x 24
// grid); the interface icons from Workspace, Ribbon, Chat and Wizard (16 x 16)
// or were drawn to match them. Every path is parsed by a test in icon.rs.

icons! {
    /// Wall (Wand).
    Wall = 24 [S("M3 8h18v8H3zM3 12h18M9 8v4M15 12v4")];
    /// Column (Stütze).
    Column = 24 [S("M8 8h8v8H8zM8 16l8-8M8 12l4-4M12 16l4-4")];
    /// Beam (Unterzug).
    Beam = 24 [S("M3 9h18v6H3zM3 12h18")];
    /// Slab (Decke).
    Slab = 24 [S("M3 11l6-4h12l-6 4zM3 11v3h12v-3M15 14l6-4V7")];
    /// Slab opening.
    SlabOpening = 24 [S("M3 5h18v14H3zM9 9h6v6H9z")];
    /// Opening from a stair.
    StairOpening = 24 [S("M3 5h18v14H3zM7 16h3v-3h3v-3h4")];
    /// Delete an opening.
    DeleteOpening = 24 [S("M3 5h18v14H3zM9 9l6 6M15 9l-6 6")];
    /// Pick properties.
    PickProperties = 24 [S("M15 4l5 5-9 9H6v-5zM13 6l5 5M4 20l2-2")];
    /// Window.
    Window = 24 [S("M2 9h4M18 9h4M2 15h4M18 15h4M6 9h12v6H6zM12 9v6M6 12h12")];
    /// Door.
    Door = 24 [S("M2 19h4M18 19h4M6 19V6M6 6a13 13 0 0 1 13 13")];
    /// Lift-and-slide door.
    LiftSlideDoor = 24 [S("M2 9h20v6H2zM6 12h10M14 10l2 2-2 2")];
    /// Opening.
    Opening = 24 [S("M2 9h5M17 9h5M2 15h5M17 15h5M7 9v6M17 9v6")];
    /// Stair.
    Stair = 24 [S("M3 20h4v-4h4v-4h4V8h4V4")];
    /// Roof.
    Roof = 24 [S("M2 13 12 4l10 9M5 11v9h14v-9")];
    /// Dormer.
    Dormer = 24 [S("M2 18 12 5l10 13M3.5 18h17M9 13.5l3-3 3 3v4.5H9z")];
    /// Roof window.
    RoofWindow = 24 [S("M3 19 8 7h13l-5 12zM10.5 10h6l-2.5 6h-6zM13.5 10l-2.5 6M9.3 13h6.4")];
    /// Section.
    Section = 24 [S("M4 4v16M20 4v16M4 12h16M5 4h4M5 20h4M15 4h4M15 20h4")];
    /// Elevations.
    Elevations = 24 [S("M3 20h18M5 20V10l7-5 7 5v10M10 20v-5h4v5")];
    /// Open a section.
    OpenSection = 24 [S("M3 5h18v14H3zM3 12h18M12 5v14")];
    /// Room.
    Room = 24 [S("M4 4h16v16H4zM9 12h6M12 9v6")];
    /// Room separation line.
    RoomSeparator = 24 [S("M4 4v16M20 4v16M4 12h3M10 12h4M17 12h3")];
    /// Railing (Geländer).
    Railing = 24 [S("M3 7h18M4 7v13M20 7v13M8 7v13M12 7v13M16 7v13M3 20h18")];
    /// Cladding or boxing (Verkleidung, Kofferung).
    Cladding = 24 [S("M2 4h20M6 4v12h12V4M10 8h4v4h-4z")];
    /// Niche (Nische): a wall in plan with a niche from one face.
    Niche = 24 [S("M2 8h18v8H2zM8 8v4h6V8")];
    /// Slot (Schlitz): a wall with a narrow slot and its diagonal.
    Slot = 24 [S("M2 8h18v8H2zM10 8v3h3V8M10 8l3 3")];
    /// Recess (Aussparung): a wall with a deep pocket and its diagonal.
    Recess = 24 [S("M2 8h18v8H2zM6 8v5h10V8M6 8l10 5")];
    /// Through-hole (Durchbruch): a crossed hole through a wall.
    ThroughHole = 24 [S("M2 8h7M15 8h7M2 16h7M15 16h7M9 8h6v8H9zM9 8l6 8M15 8l-6 8")];
    /// Chimney (Schornstein): through a roof slope, a cap on top.
    Chimney = 24 [S("M2 21L22 9M13 15.6V4h5v8.6M12 4h7M15 4v6M16.5 4v6")];
    /// Room book.
    RoomBook = 24 [S("M6 3h12v18H6zM9 8h6M9 12h6M9 16h4")];
    /// Socket (Steckdose).
    Socket = 24 [S("M12 3a9 9 0 1 0 0 18a9 9 0 1 0 0-18zM9 10v4M15 10v4")];
    /// Switch (Schalter).
    Switch = 24 [S("M5 16a3 3 0 1 0 6 0a3 3 0 1 0-6 0M10 14l8-8M16 4l4 4")];
    /// Light outlet (Leuchtenauslass).
    LightOutlet = 24 [S("M12 4a8 8 0 1 0 0 16a8 8 0 1 0 0-16zM6.3 6.3l11.4 11.4M17.7 6.3 6.3 17.7")];
    /// Data socket (Datendose).
    DataSocket = 24 [S("M4 6h16v12H4zM9 15v-4h6v4M11 11V9h2v2")];
    /// TV socket (Antennendose).
    TvSocket = 24 [S("M3 7h18v12H3zM8 22h8M8 3l4 4 4-4")];
    /// Junction box (Abzweigdose).
    JunctionBox = 24 [S("M12 7a5 5 0 1 0 0 10a5 5 0 1 0 0-10zM2 12h5M17 12h5M12 2v5")];
    /// Distribution board (Verteiler).
    Board = 24 [S("M5 3h14v18H5zM5 21 19 3")];
    /// Smoke detector (Rauchwarnmelder).
    SmokeDetector = 24 [S("M12 4a8 8 0 1 0 0 16a8 8 0 1 0 0-16zM12 9a3 3 0 1 0 0 6a3 3 0 1 0 0-6z")];
    /// Sensor.
    Sensor = 24 [S("M9 9h6v6H9zM6 6a8.5 8.5 0 0 0 0 12M18 6a8.5 8.5 0 0 1 0 12")];
    /// Wall display (a screen in a frame).
    WallDisplay = 24 [S("M4 5h16v12H4zM6.5 7.5h11v7h-11zM12 17v2")];
    /// Door station (camera and call button).
    DoorStation = 24 [S("M8 3h8v18H8zM12 6.5a1.5 1.5 0 1 0 0 3a1.5 1.5 0 1 0 0-3zM10 14h4M10 17h4")];
    /// Door opener (a strike plate in a frame).
    DoorOpener = 24 [S("M5 3h9v18H5zM17 7h2v10h-2zM14 12h3M15 10l2 2-2 2")];
    /// Wi-Fi access point.
    AccessPoint = 24 [S("M5 15h14v4H5zM8 17h1M12 11a3 3 0 0 1 3 2M12 11a3 3 0 0 0-3 2M12 7a7 7 0 0 1 6.5 4.5M12 7a7 7 0 0 0-6.5 4.5")];
    /// Wi-Fi coverage heat map.
    WifiCoverage = 24 [S("M3 4h18v16H3zM3 12h18M12 4v16M7 8h1M16 16h1")];
    /// A measured Wi-Fi level.
    WifiMeasure = 24 [S("M12 20a2 2 0 1 0 0-4a2 2 0 1 0 0 4zM8 13a5.5 5.5 0 0 1 8 0M5 10a9.5 9.5 0 0 1 14 0")];
    /// Drainage pipe (Abwasser): a double-line elbow falling down.
    PipeDrainage = 24 [S("M3 6h14v14M3 10h10v10M7 3l3 3-3 3")];
    /// Water pipe (Trinkwasser): a drop.
    PipeWater = 24 [S("M12 3c-3 4-6 7.5-6 11a6 6 0 0 0 12 0c0-3.5-3-7-6-11z")];
    /// Heating pipe (Heizung): a thermometer.
    PipeHeating = 24 [S("M10 4a2 2 0 0 1 4 0v10a4 4 0 1 1-4 0zM12 11v6")];
    /// Ventilation duct (Lüftung): a duct with an air arrow.
    Duct = 24 [S("M3 7h18v10H3zM7 12h8M12 9l3 3-3 3")];
    /// Radiator (Heizkörper).
    Radiator = 24 [S("M4 6h16v11H4zM8 6v11M12 6v11M16 6v11M7 17v3M17 17v3")];
    /// Underfloor heating (Fußbodenheizung).
    FloorHeating = 24 [S("M3 20h18M5 16h13a2 2 0 0 0 0-4H6a2 2 0 0 1 0-4h13")];
    /// Device list.
    DeviceList = 24 [S("M9 6h11M9 12h11M9 18h11M4 5h2v2H4zM4 11h2v2H4zM4 17h2v2H4z")];
    /// Finish surfaces.
    Surfaces = 24 [S("M4 4h16v16H4zM4 10l6-6M4 16 16 4M10 20 20 10M16 20l4-4")];
    /// 3D solid.
    Solid3d = 24 [S("M12 3l8 4.5v9L12 21l-8-4.5v-9zM4 7.5 12 12l8-4.5M12 12v9")];
    /// 3D surface.
    Surface3d = 24 [S("M3 14l9-5 9 5-9 5z")];
    /// Union of 3D bodies.
    BodyUnion = 24 [S("M4 9h7V4h9v11h-7v5H4z")];
    /// Subtraction of 3D bodies.
    BodySubtract = 24 [S("M4 4h12v6h-6v6H4zM13 13h7v7h-7z")];
    Revolve = 24 [S("M12 2v20M7 5h10M6 8c0 2 2 3 2 5s-2 3-2 6h12c0-3-2-4-2-6s2-3 2-5")];
    Sweep = 24 [S("M3 20c0-9 5-15 16-16M3 13h4v7H3zM17 2h4v5h-4z")];
    Loft = 24 [S("M4 20h16M7 4h10M4 20l3-16M20 20 17 4M4 20h16l-3-16")];
    ArrayGrid = 24 [S("M4 4h4v4H4zM10 4h4v4h-4zM16 4h4v4h-4zM4 10h4v4H4zM10 10h4v4h-4zM16 10h4v4h-4zM4 16h4v4H4zM10 16h4v4h-4zM16 16h4v4h-4z")];
    ArrayExplode = 24 [S("M4 4h6v6H4zM14 14h6v6h-6zM14 4h6M17 1v6M4 17h6")];
    /// Text.
    Text = 24 [S("M5 6h14M12 6v13M9 19h6")];
    /// Dimension line.
    Dimension = 24 [S("M3 12h18M3 8v8M21 8v8M6 10l-3 2 3 2M18 10l3 2-3 2")];
    /// Angle dimension.
    AngleDimension = 24 [S("M4 20h16M4 20 16 6M11 20a7 7 0 0 0-2.4-5.3")];
    /// Radius / diameter dimension.
    RadialDimension = 24 [S("M4 12a8 8 0 1 0 16 0a8 8 0 1 0-16 0M12 12l5.5-5.5")];
    /// Add a dimension point.
    DimAddPoint = 24 [S("M3 14h18M3 10v8M21 10v8M12 4v6M9 7h6")];
    /// Delete a dimension point.
    DimDeletePoint = 24 [S("M3 14h18M3 10v8M21 10v8M9 7h6")];
    /// Move a dimension line.
    DimMoveLine = 24 [S("M3 9h18M3 15h18M12 10v4M10 12l2 2 2-2")];
    /// Level marker.
    LevelMarker = 24 [S("M4 16h16M8 16l4-5 4 5M14 8h6")];
    /// Label.
    Label = 24 [S("M4 6h10l6 6-6 6H4zM8 12h.01")];
    /// Layout (sheet).
    Layout = 24 [S("M4 3h16v18H4zM4 16h16M14 16v5")];
    /// Viewport.
    Viewport = 24 [S("M4 4h16v16H4zM12 4v16")];
    /// Line.
    Line = 24 [S("M4 20 20 4")];
    /// Polyline.
    Polyline = 24 [S("M3 18l6-8 5 5 7-10")];
    /// Rectangle.
    Rectangle = 24 [S("M4 6h16v12H4z")];
    /// Circle.
    Circle = 24 [S("M4 12a8 8 0 1 0 16 0a8 8 0 1 0-16 0")];
    /// Arc.
    Arc = 24 [S("M4 18a10 10 0 0 1 16 0")];
    /// Ellipse.
    Ellipse = 24 [S("M3 12a9 5 0 1 0 18 0a9 5 0 1 0-18 0")];
    /// Spline.
    Spline = 24 [S("M3 17c4-12 8 8 12-4s4-6 6-6")];
    /// Hatch.
    Hatch = 24 [S("M4 4h16v16H4zM4 12l8-8M4 20 20 4M12 20l8-8")];
    /// Pattern.
    Pattern = 24 [S("M4 4h16v16H4zM8 8h3v3H8zM13 13h3v3h-3z")];
    /// Fill.
    Fill = 24 [S("M4 4h16v16H4zM4 8h16M4 12h16M4 16h16")];
    /// Smart part.
    SmartFit = 24 [S("M4 4h7v7H4zM13 13h7v7h-7zM11 7h4v6")];
    /// Paint.
    Paint = 24 [S("M5 14l7-7 5 5-7 7zM15 5l4 4")];
    /// Explode a symbol reference into its elements.
    Explode = 24 [S("M4 4h5v5H4zM15 4h5v5h-5zM4 15h5v5H4zM15 15h5v5h-5zM10.5 10.5l3 3M13.5 10.5l-3 3")];
    /// Select.
    Select = 24 [S("M6 3l12 7-5 1.5L11 18z")];
    /// Select all.
    SelectAll = 24 [S("M4 8V4h4M16 4h4v4M20 16v4h-4M8 20H4v-4M8 8h3.5v3.5H8zM12.5 12.5H16V16h-3.5z")];
    /// Sum function.
    Sum = 24 [S("M6 3l12 7-5 1.5L11 18zM17 15v6M14 18h6")];
    /// Measure.
    Measure = 24 [S("M3 15 15 3l6 6L9 21zM7 11l2 2M10 8l2 2M13 5l2 2")];
    /// Measure an area.
    Area = 24 [S("M4 4h16v16H4zM4 4l16 16M12 4 4 12M20 12l-8 8")];
    /// Reference point.
    Reference = 24 [S("M9 12a3 3 0 1 0 6 0a3 3 0 1 0-6 0M12 2v5M12 17v5M2 12h5M17 12h5")];
    /// Save.
    Save = 24 [S("M4 3h12l4 4v14H4zM8 3v5h8V3M7 21v-7h10v7")];
    /// Undo.
    Undo = 24 [S("M8 5 4 9l4 4M4 9h11a5 5 0 0 1 0 10h-4")];
    /// Redo.
    Redo = 24 [S("M16 5l4 4-4 4M20 9H9a5 5 0 0 0 0 10h4")];
    /// Zoom all.
    ZoomAll = 24 [S("M3 8V3h5M16 3h5v5M21 16v5h-5M8 21H3v-5")];
    /// 3D view (tool).
    View3d = 24 [S("M12 3l8 4.5v9L12 21l-8-4.5v-9zM4 7.5 12 12l8-4.5M12 12v9")];
    /// Filter.
    Filter = 24 [S("M3 5h18l-7 8v6l-4-2v-4z")];
    /// Attributes.
    Attributes = 24 [S("M4 6h16M4 12h16M4 18h10")];
    /// Transfer format.
    FormatTransfer = 24 [S("M4 8h10M10 4l4 4-4 4M20 16H10M14 12l-4 4 4 4")];
    /// Point modification (stretch).
    Stretch = 24 [S("M3 12h18M7 8l-4 4 4 4M17 8l4 4-4 4")];
    /// Parallel.
    Parallel = 24 [S("M4 18 18 4M8 21 21 8")];
    /// Intersection.
    Intersect = 24 [S("M3 7h11M17 3v11M14 7l3 0")];
    /// Extend.
    Extend = 24 [S("M4 12h11M18 5v14M12 9l3 3-3 3")];
    /// Divide.
    Divide = 24 [S("M3 12h18M12 7v10")];
    /// Fillet.
    Fillet = 24 [S("M4 20V11a7 7 0 0 1 7-7h9")];
    /// Chamfer.
    Chamfer = 24 [S("M4 20v-9l7-7h9")];
    /// Offset curves: a rounded outline and its copy beside it, the arcs
    /// on one centre.
    Offset = 24 [S("M4 20V11a7 7 0 0 1 7-7h9M8 20v-9a3 3 0 0 1 3-3h9")];
    /// Project edges of a body onto a sketch plane: the body above, its
    /// edges dashed down, the edge on the plane thick.
    Project = 24 [S("M7 3h10v4H7zM3 20l4-5h14l-4 5z"), D("M8 8.5v5M16 8.5v5", 1.5, 2.0), W("M8 17.5h8", 1.5)];
    /// Copy.
    Copy = 24 [S("M8 8h12v12H8zM4 16V4h12")];
    /// Move.
    Move = 24 [S("M12 3v18M3 12h18M9 6l3-3 3 3M9 18l3 3 3-3M6 9l-3 3 3 3M18 9l3 3-3 3")];
    /// Rotate.
    Rotate = 24 [S("M20 12a8 8 0 1 1-2.5-5.8M20 4v5h-5")];
    /// Mirror.
    Mirror = 24 [S("M12 3v18M9 7 4 17h5zM15 7l5 10h-5z")];
    /// Scale.
    Scale = 24 [S("M4 20 20 4M14 4h6v6M10 20H4v-6")];
    /// Delete.
    Delete = 24 [S("M5 7h14M10 7V4h4v3M7 7l1 13h8l1-13")];
    /// Search field.
    Search = 16 [S("M2.5 7a4.5 4.5 0 1 0 9 0a4.5 4.5 0 1 0 -9 0zM10.5 10.5l3 3")];
    /// Add.
    Plus = 16 [S("M8 3v10M3 8h10")];
    /// Remove.
    Minus = 16 [S("M3 8h10")];
    /// Close.
    Close = 16 [S("M4 4l8 8M12 4l-8 8")];
    /// Checkmark.
    Check = 16 [S("M3.5 8.5l3 3 6-7")];
    /// Chevron down.
    ChevronDown = 16 [S("M4 6l4 4 4-4")];
    /// Chevron up.
    ChevronUp = 16 [S("M4 10l4-4 4 4")];
    /// Chevron right.
    ChevronRight = 16 [S("M6 4l4 4-4 4")];
    /// Chevron left.
    ChevronLeft = 16 [S("M10 4 6 8l4 4")];
    /// More actions.
    More = 16 [W("M3.5 8h.01M8 8h.01M12.5 8h.01", 1.7)];
    /// Information.
    Info = 16 [S("M2 8a6 6 0 1 0 12 0a6 6 0 1 0 -12 0z"), S("M8 7v4M8 5v.01")];
    /// Warning.
    Warning = 16 [S("M8 2 14.5 13.5h-13z"), S("M8 6.5v3.5M8 12v.01")];
    /// Error.
    Error = 16 [S("M2 8a6 6 0 1 0 12 0a6 6 0 1 0 -12 0z"), S("M5.9 5.9l4.2 4.2M10.1 5.9l-4.2 4.2")];
    /// Success.
    Success = 16 [S("M2 8a6 6 0 1 0 12 0a6 6 0 1 0 -12 0z"), S("M5.2 8.2l1.9 1.9 3.7-4")];
    /// Help.
    Help = 16 [S("M2 8a6 6 0 1 0 12 0a6 6 0 1 0 -12 0z"), S("M6.2 6.3a1.9 1.9 0 1 1 2.6 1.8c-.5.2-.8.6-.8 1.1v.4M8 11.3v.01")];
    /// Clear a field.
    Clear = 16 [S("M2 8a6 6 0 1 0 12 0a6 6 0 1 0 -12 0z"), S("M6 6l4 4M10 6l-4 4")];
    /// Settings.
    Settings = 16 [S("M2.5 4.5h2M8 4.5h5.5M2.5 8h7M13 8h.5M2.5 11.5h1M7 11.5h6.5"), S("M4.5 4.5a1.75 1.75 0 1 0 3.5 0a1.75 1.75 0 1 0 -3.5 0zM9.5 8a1.75 1.75 0 1 0 3.5 0a1.75 1.75 0 1 0 -3.5 0zM3.5 11.5a1.75 1.75 0 1 0 3.5 0a1.75 1.75 0 1 0 -3.5 0z")];
    /// Appearance (light / dark).
    Theme = 16 [S("M2.4 8a5.6 5.6 0 1 0 11.2 0a5.6 5.6 0 1 0 -11.2 0z"), F("M8 2.4a5.6 5.6 0 0 1 0 11.2z")];
    /// Show or hide the left panel.
    SidebarLeft = 16 [S("M3.5 2.5h9a2 2 0 0 1 2 2v7a2 2 0 0 1 -2 2h-9a2 2 0 0 1 -2 -2v-7a2 2 0 0 1 2 -2zM6 2.5v11")];
    /// Show or hide the right panel.
    SidebarRight = 16 [S("M3.5 2.5h9a2 2 0 0 1 2 2v7a2 2 0 0 1 -2 2h-9a2 2 0 0 1 -2 -2v-7a2 2 0 0 1 2 -2zM10 2.5v11")];
    /// One view.
    ViewSingle = 16 [S("M3.5 3h9a1.5 1.5 0 0 1 1.5 1.5v7a1.5 1.5 0 0 1 -1.5 1.5h-9a1.5 1.5 0 0 1 -1.5 -1.5v-7a1.5 1.5 0 0 1 1.5 -1.5z")];
    /// Two views.
    ViewTwo = 16 [S("M3.5 3h9a1.5 1.5 0 0 1 1.5 1.5v7a1.5 1.5 0 0 1 -1.5 1.5h-9a1.5 1.5 0 0 1 -1.5 -1.5v-7a1.5 1.5 0 0 1 1.5 -1.5zM8 3v10")];
    /// Three views.
    ViewThree = 16 [S("M3.5 3h9a1.5 1.5 0 0 1 1.5 1.5v7a1.5 1.5 0 0 1 -1.5 1.5h-9a1.5 1.5 0 0 1 -1.5 -1.5v-7a1.5 1.5 0 0 1 1.5 -1.5zM8 3v10M8 8h6")];
    /// Maximise.
    Maximize = 16 [S("M9.5 2.5h4v4M6.5 13.5h-4v-4M13.5 2.5 9 7M2.5 13.5 7 9")];
    /// Undock (float).
    Undock = 16 [S("M7 3H4.5A1.5 1.5 0 0 0 3 4.5v7A1.5 1.5 0 0 0 4.5 13h7a1.5 1.5 0 0 0 1.5 -1.5V9M9.5 3H13v3.5M13 3 7.5 8.5")];
    /// Dock.
    Dock = 16 [S("M3.5 2h9a1.5 1.5 0 0 1 1.5 1.5v9a1.5 1.5 0 0 1 -1.5 1.5h-9a1.5 1.5 0 0 1 -1.5 -1.5v-9a1.5 1.5 0 0 1 1.5 -1.5zM10 6 6 10M6 6.5V10h3.5")];
    /// Properties.
    Properties = 16 [S("M3 4h10M3 8h10M3 12h6")];
    /// Building structure.
    Structure = 16 [S("M2 13h12M3.5 13V6l4.5-3 4.5 3v7")];
    /// Architecture task area.
    Home = 16 [S("M2 7.5 8 2.5l6 5M3.5 6.5v7h9v-7M6.8 13.5V10h2.4v3.5")];
    /// Draft task area (pencil).
    Draft = 16 [S("M10.5 2.5l3 3-8 8H2.5v-3zM9 4l3 3")];
    /// Engineering task area (I-beam).
    Engineering = 16 [S("M3 3h10M3 13h10M8 3v10")];
    /// Building services task area (pipe elbow and bolt).
    Services = 16 [S("M2 13.5h3.5V7a2 2 0 0 1 2-2h6.5M2 11.5v4M14 3v4"), S("M11.5 8.5 9.5 11h3l-2 3")];
    /// Surroundings task area (tree).
    Surroundings = 16 [S("M8 14v-3M8 2 4 8h2.5L4 11h8L9.5 8H12z")];
    /// Layers.
    Layers = 16 [S("M8 2 14 5 8 8 2 5z"), S("M2 8l6 3 6-3M2 11l6 3 6-3")];
    /// AI chat.
    Chat = 16 [S("M2.5 3.5h11v7h-6l-3 2.5v-2.5h-2z")];
    /// Send.
    Send = 16 [S("M8 13V3M3.5 7.5 8 3l4.5 4.5")];
    /// AI set-up.
    Sparkle = 18 [S("M9 2v3M9 13v3M2 9h3M13 9h3M4 4l2 2M12 12l2 2M14 4l-2 2M6 12l-2 2")];
    /// Visible.
    Eye = 16 [S("M1.5 8s2.4-4.5 6.5-4.5S14.5 8 14.5 8s-2.4 4.5-6.5 4.5S1.5 8 1.5 8z"), S("M6 8a2 2 0 1 0 4 0a2 2 0 1 0 -4 0z")];
    /// Hidden.
    EyeClosed = 16 [S("M2 6.5s2.2 3.5 6 3.5 6-3.5 6-3.5M4 9l-1.2 1.6M8 10v2M12 9l1.2 1.6")];
    /// Locked.
    Lock = 16 [S("M5 7h6a1.5 1.5 0 0 1 1.5 1.5v3.5a1.5 1.5 0 0 1 -1.5 1.5h-6a1.5 1.5 0 0 1 -1.5 -1.5v-3.5a1.5 1.5 0 0 1 1.5 -1.5z"), S("M5.5 7V5a2.5 2.5 0 0 1 5 0v2")];
    /// Unlocked.
    Unlock = 16 [S("M5 7h6a1.5 1.5 0 0 1 1.5 1.5v3.5a1.5 1.5 0 0 1 -1.5 1.5h-6a1.5 1.5 0 0 1 -1.5 -1.5v-3.5a1.5 1.5 0 0 1 1.5 -1.5z"), S("M5.5 7V5a2.5 2.5 0 0 1 4.9-.7")];
    /// Folder.
    Folder = 16 [S("M2 4.5a1 1 0 0 1 1-1h3l1.5 1.5H13a1 1 0 0 1 1 1V12a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1z")];
    /// Drawing file (sheet).
    Drawing = 16 [S("M4 2h5.5L12 4.5V14H4zM9.5 2v2.5H12")];
    /// Storey.
    Storey = 16 [S("M2 4h12v8H2zM2 8h12")];
    /// Floor plan view.
    Plan = 16 [S("M3 2h10a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1zM2 8h7V2")];
    /// Section view.
    SectionView = 16 [D("M3 2v12M13 2v12M3 8h10", 2.0, 1.5)];
    /// 3D view.
    Cube = 16 [S("M8 1.5 14 5v6l-6 3.5L2 11V5z"), S("M2 5l6 3.5L14 5M8 8.5v6")];
    /// Perspective.
    Perspective = 16 [S("M2 4l5 2v8L2 12zM14 4l-5 2v8l5-2z")];
    /// Edges.
    Edges = 16 [S("M3 3h10v10H3z")];
    /// Rooms (air volumes).
    Rooms = 16 [D("M3 3h10v10H3z", 2.0, 1.5)];
    /// Walk through the building (first person).
    Walk = 16 [
        S("M8.2 2.6a1.3 1.3 0 1 0 2.6 0a1.3 1.3 0 1 0 -2.6 0"),
        S("M9 5.5L7.8 10l2.4 2.2v2.6M7.8 10l-1.3 2.2-2.2 2.4M9 5.5l2.2 2 2 .8M9 5.5l-2.4 1.4-1 2.2"),
    ];
    /// Horizontal cut.
    Cut = 16 [D("M2 9h12", 2.0, 1.5), S("M4 3h8a1 1 0 0 1 1 1v8a1 1 0 0 1 -1 1h-8a1 1 0 0 1 -1 -1v-8a1 1 0 0 1 1 -1z")];
    /// Open the ceiling: a slab lifted off a storey, an arrow up.
    OpenCeiling = 16 [S("M2 12.5h12M2 12.5v1.5M14 12.5v1.5"), S("M2 8.5l2.5-2h9.5l-2.5 2zM2 8.5v1.2h9.5V8.5M11.5 9.7l2.5-2V6.5"), S("M8 5V1.5M6.3 3.2 8 1.5l1.7 1.7")];
    /// Flip a side (two opposite arrows).
    Flip = 16 [S("M2.5 5.5h10M10 3l2.5 2.5L10 8M13.5 10.5h-10M6 8l-2.5 2.5L6 13")];
    /// Grid.
    Grid = 16 [W("M2 2h12v12H2zM6 2v12M10 2v12M2 6h12M2 10h12", 0.8)];
    /// Pen widths.
    PenWidths = 16 [W("M2 4h12", 0.6), S("M2 8h12"), W("M2 12h12", 1.9)];
    /// History.
    History = 16 [S("M2 8a6 6 0 1 0 12 0a6 6 0 1 0 -12 0z"), S("M8 5v3l2 1.5")];
    /// Library.
    Library = 16 [S("M8 4C6.5 3 4.5 2.8 2.5 3.2v9.3c2-.4 4-.2 5.5.8 1.5-1 3.5-1.2 5.5-.8V3.2C11.5 2.8 9.5 3 8 4zM8 4v9.3")];
    /// Objects.
    Objects = 16 [S("M2.5 2.5H7V7H2.5zM9 2.5h4.5V7H9zM2.5 9H7v4.5H2.5zM9 9h4.5v4.5H9z")];
    /// Sort ascending.
    SortAscending = 16 [S("M5 12V4M2.5 6.5 5 4l2.5 2.5M9 5h4.5M9 8h3.5M9 11h2")];
    /// Sort descending.
    SortDescending = 16 [S("M5 4v8M2.5 9.5 5 12l2.5-2.5M9 5h2M9 8h3.5M9 11h4.5")];
    /// Drag handle.
    DragHandle = 16 [W("M6 4h.01M10 4h.01M6 8h.01M10 8h.01M6 12h.01M10 12h.01", 1.6)];
    /// Pin.
    Pin = 16 [S("M9.5 2.5l4 4-2 1-2.5 2.5.5 2.5-1 1-5.5-5.5 1-1 2.5.5L9 5.5zM2.5 13.5l3-3")];
    /// Download.
    Download = 16 [S("M8 2.5v8M4.5 7 8 10.5 11.5 7M3 13.5h10")];
    /// Upload.
    Upload = 16 [S("M8 10.5v-8M4.5 6 8 2.5 11.5 6M3 13.5h10")];
    /// Refresh.
    Refresh = 16 [S("M13 8a5 5 0 1 1-1.5-3.6M13 2.5V5h-2.5")];
    /// Language.
    Globe = 16 [S("M2 8a6 6 0 1 0 12 0a6 6 0 1 0 -12 0z"), S("M2 8h12M8 2c-2.2 2-2.2 10 0 12M8 2c2.2 2 2.2 10 0 12")];
    /// Favourite.
    Star = 16 [S("M8 2l1.8 3.8 4.2.5-3.1 2.9.8 4.1L8 11.3l-3.7 2 .8-4.1L2 6.3l4.2-.5z")];
    /// Dock left.
    DockLeft = 16 [S("M3 2h10a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1z"), F("M3 3h4v10H3z")];
    /// Dock right.
    DockRight = 16 [S("M3 2h10a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1z"), F("M9 3h4v10H9z")];
    /// Dock top.
    DockTop = 16 [S("M3 2h10a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1z"), F("M3 3h10v4H3z")];
    /// Dock bottom.
    DockBottom = 16 [S("M3 2h10a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1z"), F("M3 9h10v4H3z")];
    /// Dock as a tab.
    DockCenter = 16 [S("M3 2h10a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-10a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1z"), F("M5 5h6v6H5z")];
}

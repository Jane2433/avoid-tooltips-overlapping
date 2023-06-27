/// A drawing cicle on the panel
pub struct Marker {
    /// Position
    pub pos: egui::Pos2,

    /// Radius of the Marker
    pub radius: f32,

    /// Color
    pub color: egui::Color32,

    /// Tooltop
    pub tooltip: Option<Tooltip>,
}

/// The tooltip belong to the Marker
pub struct Tooltip {
    /// Position
    pub rect: egui::Rect,

    /// content
    pub content: String,
}

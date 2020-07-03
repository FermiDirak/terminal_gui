#[derive(Debug, Clone)]
pub struct Container {
    /// 1 indexed x start position
    pub x: u16,
    /// 1 indexed y start position
    pub y: u16,
    /// container width
    pub width: u16,
    /// container height
    pub height: u16,
}

impl Default for Container {
    fn default() -> Container {
        Container {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}

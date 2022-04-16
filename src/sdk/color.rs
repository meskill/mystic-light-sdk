pub type SingleColor = u32;

/// Represent RGB color
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    pub red: SingleColor,
    pub green: SingleColor,
    pub blue: SingleColor,
}

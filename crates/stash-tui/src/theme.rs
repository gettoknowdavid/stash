use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct Theme {
    pub border: Color,
    pub border_focused: Color,
    pub error: Color,
    pub selected: Style,
    pub low_stock: Style,
    pub ok_stock: Style,
    pub status_bar: Style,
}
impl Theme {
    pub fn dark() -> Self {
        Self {
            border: Color::White,
            border_focused: Color::Yellow,
            error: Color::Red,
            selected: Style::default().bg(Color::LightBlue),
            low_stock: Style::default().bg(Color::Red),
            ok_stock: Style::default().bg(Color::DarkGray),
            status_bar: Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC),
        }
    }
    pub fn light() -> Self {
        Self {
            border: Color::Black,
            border_focused: Color::Blue,
            selected: Style::default().bg(Color::LightBlue),
            low_stock: Style::default().fg(Color::Red),
            ok_stock: Style::default().fg(Color::DarkGray),
            error: Color::Red,
            status_bar: Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC),
        }
    }
    pub fn border_style(&self, focused: bool) -> Style {
        Style::default().fg(if focused { self.border_focused } else { self.border })
    }
}
impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

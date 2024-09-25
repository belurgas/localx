use ratatui::{symbols::border::THICK, widgets::{Block, Borders, Paragraph, Widget}};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Settings {
    selected: usize
}


impl Widget for &Settings {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new("Not Realized")
        .block(Block::default().border_set(THICK).borders(Borders::ALL).title("Settings"))
        .centered()
        .render(area, buf);
    }
}


// Перенести в имплементации
impl Settings {
    pub fn new() -> Settings {
        Settings {
            selected: 0,
        }
    }
}

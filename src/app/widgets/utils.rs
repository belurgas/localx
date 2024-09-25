use ratatui::{symbols::border::THICK, widgets::{Block, Borders, Paragraph, Widget}};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Utils {
    selected: usize
}

impl Widget for &Utils {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new("Not Realized")
        .block(Block::default().border_set(THICK).borders(Borders::ALL).title("Utils"))
        .centered()
        .render(area, buf);
    }
}

// Перенести в имплементации
impl Utils {
    pub fn new() -> Utils {
        Utils {
            selected: 0,
        }
    }
}

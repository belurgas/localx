use ratatui::{symbols::border::THICK, widgets::{Block, Borders, Paragraph, Widget}};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CrossCompile {
    selected: usize
}

impl Widget for &CrossCompile {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new("Not Realized. Need renaming")
        .block(Block::default().border_set(THICK).borders(Borders::ALL).title("CrossCompile"))
        .centered()
        .render(area, buf);
    }
}

// Перенести в имплементации
impl CrossCompile {
    pub fn new() -> CrossCompile {
        CrossCompile {
            selected: 0,
        }
    }
}

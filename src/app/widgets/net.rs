use pnet::datalink::NetworkInterface;
use ratatui::{layout::{Alignment, Constraint, Layout}, style::{Color, Modifier, Style, Stylize}, symbols::border::THICK, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, Paragraph, Widget}};


#[derive(Debug, PartialEq, Eq)]
pub struct NetWidget {
    pub interfaces: NetworkInterface,
    pub selected: usize,
    pub items: Vec<String>,
}

impl Widget for &NetWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let [command_area, info_area] = Layout::horizontal([
            Constraint::Percentage(10),
            Constraint::Percentage(90),
        ]).areas(area);

        self.create_menu().render(command_area, buf);

        Paragraph::new(self.create_lines())
        .block(Block::default().border_set(THICK).borders(Borders::ALL).title("NetSystem"))
        .gray()
        .render(info_area, buf);

    }
}

impl NetWidget {
    pub fn new(interface: NetworkInterface) -> NetWidget {
        NetWidget {
            interfaces: interface,
            selected: 0,
            items: vec!["Info".to_string(),
                    "Firewall".to_string(),
                    "Scaner".to_string(),
                    "Monitor".to_string()],
        }
    }

    pub fn create_lines(&self) -> Vec<Line<'static>> {
        let overhead = Span::styled("Управление внутренней сетью", Style::default().add_modifier(Modifier::BOLD).fg(Color::Green));
        let interface_data = self.get_interface_info();
        let widget_data = vec![
            Line::from(overhead).alignment(Alignment::Center),
            Line::raw(format!("Имя: {}", interface_data[0])),
            Line::raw(format!("Описание: {}", interface_data[1])),
            Line::raw(format!("Индес: {}", interface_data[2])),
            Line::raw(format!("MAC-адрес: {}", interface_data[3])),
        ];
        widget_data
    }

    fn get_interface_info(&self) -> Vec<String> {
        let inter = self.interfaces.clone();

        let mac = if let Some(mac) = inter.mac {
            format!("{:?}", mac)
        } else {
            format!("Mac-адрес не обнаружен")
        };

        let info: Vec<String> = vec![
            inter.name,
            inter.description,
            format!("{}", inter.index),
            mac,
        ];
        info
    }

    pub fn create_menu(&self) -> List<'static> {
        let items: Vec<ListItem> = self.items.iter()
            .enumerate().map(|(i, item)| {
                let style = if i == self.selected {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(item.clone()).style(style)
            }).collect();
        List::new(items)
            .block(Block::default().border_set(THICK).borders(Borders::ALL).border_style(Style::default().fg(Color::LightYellow)).title("NetworkMenu"))
            .highlight_style(Style::default().bg(Color::Yellow))
            .highlight_symbol(">")
    }

    pub fn next(&mut self) {
        if self.selected >= self.items.len() - 1 {
            self.selected = 0;
        } else {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.items.len() - 1;
        }
    }
}

// use std::rc::Rc;

use figlet_rs::FIGfont;
use ratatui::{
    buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style}, symbols::border::THICK, widgets::{Block, Borders, List, ListItem, Paragraph, Widget}, Frame
};

use crate::app::{handlers::time_compile, net::base, widgets::app::{App, KeyboardEvent, RunningState, States, WidgetKind}};

// Меню (Добавить FIREWALL)
const MENU_ITEMS: [&str; 4] = ["CrossCompile", "Net", "Utils", "Settings"];

// Имплементируем рендеринг виджета
impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Разделение окна
        let splited_area =
            Layout::horizontal([Constraint::Percentage(15), Constraint::Percentage(85)])
                .split(area);

        // Созданме меню
        self.crate_menu().render(splited_area[0], buf);


        // Отображение flf
        let widgets_area = Layout::vertical([
            Constraint::Length(7),
            Constraint::Min(0),
        ]).split(splited_area[1]);
        let fig_default = FIGfont::standard().unwrap();
        let fig = FIGfont::from_file(r"C:\Program Files\LocalX\Fonts\ANSI.flf").unwrap_or(fig_default);
        let text = format!("{}", fig.convert("NETSEC").unwrap());
        Paragraph::new(text).centered().block(Block::default().border_set(THICK).borders(Borders::ALL)).render(widgets_area[0], buf);

        //  Виджеты под меню (Здесь как раз надо сделать обработчик выбора виджета, то-есть в зависимости от того какой виджет бдует рендериться именно этот виджет поверх дефолтного)
        // TEST WIDGET
        Paragraph::new("PRE-BUILD VERSION").centered().block(Block::default().border_set(THICK).borders(Borders::ALL)).render(widgets_area[1], buf);
    }
}

impl<'a> App<'a> {
    // Методы App
    pub fn new() -> App<'a> {
        App {
            items: MENU_ITEMS,
            selected: 0,
            running_state: RunningState::Running,
            state: States::new(),
        }
    }

    // Вниз
    pub fn next(&mut self) {
        if self.selected >= self.items.len() - 1 {
            self.selected = 0;
        } else {
            self.selected += 1;
        }
    }

    // Вверх
    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.items.len() - 1;
        }
    }

    // Главная функци отображения всего приложения
    pub fn view(&mut self, frame: &mut Frame) {
        frame.render_widget(&*self, frame.area());
    }

    // Создание меню
    pub fn crate_menu(&self) -> List<'_> {
        let items: Vec<ListItem> = self.items.iter()
            .enumerate().map(|(i, item)| {
                let style = if i == self.selected {
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(*item).style(style)
            }).collect();
        let list = List::new(items)
            .block(Block::default().border_set(THICK).borders(Borders::ALL).title("LocalX"))
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol(">");
        list
    }

    // Обновленеи состояний KeyboardEvent matching
    pub fn update(&mut self, event: KeyboardEvent) -> Option<KeyboardEvent> {
        match event {
            KeyboardEvent::Up => {
                // Пока перемещаемся по меню
                self.previous();
            }

            KeyboardEvent::Down => {
                self.next();
            }

            // Воход в виджет, иная реализация нежеле в первой версии
            KeyboardEvent::Enter => {
                self.state.widget_active = true;

                let data = time_compile::get_os_info();
                match self.selected {
                    0 => self.state.widget_kind = WidgetKind::CrossCompile
                        {   os_name: data.0,
                            os_version: data.1,
                            os_bitness: data.2,
                            os_arch: data.3     },
                    1 => self.state.widget_kind = WidgetKind::Net { interfaces: base::get_interface() },
                    2 => self.state.widget_kind = WidgetKind::Utils,
                    3 => self.state.widget_kind = WidgetKind::Settings,
                    _ => panic!("Ошибка определения состояния виджета"),

                }
            }
            KeyboardEvent::Out => {
                self.state.widget_active = false;

                match self.state.widget_kind {
                    WidgetKind::NoneActive => {},
                    WidgetKind::CrossCompile { os_name: _, os_version: _, os_bitness: _, os_arch: _ } => {
                        self.selected = 0;
                    },
                    WidgetKind::Net { interfaces: _ } => {
                        self.selected = 1;
                    },
                    WidgetKind::Utils => {
                        self.selected = 2;
                    },
                    WidgetKind::Settings => {
                        self.selected = 3;
                    },
                }
            }
            // Выход из приложения
            KeyboardEvent::Quit => self.running_state = RunningState::Done,
        };
        None
    }
}

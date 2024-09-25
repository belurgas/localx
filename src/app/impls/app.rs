// use std::rc::Rc;

use figlet_rs::FIGfont;
use ratatui::{
    buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{Color, Modifier, Style}, symbols::border::THICK, text::Text, widgets::{Block, Borders, List, ListItem, Paragraph, Widget}, Frame
};

use crate::app::{net::base, widgets::{app::{App, KeyboardEvent, RunningState, States, WidgetKind}, cross_compile::CrossCompile, map::MapS, net::NetWidget, settings::Settings, utils::Utils}};

// Меню (Добавить FIREWALL)
const MENU_ITEMS: [&str; 5] = ["Manual", "CrossCompile", "Net", "Utils", "Settings"];

// Имплементируем рендеринг виджета
impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Разделение окна
        let splited_area =
            Layout::horizontal([Constraint::Length(14), Constraint::Min(0)])
                .split(area);

        // Созданме меню
        self.crate_menu(self.state.widget_active).render(splited_area[0], buf);


        // Отображение flf
        let widgets_area = Layout::vertical([
            Constraint::Length(7),
            Constraint::Min(0),
        ]).split(splited_area[1]);
        let fig_default = FIGfont::standard().unwrap();
        let fig = FIGfont::from_file(r"C:\Program Files\LocalX\Fonts\ANSI.flf").unwrap_or(fig_default);
        let text = Text::styled(format!("{}", fig.convert("DewLor").unwrap()), Style::default().fg(Color::LightMagenta));
        Paragraph::new(text).centered().block(Block::default().border_set(THICK).borders(Borders::ALL)).render(widgets_area[0], buf);

        //  Виджеты под меню (Здесь как раз надо сделать обработчик выбора виджета, то-есть в зависимости от того какой виджет бдует рендериться именно этот виджет поверх дефолтного)
        match &self.state.widget_kind {
            WidgetKind::NoneActive => {
                Paragraph::new("PRE-BUILD VERSION").centered().block(Block::default().border_set(THICK).borders(Borders::ALL)).render(widgets_area[1], buf);
            },
            WidgetKind::CrossCompile(widget) => {
                todo!();
            },
            WidgetKind::Net(widget) => {
                widget.render(widgets_area[1], buf);
            },
            WidgetKind::Utils(widget) => {
                let a = MapS::new();
                a.render(widgets_area[1], buf);

            },
            WidgetKind::Settings(widget) => {
                todo!();
            },
        }

        // // TEST WIDGET
        // Paragraph::new("PRE-BUILD VERSION").centered().block(Block::default().border_set(THICK).borders(Borders::ALL)).render(widgets_area[1], buf);
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
    pub fn crate_menu(&self, active: bool) -> List<'_> {
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
        let list_style = if active {
            Style::default()
        } else {
            Style::default().fg(Color::LightYellow)
        };
        let list = List::new(items)
            .block(Block::default().border_set(THICK).borders(Borders::ALL).border_style(list_style).title("LocalX"))
            .highlight_symbol(">");
        list
    }

    pub fn hande_widget_up(&mut self) {
        match &mut self.state.widget_kind {
            WidgetKind::NoneActive => {},
            WidgetKind::CrossCompile(cross_compile) => {},
            WidgetKind::Net(net_widget) => net_widget.previous(),
            WidgetKind::Utils(utils) => {},
            WidgetKind::Settings(settings) => {},
        }
    }

    pub fn handle_widget_down(&mut self) {
        match &mut self.state.widget_kind {
            WidgetKind::NoneActive => {},
            WidgetKind::CrossCompile(cross_compile) => todo!(),
            WidgetKind::Net(net_widget) => net_widget.next(),
            WidgetKind::Utils(utils) => {},
            WidgetKind::Settings(settings) => {},
        }
    }

    // Обновленеи состояний KeyboardEvent matching
    pub fn update(&mut self, event: KeyboardEvent) -> Option<KeyboardEvent> {
        match event {
            KeyboardEvent::Up => {
                if !self.state.widget_active {
                    self.previous();
                } else {
                    self.hande_widget_up();
                }
            }

            KeyboardEvent::Down => {
                if !self.state.widget_active {
                    self.next();
                } else {
                    self.handle_widget_down();
                }
            }

            // Воход в виджет, иная реализация нежеле в первой версии
            KeyboardEvent::Enter => {
                let active_state_after_enter = &self.state.widget_kind;
                if active_state_after_enter == &WidgetKind::NoneActive {
                    self.state.widget_active = true;
                    match self.selected {
                        0 => {},
                        1 => self.state.widget_kind = WidgetKind::CrossCompile(CrossCompile::new()),
                        2 => self.state.widget_kind = WidgetKind::Net(NetWidget::new(base::get_interface())),
                        3 => self.state.widget_kind = WidgetKind::Utils(Utils::new()),
                        4 => self.state.widget_kind = WidgetKind::Settings(Settings::new()),
                        _ => panic!("Ошибка определения состояния виджета"),
                    }
                } else {
                    println!("Бублик");
                }
            }
            KeyboardEvent::Out => {
                self.state.widget_active = false;
                match &self.state.widget_kind {
                    WidgetKind::NoneActive => {},
                    WidgetKind::CrossCompile(widget) => {
                        self.selected = 1;
                        self.state.widget_kind = WidgetKind::NoneActive;
                    },
                    WidgetKind::Net(widget) => {
                        self.selected = 2;
                        self.state.widget_kind = WidgetKind::NoneActive;
                    },
                    WidgetKind::Utils(widget) => {
                        self.selected = 3;
                        self.state.widget_kind = WidgetKind::NoneActive;
                    },
                    WidgetKind::Settings(widget) => {
                        self.selected = 4;
                        self.state.widget_kind = WidgetKind::NoneActive;
                    },
                }
            }
            // Выход из приложения
            KeyboardEvent::Quit => self.running_state = RunningState::Done,
        };
        None
    }
}

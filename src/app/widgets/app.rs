use pnet::datalink::NetworkInterface;

use super::{cross_compile::CrossCompile, net::NetWidget, settings::Settings, utils::Utils};

// Основной виджет
#[derive(Debug)]
pub struct App<'a> {
    pub items: [&'a str; 5],
    pub selected: usize,

    pub running_state: RunningState,
    pub state: States,
}

// Перечесление внешних состояний. Используется только для App стрктуры
#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, PartialEq, Eq)]
pub struct States {
    pub widget_active: bool,     // Активен ли виджет или нет
    pub widget_kind: WidgetKind, // Тип виджета
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum WidgetKind {
    #[default]
    NoneActive, // Нет активного виджета
    CrossCompile(CrossCompile),
    Net(NetWidget),
    Utils(Utils),
    Settings(Settings),
}

#[derive(PartialEq)]
pub enum KeyboardEvent {
    Up,
    Down,
    Enter,
    Out,
    Quit,
}

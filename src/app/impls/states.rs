use crate::app::widgets::{app::{States, WidgetKind}, net::NetWidget};

impl States {
    pub fn new() -> States {
        States {
            widget_active: false,
            widget_kind: WidgetKind::NoneActive,
        }
    }
}

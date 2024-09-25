use crate::app::widgets::app::{States, WidgetKind};

impl States {
    pub fn new() -> States {
        States {
            widget_active: false,
            widget_kind: WidgetKind::NoneActive,
        }
    }
}

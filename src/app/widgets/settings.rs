#[derive(Debug, Default, PartialEq, Eq)]
pub struct Settings {
    selected: usize
}


// Перенести в имплементации
impl Settings {
    pub fn new() -> Settings {
        Settings {
            selected: 0,
        }
    }
}

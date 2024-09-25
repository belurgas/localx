#[derive(Debug, Default, PartialEq, Eq)]
pub struct Utils {
    selected: usize
}


// Перенести в имплементации
impl Utils {
    pub fn new() -> Utils {
        Utils {
            selected: 0,
        }
    }
}

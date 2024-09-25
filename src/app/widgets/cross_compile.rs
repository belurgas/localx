#[derive(Debug, Default, PartialEq, Eq)]
pub struct CrossCompile {
    selected: usize
}

// Перенести в имплементации
impl CrossCompile {
    pub fn new() -> CrossCompile {
        CrossCompile {
            selected: 0,
        }
    }
}

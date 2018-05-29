#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    Red,
    Black,
}


impl Color {
    pub fn other(&self) -> Color {
        match *self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }
}

pub trait Edge {
    fn from(&self) -> usize;
    fn to(&self) -> usize;
    fn color(&self) -> u8;
    fn set_color(&mut self, color: u8);
}

pub trait EdgeConstructor {
    fn new(from: usize, to: usize) -> Self;
    fn new_with_colour(from: usize, to: usize, colour: u8) -> Self;
}

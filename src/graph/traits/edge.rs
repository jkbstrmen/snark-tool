pub trait Edge {
    fn new(from: usize, to: usize) -> Self;
    fn from(&self) -> usize;
    fn to(&self) -> usize;
    fn color(&self) -> u8;
    fn set_color(&mut self, color: u8);
}
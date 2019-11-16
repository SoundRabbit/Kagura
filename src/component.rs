pub trait Composable<T> {
    fn render(&mut self) -> T;
}

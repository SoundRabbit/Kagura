pub trait BasicComponent<T> {
    fn render(&mut self) -> T;
}

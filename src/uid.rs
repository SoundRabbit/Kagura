use std::cell::Cell;

thread_local!(static COUNT: Cell<IdType> = Cell::new(0));

#[allow(dead_code)]
pub type IdType = u64;

#[allow(dead_code)]
pub fn get() -> IdType {
    let count = COUNT.with(|count| {
        let c = count.get();
        count.set(c + 1);
        c
    });
    count
}

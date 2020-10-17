use std::cell::Cell;

thread_local!(static COUNT: Cell<u32> = Cell::new(0));

pub fn get() -> u32 {
    let count = COUNT.with(|count| {
        let c = count.get();
        count.set(c + 1);
        c
    });
    count
}

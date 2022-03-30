use std::collections::{HashMap, VecDeque};

enum Trace {
    Replace,
    Append,
    Remove,
    Keep,
}

pub enum Edit<X, Y> {
    Replace(X, Y),
    Append(Y),
    Remove(X),
    Keep(X, Y),
}

pub fn mix<X, Y>(
    mut xs: VecDeque<X>,
    mut ys: VecDeque<Y>,
    mut is_same: impl FnMut(&X, &Y) -> bool,
    replace_cost: f64,
    append_cost: f64,
    remove_cost: f64,
) -> VecDeque<Edit<X, Y>> {
    if xs.len() == 0 {
        let mut res = VecDeque::new();
        for y in ys {
            res.push_back(Edit::Append(y));
        }

        return res;
    }

    if ys.len() == 0 {
        let mut res = VecDeque::new();
        for x in xs {
            res.push_back(Edit::Remove(x));
        }

        return res;
    }

    let mut d: HashMap<[i32; 2], (f64, Trace)> = HashMap::new();

    for i in 0..xs.len() {
        let ii = i as i32;
        d.insert([ii, -1], (ii as f64 * remove_cost, Trace::Remove));
    }

    for i in 0..ys.len() {
        let ii = i as i32;
        d.insert([-1, ii], (ii as f64 * append_cost, Trace::Append));
    }

    d.insert([-1, -1], (0.0, Trace::Replace));

    for xi in 0..xs.len() {
        for yi in 0..ys.len() {
            let xii = xi as i32;
            let yii = yi as i32;
            let mut is_keep = false;
            let replace = if is_same(&xs.get(xi).unwrap(), &ys.get(yi).unwrap()) {
                is_keep = true;
                d.get(&[xii - 1, yii - 1]).unwrap().0
            } else {
                d.get(&[xii - 1, yii - 1]).unwrap().0 + replace_cost
            };
            let append = d.get(&[xii, yii - 1]).unwrap().0 + append_cost;
            let remove = d.get(&[xii - 1, yii]).unwrap().0 + remove_cost;

            if replace <= append && replace <= remove {
                if is_keep {
                    d.insert([xii, yii], (replace, Trace::Keep));
                } else {
                    d.insert([xii, yii], (replace, Trace::Replace));
                }
            } else if append <= remove {
                d.insert([xii, yii], (append, Trace::Append));
            } else {
                d.insert([xii, yii], (remove, Trace::Remove));
            }
        }
    }

    d.remove(&[-1, -1]);

    let mut res = VecDeque::new();
    let (mut xii, mut yii) = (xs.len() as i32 - 1, ys.len() as i32 - 1);
    while let Some((_, op)) = d.get(&[xii, yii]) {
        match op {
            Trace::Replace => {
                if let (Some(x), Some(y)) = (xs.pop_back(), ys.pop_back()) {
                    res.push_front(Edit::Replace(x, y));
                }
                xii -= 1;
                yii -= 1;
            }
            Trace::Keep => {
                if let (Some(x), Some(y)) = (xs.pop_back(), ys.pop_back()) {
                    res.push_front(Edit::Keep(x, y));
                }
                xii -= 1;
                yii -= 1;
            }
            Trace::Append => {
                if let Some(y) = ys.pop_back() {
                    res.push_front(Edit::Append(y));
                }
                yii -= 1;
            }
            Trace::Remove => {
                if let Some(x) = xs.pop_back() {
                    res.push_front(Edit::Remove(x));
                }
                xii -= 1;
            }
        }
    }

    res
}

use std::collections::{HashMap, VecDeque};

enum Edit {
    Replace,
    Append,
    Remove,
}

pub fn diff_mix<X, Y>(
    mut xs: VecDeque<X>,
    mut ys: VecDeque<Y>,
    mut is_same: impl FnMut(&X, &Y) -> bool,
    replace_cost: f64,
    append_cost: f64,
    remove_cost: f64,
) -> VecDeque<(Option<X>, Option<Y>)> {
    if xs.len() == 0 {
        let mut res = VecDeque::new();
        for y in ys {
            res.push_back((None, Some(y)));
        }

        return res;
    }

    if ys.len() == 0 {
        let mut res = VecDeque::new();
        for x in xs {
            res.push_back((Some(x), None));
        }

        return res;
    }

    let mut d: HashMap<[i32; 2], (f64, Edit)> = HashMap::new();

    for i in 0..xs.len() {
        let ii = i as i32;
        d.insert([ii, -1], (ii as f64 * remove_cost, Edit::Remove));
    }

    for i in 0..ys.len() {
        let ii = i as i32;
        d.insert([-1, ii], (ii as f64 * append_cost, Edit::Append));
    }

    d.insert([-1, -1], (0.0, Edit::Replace));

    for xi in 0..xs.len() {
        for yi in 0..ys.len() {
            let xii = xi as i32;
            let yii = yi as i32;
            let replace = if is_same(&xs.get(xi).unwrap(), &ys.get(yi).unwrap()) {
                d.get(&[xii - 1, yii - 1]).unwrap().0
            } else {
                d.get(&[xii - 1, yii - 1]).unwrap().0 + replace_cost
            };
            let append = d.get(&[xii, yii - 1]).unwrap().0 + append_cost;
            let remove = d.get(&[xii - 1, yii]).unwrap().0 + remove_cost;

            if replace <= append && replace <= remove {
                d.insert([xii, yii], (replace, Edit::Replace));
            } else if append <= remove {
                d.insert([xii, yii], (append, Edit::Append));
            } else {
                d.insert([xii, yii], (remove, Edit::Remove));
            }
        }
    }

    d.remove(&[-1, -1]);

    let mut res = VecDeque::new();
    let (mut xii, mut yii) = (xs.len() as i32 - 1, ys.len() as i32 - 1);
    while let Some((_, op)) = d.get(&[xii, yii]) {
        match op {
            Edit::Replace => {
                res.push_front((xs.pop_back(), ys.pop_back()));
                xii -= 1;
                yii -= 1;
            }
            Edit::Append => {
                res.push_front((None, ys.pop_back()));
                yii -= 1;
            }
            Edit::Remove => {
                res.push_front((xs.pop_back(), None));
                xii -= 1;
            }
        }
    }

    res
}

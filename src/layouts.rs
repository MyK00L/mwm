pub fn get_layout(n: usize, size: (i32, i32)) -> Vec<((i32, i32), (i32, i32))> {
    let mut res: Vec<((i32, i32), (i32, i32))> = vec![((0, 0), (0, 0)); n];
    if n > 0 {
        res[0] = ((0, 0), (size.0 / 2, size.1));
    }
    for i in 1..n {
        if i % 2 == 1 {
            res[i] = (
                ((res[i - 1].0).0 + (res[i - 1].1).0, (res[i - 1].0).1),
                ((res[i - 1].1).0, (res[i - 1].1).1 / 2),
            );
        } else {
            res[i] = (
                ((res[i - 1].0).0, (res[i - 1].0).1 + (res[i - 1].1).1),
                ((res[i - 1].1).0 / 2, (res[i - 1].1).1),
            );
        }
    }
    res
}

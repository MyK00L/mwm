#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

pub fn get_layout(n: usize, size: (u32, u32)) -> Vec<Position> {
    if n == 0 {
        return Vec::<Position>::new();
    }
    let mut res: Vec<Position> = vec![
        Position {
            x: 0,
            y: 0,
            w: 0,
            h: 0
        };
        n
    ];
    res[0] = Position {
        x: 0,
        y: 0,
        w: size.0 / 2,
        h: size.1,
    };
    for i in 1..n {
        if i % 2 == 1 {
            res[i] = Position {
                x: res[i - 1].x + res[i - 1].w as i32,
                y: res[i - 1].y,
                w: res[i - 1].w,
                h: res[i - 1].h / 2,
            };
        } else {
            res[i] = Position {
                x: res[i - 1].x,
                y: res[i - 1].y + res[i - 1].h as i32,
                w: res[i - 1].w / 2,
                h: res[i - 1].h,
            };
        }
    }
    if n % 2 == 1 {
        res[n - 1].w *= 2;
    } else {
        res[n - 1].h *= 2;
    }
    res
}

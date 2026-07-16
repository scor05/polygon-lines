use raylib::prelude::*;

pub fn calc_bresenham(a: Vector2, b: Vector2) -> Vec<Vector2> {
    let x2 = b.x as i32;
    let x1 = a.x as i32;
    let y2 = b.y as i32;
    let y1 = a.y as i32;
    let mut res: Vec<Vector2> = Vec::new();

    if x1 == x2 {
        for i in y1..=y2 {
            res.push(Vector2::new(x1 as f32, i as f32));
        }
    } else if y1 == y2 {
        for i in x1..=x2 {
            res.push(Vector2::new(i as f32, y1 as f32));
        }
    } else {
        bresenham_x(&mut res, x1, x2, y1, y2);
    }

    return res;
}

pub fn bresenham_x(res: &mut Vec<Vector2>, x1: i32, x2: i32, y1: i32, y2: i32) {
    let m = 2 * (y2 - y1);
    let mut slope_err = m - (x2 - x1);
    let mut curr = Vector2::new(x1 as f32, y1 as f32);

    for _i in x1..=x2 {
        res.push(curr);
        slope_err += m;

        if slope_err >= 0 {
            curr.y += 1.0;
            slope_err -= 2 * (x2 - x1);
        } else {
            curr.y -= 1.0;
            slope_err += 2 * (x2 - x1);
        }

        curr.x += 1.0;
    }
}

use raylib::prelude::*;

pub fn calc_bresenham(a: Vector2, b: Vector2) -> Vec<Vector2> {
    let x2 = b.x as i32;
    let x1 = a.x as i32;
    let y2 = b.y as i32;
    let y1 = a.y as i32;
    let mut res: Vec<Vector2> = Vec::new();

    if x1 == x2 {
        let y_step = if y1 <= y2 { 1 } else { -1 };
        let mut i = y1;
        while i != y2 + y_step {
            res.push(Vector2::new(x1 as f32, i as f32));
            i += y_step;
        }
    } else if y1 == y2 {
        let x_step = if x1 <= x2 { 1 } else { -1 };
        let mut i = x1;
        while i != x2 + x_step {
            res.push(Vector2::new(i as f32, y1 as f32));
            i += x_step;
        }
    } else if (x2 - x1).abs() >= (y2 - y1).abs() {
        bresenham_x(&mut res, x1, x2, y1, y2);
    } else {
        bresenham_y(&mut res, x1, x2, y1, y2);
    }

    return res;
}

pub fn bresenham_x(res: &mut Vec<Vector2>, x1: i32, x2: i32, y1: i32, y2: i32) {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let x_step = if x1 <= x2 { 1 } else { -1 };
    let y_step = if y1 <= y2 { 1.0 } else { -1.0 };
    let mut slope_err = 2 * dy - dx;
    let mut curr = Vector2::new(x1 as f32, y1 as f32);
    let mut x = x1;

    while x != x2 + x_step {
        res.push(curr);

        if slope_err >= 0 {
            curr.y += y_step;
            slope_err -= 2 * dx;
        }

        slope_err += 2 * dy;
        curr.x += x_step as f32;
        x += x_step;
    }
}

pub fn bresenham_y(res: &mut Vec<Vector2>, x1: i32, x2: i32, y1: i32, y2: i32) {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let x_step = if x1 <= x2 { 1.0 } else { -1.0 };
    let y_step = if y1 <= y2 { 1 } else { -1 };
    let mut slope_err = 2 * dx - dy;
    let mut curr = Vector2::new(x1 as f32, y1 as f32);
    let mut y = y1;

    while y != y2 + y_step {
        res.push(curr);

        if slope_err >= 0 {
            curr.x += x_step;
            slope_err -= 2 * dy;
        }

        slope_err += 2 * dx;
        curr.y += y_step as f32;
        y += y_step;
    }
}

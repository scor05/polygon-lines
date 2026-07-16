use crate::bresenham::calc_bresenham;
use raylib::prelude::*;

pub fn draw_line(img: &mut Image, start: Vector2, end: Vector2, color: Color) {
    if start.x < 0.0
        || end.x < 0.0
        || start.y < 0.0
        || end.y < 0.0
        || start.x >= img.width as f32
        || end.x >= img.width as f32
        || start.y >= img.height as f32
        || end.y >= img.height as f32
    {
        eprintln!("Line input point out of bounds!");
        return;
    }

    let points = calc_bresenham(start, end);
    for point in points {
        img.draw_pixel(point.x as i32, point.y as i32, color);
    }
}

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

    // para siempre evaluar de izquierda a derecha las líneas
    let mut vec1: Vector2 = start;
    let mut vec2: Vector2 = end;
    if start.x > end.x || start.y > end.y {
        vec1 = end;
        vec2 = start;
    }

    let points = calc_bresenham(vec1, vec2);
    for point in points {
        img.draw_pixel(point.x as i32, point.y as i32, color);
    }
}

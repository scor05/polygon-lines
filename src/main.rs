mod bresenham;
mod line;

use line::draw_line;
use raylib::prelude::*;

fn main() {
    let width = 600;
    let height = 600;
    let background_color = Color::BLACK;
    let file_path = format!("{}/output.png", env!("CARGO_MANIFEST_DIR"));

    let mut img = Image::gen_image_color(width, height, background_color);

    let point1: Vector2 = Vector2::new(377.0, 249.0);
    let point2: Vector2 = Vector2::new(411.0, 197.0);
    let point3: Vector2 = Vector2::new(436.0, 249.0);
    draw_line(&mut img, point1, point2, Color::RED);
    draw_line(&mut img, point3, point2, Color::BLUE);
    draw_line(&mut img, point3, point1, Color::GREEN);

    img.export_image(&file_path);
    println!("Image output saved in {file_path}");
}

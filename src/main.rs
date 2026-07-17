mod bresenham;
mod line;
mod polygon;

use polygon::{draw_polygon, draw_polygon_with_holes, points};
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;
    let background_color = Color::BLACK;
    let file_path = format!("{}/out.png", env!("CARGO_MANIFEST_DIR"));
    let mut img = Image::gen_image_color(width, height, background_color);

    let polygon1 = points(&[
        165, 380, 185, 360, 180, 330, 207, 345, 233, 330, 230, 360, 250, 380, 220, 385, 205, 410,
        193, 383,
    ]);
    let polygon2 = points(&[321, 335, 288, 286, 339, 251, 374, 302]);
    let polygon3 = points(&[377, 249, 411, 197, 436, 249]);
    let polygon4 = points(&[
        413, 177, 448, 159, 502, 88, 553, 53, 535, 36, 676, 37, 660, 52, 750, 145, 761, 179, 672,
        192, 659, 214, 615, 214, 632, 230, 580, 230, 597, 215, 552, 214, 517, 144, 466, 180,
    ]);
    let polygon5 = points(&[682, 175, 708, 120, 735, 148, 739, 170]);

    draw_polygon(&mut img, &polygon1, Color::RED);
    draw_polygon(&mut img, &polygon2, Color::BLUE);
    draw_polygon(&mut img, &polygon3, Color::GREEN);
    draw_polygon_with_holes(&mut img, &polygon4, &[&polygon5], Color::RED);

    img.export_image(&file_path);
    println!("Image output saved in {file_path}");
}

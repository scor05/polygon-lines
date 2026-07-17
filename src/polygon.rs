use crate::line::draw_line;
use raylib::prelude::*;

pub fn points(coords: &[i32]) -> Vec<Vector2> {
    coords
        .chunks_exact(2)
        .map(|point| Vector2::new(point[0] as f32, point[1] as f32))
        .collect()
}

pub fn draw_polygon(img: &mut Image, polygon: &[Vector2], color: Color) {
    fill_polygon(img, &[polygon], color);
    draw_outline(img, polygon, color);
}

pub fn draw_polygon_with_holes(
    img: &mut Image,
    polygon: &[Vector2],
    holes: &[&[Vector2]],
    color: Color,
) {
    let mut contours = Vec::with_capacity(holes.len() + 1);
    contours.push(polygon);
    contours.extend_from_slice(holes);

    fill_polygon(img, &contours, color);
    draw_outline(img, polygon, color);

    for hole in holes {
        draw_outline(img, hole, color);
    }
}

fn fill_polygon(img: &mut Image, contours: &[&[Vector2]], color: Color) {
    let Some((min_y, max_y)) = y_bounds(contours) else {
        return;
    };

    for y in min_y..=max_y {
        let scan_y = y as f32 + 0.5;
        let mut intersections = Vec::new();

        for contour in contours {
            add_intersections(&mut intersections, contour, scan_y);
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for pair in intersections.chunks_exact(2) {
            let x_start = pair[0].ceil() as i32;
            let x_end = pair[1].floor() as i32;

            for x in x_start..=x_end {
                if x >= 0 && x < img.width && y >= 0 && y < img.height {
                    img.draw_pixel(x, y, color);
                }
            }
        }
    }
}

fn add_intersections(intersections: &mut Vec<f32>, polygon: &[Vector2], scan_y: f32) {
    if polygon.len() < 3 {
        return;
    }

    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];

        if scan_y >= p1.y.min(p2.y) && scan_y < p1.y.max(p2.y) {
            let x = p1.x + (scan_y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
            intersections.push(x);
        }
    }
}

fn draw_outline(img: &mut Image, polygon: &[Vector2], color: Color) {
    if polygon.len() < 2 {
        return;
    }

    for i in 0..polygon.len() {
        draw_line(img, polygon[i], polygon[(i + 1) % polygon.len()], color);
    }
}

fn y_bounds(contours: &[&[Vector2]]) -> Option<(i32, i32)> {
    let mut points = contours.iter().flat_map(|contour| contour.iter());
    let first = points.next()?;
    let mut min_y = first.y as i32;
    let mut max_y = first.y as i32;

    for point in points {
        min_y = min_y.min(point.y as i32);
        max_y = max_y.max(point.y as i32);
    }

    Some((min_y, max_y))
}

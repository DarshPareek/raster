use freetype as ft;
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::ffi::KeyboardKey;
use raylib::ffi::MouseButton;
use raylib::math::Vector2;
use raylib::prelude::*;
use std::cmp::Ordering;
use std::collections::VecDeque;
const WIDTH_FACTOR: i32 = 4;
const HEIGHT_FACTOR: i32 = 3;
const WINDOW_FACTOR: i32 = 300;
const WINDOW_WIDTH: i32 = WINDOW_FACTOR * WIDTH_FACTOR;
const WINDOW_HEIGHT: i32 = WINDOW_FACTOR * HEIGHT_FACTOR;
const GRID_FACTOR: i32 = 200;
const GRID_WIDTH: i32 = WIDTH_FACTOR * GRID_FACTOR;
const GRID_HEIGHT: i32 = HEIGHT_FACTOR * GRID_FACTOR;
const MARKER_SIZE: Vector2 = Vector2 { x: 25.0, y: 25.0 };
const BACKGROUD_COLOR: &str = "0D0C1D";
const BEZIER_COLOR: &str = "F1DAC4";
const MARKER_COLOR: &str = "474973";
const CELL_WIDTH: i32 = WINDOW_WIDTH / GRID_WIDTH;
const CELL_HEIGHT: i32 = WINDOW_HEIGHT / GRID_HEIGHT;
fn hexcolor(color: &str) -> Color {
    match Color::from_hex(color) {
        Ok(res) => {
            return res;
        }
        _ => {
            println!("Error with colors, giving white");
            return Color::WHITE;
        }
    }
}
fn render_marker(d: &mut RaylibDrawHandle, pos: Vector2, color: Color) {
    let begin: Vector2 = pos - MARKER_SIZE.scale_by(0.5);
    d.draw_rectangle(
        begin.x as i32,
        begin.y as i32,
        (MARKER_SIZE.x * 0.5) as i32,
        (MARKER_SIZE.y * 0.5) as i32,
        color,
    );
}
fn main() {
    let font = "./assets/JBMN.ttf";
    let character = 'X' as usize;
    let library = ft::Library::init().unwrap();
    let face = library.new_face(font, 0).unwrap();

    // face.set_char_size(40, 40, 40, 40).unwrap();
    // face.set_char_size(0, 11 * 64 + 32, 0, 0);
    face.load_char(character, ft::face::LoadFlag::NO_SCALE)
        .unwrap();

    let glyph = face.glyph();
    let metrics = glyph.metrics();
    let xmin = metrics.horiBearingX - 5;
    let width = metrics.width + 10;
    let ymin = -metrics.horiBearingY - 5;
    let height = metrics.height + 10;
    let outline = glyph.outline().unwrap();
    let points = outline.points();
    let contours = outline.contours();
    let n_points = points.len();
    let n_contours = contours.len();
    let mut x_min = points[0].x;
    let mut y_max = points[0].y;
    println!("{n_points} {n_contours}");
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Splines")
        .build();
    rl.set_target_fps(60);
    for i in 0..points.len() {
        let x = points[i].x;
        let y = points[i].y;
        if x_min > x {
            x_min = x;
        }
        if y_max < y {
            y_max = y;
        }
        let pos = Vector2 {
            x: (x - xmin) as f32,
            y: y as f32,
        };
        println!("{x} {y}");
    }
    let colors: Vec<Color> = vec![
        Color::RED,
        Color::BLUE,
        Color::GREEN,
        Color::YELLOW,
        Color::PINK,
        Color::WHITE,
        Color::CYAN,
    ];
    while !rl.window_should_close() {
        let mut pid: usize = 0;
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for i in 0..n_contours {
            while pid <= contours[i] as usize {
                let x = points[pid].x;
                let y = points[pid].y;
                let pos = Vector2 {
                    x: (x - x_min) as f32,
                    y: (y_max - y) as f32,
                };
                render_marker(
                    &mut d,
                    (Vector2 { x: 100.0, y: 100.0 } + pos).scale_by(0.5),
                    colors[i % colors.len()],
                );
                pid += 1;
            }
        }
    }
}

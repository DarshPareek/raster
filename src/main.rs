use freetype as ft;
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::ffi::KeyboardKey;
use raylib::ffi::MouseButton;
use raylib::math::Vector2;
use raylib::prelude::*;
use std::cmp::Ordering;
const WIDTH_FACTOR: i32 = 4;
const HEIGHT_FACTOR: i32 = 3;
const WINDOW_FACTOR: i32 = 300;
const WINDOW_WIDTH: i32 = WINDOW_FACTOR * WIDTH_FACTOR;
const WINDOW_HEIGHT: i32 = WINDOW_FACTOR * HEIGHT_FACTOR;
const GRID_FACTOR: i32 = 40;
const GRID_WIDTH: i32 = WIDTH_FACTOR * GRID_FACTOR;
const GRID_HEIGHT: i32 = HEIGHT_FACTOR * GRID_FACTOR;
const MARKER_SIZE: Vector2 = Vector2 { x: 25.0, y: 25.0 };
const BACKGROUD_COLOR: &str = "0D0C1D";
const BEZIER_COLOR: &str = "F1DAC4";
const MARKER_COLOR: &str = "474973";
const CELL_WIDTH: i32 = WINDOW_WIDTH / GRID_WIDTH;
const CELL_HEIGHT: i32 = WINDOW_HEIGHT / GRID_HEIGHT;
const OFFSET: Vector2 = Vector2 { x: 150.0, y: 150.0 };
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

fn display_grid(d: &mut RaylibDrawHandle, grid: &Vec<Vec<bool>>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if grid[y as usize][x as usize] {
                let position: Vector2 = Vector2 {
                    x: (x * CELL_WIDTH) as f32,
                    y: (y * CELL_HEIGHT) as f32,
                };
                let size: Vector2 = Vector2 {
                    x: CELL_WIDTH as f32,
                    y: CELL_HEIGHT as f32,
                };
                d.draw_rectangle_v(
                    position + size.scale_by(0.5),
                    size.scale_by(0.5),
                    hexcolor(BEZIER_COLOR),
                );
            }
        }
    }
}

struct Solution {
    tx: f32,
    d: f32,
}

impl Clone for Solution {
    fn clone(&self) -> Self {
        return Solution {
            tx: self.tx,
            d: self.d,
        };
    }
}

fn compare_solutions_by_tx(a: &Solution, b: &Solution) -> Ordering {
    if a.tx < b.tx {
        return Ordering::Less;
    } else if a.tx > b.tx {
        return Ordering::Greater;
    } else {
        return Ordering::Equal;
    }
}

fn solve_row(row: i32, ps: &Vec<Vector2>) -> Vec<Solution> {
    let mut solutions: Vec<Solution> = vec![];
    let y = ((row as f32) + 0.5) * CELL_HEIGHT as f32;
    let mut i: usize = 0;
    while i + 2 <= ps.len() {
        let p1 = ps[i];
        let p2 = ps[i + 1];
        let p3 = ps[(i + 2) % ps.len()];
        let dx12 = p2.x - p1.x;
        let dx23 = p3.x - p2.x;
        let dy12 = p2.y - p1.y;
        let dy23 = p3.y - p2.y;
        let a = dy23 - dy12;
        let b = 2.0 * dy12;
        let c = p1.y - y;
        let mut t: Vec<f32> = vec![];
        if a.abs() <= 1e-6 {
            if b.abs() > 1e-6 {
                t.push(-c / b);
            }
        } else {
            let d = b * b - 4.0 * a * c;
            if d >= 0.0 {
                t.push((-b + d.sqrt()) / (2.0 * a));
                t.push((-b - d.sqrt()) / (2.0 * a));
            }
        }
        for j in 0..t.len() {
            if !(0.0 <= t[j] && t[j] <= 1.0) {
                // i += 2;
                continue;
            } else {
                let tx = (dx23 - dx12) * t[j] * t[j] + 2.0 * dx12 * t[j] + p1.x;
                let s = (dy23 - dy12) * t[j] + dy12;
                let soln = Solution { tx: tx, d: s };
                solutions.push(soln);
            }
        }

        i += 2
    }
    solutions.sort_by(compare_solutions_by_tx);
    return solutions;
}

fn render_splines_into_grid(ps: &Vec<Vector2>, grid: &mut Vec<Vec<bool>>) {
    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            grid[row as usize][col as usize] = false;
        }
    }
    for row in 0..GRID_HEIGHT {
        let mut winding = 0;
        let solutions = solve_row(row, ps);
        for i in 0..solutions.len() {
            let s = solutions[i].clone();
            if winding > 0 {
                if i > 0 {
                    let ps = solutions[i - 1].clone();
                    let col1 = (ps.tx / CELL_WIDTH as f32) as usize;
                    let col2 = (s.tx / CELL_WIDTH as f32) as usize;
                    let mut j = col1;
                    while j <= col2 {
                        grid[row as usize][j] = true;
                        j += 1
                    }
                }
            }
            if s.d < 0.0 {
                winding += 1;
            } else if s.d > 0.0 {
                winding -= 1;
            }
        }
    }
}

fn ps_at(pos: Vector2, ps: &Vec<Vector2>) -> i32 {
    for i in 0..ps.len() {
        let begin: Vector2 = ps[i] - MARKER_SIZE.scale_by(0.5);
        let end: Vector2 = begin + MARKER_SIZE;
        if begin.x <= pos.x && pos.x <= end.x && begin.y <= pos.y && pos.y <= end.y {
            let res = i as i32;
            return res;
        }
    }
    return -1;
}

fn main() {
    let mut ps: Vec<Vector2> = Vec::new();
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut ps_selected: i32 = -1;
    for _ in 0..GRID_HEIGHT {
        let mut temp: Vec<bool> = Vec::new();
        for _ in 0..GRID_WIDTH {
            temp.push(false);
        }
        grid.push(temp);
    }
    let font = "./assets/JBMN.ttf";
    let character = 'W' as usize;
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
    let tags = outline.tags();
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
        let t = tags[i];
        if x_min > x {
            x_min = x;
        }
        if y_max < y {
            y_max = y;
        }
        let pos = Vector2 {
            x: (x - xmin) as f32,
            y: (y_max - y) as f32,
        };
        println!("{x} {y} {t}");
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
        let mouse_pos = rl.get_mouse_position();
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            ps_selected = ps_at(mouse_pos, &ps);
            if ps_selected < 0 {
                ps.push(mouse_pos);
                ps_selected = (ps.len() - 1) as i32;
            }
            // render_to_grid = true;
        }
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            if ps_selected > -1 && ps_selected < ps.len() as i32 {
                ps[ps_selected as usize] = mouse_pos;
            }
            // render_to_grid = true;
        }
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            ps_selected = -1;
            // render_to_grid = true;
        }
        let mut pid: usize = 0;
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        render_splines_into_grid(&ps, &mut grid);
        display_grid(&mut d, &grid);
        for i in 0..n_contours {
            while pid <= contours[i] as usize {
                let x = points[pid].x;
                let y = points[pid].y;
                let t = tags[pid];
                let pos = Vector2 {
                    x: (x - x_min) as f32,
                    y: (y_max - y) as f32,
                };
                ps.push((Vector2 { x: 150.0, y: 150.0 } + pos).scale_by(0.5));
                // render_marker(
                //     &mut d,
                //     (Vector2 { x: 150.0, y: 150.0 } + pos).scale_by(0.5),
                //     colors[i % colors.len()],
                // );
                let mut col = Color::GREEN;
                // println!("{t}");
                if t == 0 {
                    col = Color::RED;
                    // println!("SET TO RED");
                }
                // render_marker(
                //     &mut d,
                //     (Vector2 { x: 150.0, y: 150.0 } + pos).scale_by(0.5),
                //     col,
                // );
                pid += 1;
            }
        }
    }
}

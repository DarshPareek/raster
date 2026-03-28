use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::ffi::KeyboardKey;
use raylib::ffi::MouseButton;
use raylib::ffi::false_;
use raylib::math::Vector2;
use raylib::prelude::*;
const WIDTH_FACTOR: i32 = 4;
const HEIGHT_FACTOR: i32 = 3;
const WINDOW_FACTOR: i32 = 200;
const WINDOW_WIDTH: i32 = WINDOW_FACTOR * WIDTH_FACTOR;
const WINDOW_HEIGHT: i32 = WINDOW_FACTOR * HEIGHT_FACTOR;
const GRID_FACTOR: i32 = 50;
const GRID_WIDTH: i32 = WIDTH_FACTOR * GRID_FACTOR;
const GRID_HEIGHT: i32 = HEIGHT_FACTOR * GRID_FACTOR;
const MARKER_SIZE: Vector2 = Vector2 { x: 25.0, y: 25.0 };
const BACKGROUD_COLOR: &str = "0D0C1D";
const BEZIER_COLOR: &str = "F1DAC4";
const LINE_COLOR: &str = "161B33";
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
        MARKER_SIZE.x as i32,
        MARKER_SIZE.y as i32,
        color,
    );
}

fn render_line(d: &mut RaylibDrawHandle, from: Vector2, to: Vector2, color: Color) {
    d.draw_line_ex(from, to, 3.0, color);
}

fn bezier3_sample(p1: Vector2, p2: Vector2, p3: Vector2, t: f32) -> Vector2 {
    return p1 + (p2 - p1).scale_by(t * 2.0) + (p3 + p2.scale_by(-2.0) + p1).scale_by(t * t);
}

fn beziern_sample(ps: &mut Vec<Vector2>, p: f32) -> Vector2 {
    let mut xs: Vec<Vector2> = ps.clone();
    while xs.len() > 1 {
        for i in 0..xs.len() - 1 {
            xs[i] = xs[i].lerp(xs[i + 1], p);
        }
        xs.pop();
    }
    return xs[0];
}

fn render_bezier_markers(d: &mut RaylibDrawHandle, ps: &mut Vec<Vector2>, s: f32, color: Color) {
    let mut p: f32 = 0.0;
    while p <= 1.0 {
        render_marker(d, bezier3_sample(ps[0], ps[1], ps[2], p), color);
        p += s;
    }
}

fn render_bezier_curve(d: &mut RaylibDrawHandle, ps: &mut Vec<Vector2>, s: f32, color: Color) {
    let mut p: f32 = 0.0;
    while p <= 1.0 {
        let mut start: Vector2 = beziern_sample(ps, p);
        // let mut start: Vector2 = bezier3_sample(ps[0], ps[1], ps[2], p);
        let end: Vector2 = beziern_sample(ps, p + s);
        render_line(d, start, end, color);
        // render_line(d, ps[0], start, color);
        // render_line(d, start, ps[2], color);
        // start = ps[2];
        start = end;
        p += s;
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

fn render_spline_markers(d: &mut RaylibDrawHandle, ps: &Vec<Vector2>) {
    let mut i: usize = 0;
    while i + 2 <= ps.len() {
        let p1 = ps[i];
        let p2 = ps[i + 1];
        let p3 = ps[(i + 2) % ps.len()];
        let n: usize = 25;
        let mut j: usize = 0;
        while j <= n {
            let t: f32 = j as f32 / n as f32;
            let position = bezier3_sample(p1, p2, p3, t);
            d.draw_rectangle_v(
                position - MARKER_SIZE.scale_by(0.25),
                MARKER_SIZE.scale_by(0.5),
                hexcolor(BEZIER_COLOR),
            );
            j += 1;
        }
        i += 2;
    }
}

fn solve_quad_equation(p1: Vector2, p2: Vector2, p3: Vector2, p0: Vector2, threshold: f32) -> i32 {
    let ax = (p3 + p2.scale_by(-2.0) + p1).x;
    let bx = (p2 - p1).scale_by(2.0).x;
    let cx = (p1 - p0).x;
    let dx = bx * bx - 4.0 * ax * cx;
    if dx > 0.0 {
        let t1 = ((p2 - p1).scale_by(-2.0).x - dx.sqrt())
            / (p3 + p2.scale_by(-2.0) + p1).scale_by(2.0).x;
        let t2 = ((p2 - p1).scale_by(-2.0).x + dx.sqrt())
            / (p3 + p2.scale_by(-2.0) + p1).scale_by(2.0).x;
        let y1 = p1.y + 2.0 * t1 * (p2.y - p1.y) + t1 * t1 * (p3.y - 2.0 * p2.y + p1.y);
        let y2 = p1.y + 2.0 * t2 * (p2.y - p1.y) + t2 * t2 * (p3.y - 2.0 * p2.y + p1.y);
        return ((0.0 <= t1 && t1 <= 1.0 && (p0.y - y1).abs() < threshold)
            || (0.0 <= t2 && t2 <= 1.0 && (p0.y - y2).abs() < threshold)) as i32;
    }
    return 0;
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
                d.draw_rectangle_v(position + size.scale_by(0.5), size, hexcolor(BEZIER_COLOR));
            }
        }
    }
}

fn render_splines_into_grid(
    d: &mut RaylibDrawHandle,
    ps: &Vec<Vector2>,
    grid: &mut Vec<Vec<bool>>,
) {
    for row in 0..GRID_HEIGHT {
        let mut winding = 0;
        for col in 0..GRID_WIDTH {
            let cell_size = Vector2 {
                x: CELL_WIDTH as f32,
                y: CELL_HEIGHT as f32,
            };
            let center = Vector2 {
                x: (col * CELL_WIDTH) as f32,
                y: (row * CELL_HEIGHT) as f32,
            };
            let cell_position = center + cell_size.scale_by(0.5);
            let x = cell_position.x;
            let y = cell_position.y;
            let index = -1;
            let mut i: usize = 0;
            while i + 2 <= ps.len() {
                // println!("{row} {col}");
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
                let d = b * b - 4.0 * a * c;
                if d <= 0.0 {
                    i += 2;
                    continue;
                } else if d > 0.0 {
                    let t: Vec<f32> =
                        vec![(-b + d.sqrt()) / (2.0 * a), (-b - d.sqrt()) / (2.0 * a)];
                    for j in 0..t.len() {
                        if !(0.0 <= t[j] && t[j] <= 1.0) {
                            // i += 2;
                            continue;
                        } else {
                            let tx = (dx23 - dx12) * t[j] * t[j] + 2.0 * dx12 * t[j] + p1.x;
                            let ty = (dy23 - dy12) * t[j] * t[j] + 2.0 * dy12 * t[j] + p1.y;
                            // if x < tx {
                            //     continue;
                            // }
                            if (tx - x).abs() < cell_size.x * 0.5 {
                                if (dy23 - dy12) * t[j] + dy12 <= 0.0 {
                                    winding += 1;
                                } else if (dy23 - dy12) * t[j] + dy12 > 0.0 {
                                    winding -= 1;
                                }
                            }
                        }
                    }
                }
                i += 2
            }
            if winding > 0 {
                grid[row as usize][col as usize] = true;
            } else {
                grid[row as usize][col as usize] = false;
            }
        }
    }
}

fn main() {
    println!("{CELL_HEIGHT}, {CELL_WIDTH}, {WINDOW_HEIGHT}, {WINDOW_WIDTH}");
    let mut ps: Vec<Vector2> = Vec::new();
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for _ in 0..GRID_HEIGHT {
        let mut temp: Vec<bool> = Vec::new();
        for _ in 0..GRID_WIDTH {
            temp.push(false);
        }
        grid.push(temp);
    }
    let mut s: f32 = 0.01;
    let mut ps_selected: i32 = -1;
    let mut line_toggle = false;
    let mut marker_toggle = true;
    let mut render_to_grid = false;
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Splines")
        .build();
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            if !marker_toggle {
                marker_toggle = true
            } else {
                marker_toggle = false;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_L) {
            if !line_toggle {
                line_toggle = true
            } else {
                line_toggle = false;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            render_to_grid = true;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_C) {
            ps.clear();
        }
        let mouse_pos = rl.get_mouse_position();
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            ps_selected = ps_at(mouse_pos, &ps);
            if ps_selected < 0 {
                ps.push(mouse_pos);
                ps_selected = (ps.len() - 1) as i32;
            }
        }
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            if ps_selected > -1 && ps_selected < ps.len() as i32 {
                ps[ps_selected as usize] = mouse_pos;
            }
        }
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            ps_selected = -1;
        }

        let mouse_wheel = rl.get_mouse_wheel_move();
        if mouse_wheel > 0.0 {
            s += 0.01;
            if s > 0.99 {
                s = 0.99;
            }
        } else if mouse_wheel < 0.0 {
            s -= 0.01;
            if s < 0.1 {
                s = 0.01;
            }
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(hexcolor(BACKGROUD_COLOR));
        if render_to_grid {
            render_splines_into_grid(&mut d, &ps, &mut grid);
            render_to_grid = false;
        }
        display_grid(&mut d, &grid);
        if marker_toggle {
            for i in 0..ps.len() {
                render_marker(&mut d, ps[i], hexcolor(MARKER_COLOR));
            }
        }
    }
}

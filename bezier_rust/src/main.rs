use raylib::{math::Vector2, prelude::*};
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;
const MARKER_SIZE: Vector2 = Vector2 { x: 25.0, y: 25.0 };
const BACKGROUD_COLOR: &str = "0D0C1D";
const BEZIER_COLOR: &str = "F1DAC4";
const LINE_COLOR: &str = "161B33";
const MARKER_COLOR: &str = "474973";

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
    d.draw_line(
        from.x as i32,
        from.y as i32,
        to.x as i32,
        to.y as i32,
        color,
    );
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
            || (0.0 <= t1 && t1 <= 1.0 && (p0.y - y2).abs() < threshold)) as i32;
    }
    return 0;
}

fn main() {
    let mut ps: Vec<Vector2> = Vec::new();
    let mut s: f32 = 0.01;
    let mut ps_selected: i32 = -1;
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Bezier Curves")
        .build();
    let mut shader = rl.load_shader(&thread, Some(""), Some("src/shader.fs"));
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        // Handling Inputs
        if rl.is_key_pressed(KeyboardKey::KEY_A) {
            println!("Working on Animations");
        }
        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            println!("Working on Adding Markers");
        }
        if rl.is_key_pressed(KeyboardKey::KEY_L) {
            println!("Working on Adding Lines");
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
        let mut sd = d.begin_shader_mode(&mut shader);
        for i in 0..ps.len() {
            render_marker(&mut sd, ps[i], hexcolor(MARKER_COLOR));
        }
        if ps.len() > 2 {
            render_bezier_curve(&mut sd, &mut ps, s, hexcolor(BEZIER_COLOR));
            render_bezier_markers(&mut sd, &mut ps, s, hexcolor(BEZIER_COLOR));
        }
        let bezier_probe = mouse_pos;
        if ps.len() >= 3 {
            let roots = solve_quad_equation(ps[0], ps[1], ps[2], mouse_pos, 10.0);
            if roots == 1 {
                render_marker(&mut sd, bezier_probe, Color::GREEN);
            } else {
                render_marker(&mut sd, bezier_probe, Color::RED);
            }
        }
        // if ps.len() > 1 {
        //     for i in 0..(ps.len() - 1) {
        //         render_line(&mut d, ps[i], ps[i + 1], Color::LIGHTGREEN);
        //     }
        // }
    }
}

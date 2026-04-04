mod raster;
use freetype as ft;
use raster::*;
use raylib::color::Color;
use raylib::ffi::MouseButton;
use raylib::math::Vector2;
use raylib::prelude::*;
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
    let font = "./assets/FiraCode-Regular.ttf";
    let character = '>' as usize;
    let library = ft::Library::init().unwrap();
    let face = library.new_face(font, 0).unwrap();
    face.load_char(character, ft::face::LoadFlag::NO_SCALE)
        .unwrap();

    let glyph = face.glyph();
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
        }
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            if ps_selected > -1 && ps_selected < ps.len() as i32 {
                ps[ps_selected as usize] = mouse_pos;
            }
        }
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            ps_selected = -1;
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
                pid += 1;
            }
        }
    }
}

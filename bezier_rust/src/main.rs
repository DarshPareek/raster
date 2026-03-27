use raylib::{math::Vector2, prelude::*};
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;
const MARKER_SIZE: Vector2 = Vector2 { x: 25.0, y: 25.0 };

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Bezier Curves")
        .build();
    let mut shader = rl.load_shader(&thread, Some("src/Vert.vs"), Some("src/Frag.fs"));
    let u_p1 = shader.get_shader_location("p1");
    let u_p2 = shader.get_shader_location("p2");
    let u_p3 = shader.get_shader_location("p3");
    let u_mr = shader.get_shader_location("marker_radius");
    let mut selected_idx: Option<usize> = None;
    let mut ps = vec![
        Vector2 { x: 100.0, y: 100.0 },
        Vector2 { x: 200.0, y: 200.0 },
        Vector2 { x: 300.0, y: 300.0 },
    ];
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
        let mut mouse_pos = rl.get_mouse_position();
        mouse_pos = Vector2 {
            x: mouse_pos.x,
            y: WINDOW_HEIGHT as f32 - mouse_pos.y,
        };
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            selected_idx = None; // Reset
            for (i, p) in ps.iter().enumerate() {
                if (*p - mouse_pos).length() < MARKER_SIZE.x {
                    selected_idx = Some(i);
                    break;
                }
            }
        }
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some(idx) = selected_idx {
                ps[idx] = mouse_pos;
            }
        }
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            selected_idx = None;
        }
        let mut d = rl.begin_drawing(&thread);
        shader.set_shader_value(u_p1, ps[0]);
        shader.set_shader_value(u_p2, ps[1]);
        shader.set_shader_value(u_p3, ps[2]);
        shader.set_shader_value(u_mr, MARKER_SIZE.x);
        let mut sd = d.begin_shader_mode(&mut shader);
        sd.clear_background(Color::BLACK);
        sd.draw_rectangle(0, 0, 800, 800, Color::FLORALWHITE);
    }
}

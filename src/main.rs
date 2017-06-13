#![feature(drop_types_in_const)]

#[macro_use]
extern crate glium;
extern crate cgmath;

pub mod ui_screen;
pub mod input;

const INITIAL_HEIGHT: u32 = 600;
const INITIAL_WIDTH: u32 = 800;

fn main() {

    // Initialize keyboard & mouse
    let mut keyboard_state = input::KeyboardState::new();
    let mut mouse_state = input::MouseState::new(10.0, 10.0);
    let mut window_state = input::WindowState::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    let mut ui_screen = ui_screen::UiScreen::new();

    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(INITIAL_WIDTH, INITIAL_HEIGHT)
        .with_title("File Explorer")
        .with_multisampling(4)
        .build_glium()
        .unwrap();

    for event in display.wait_events() {
        if input::handle_event(&event, &mut window_state, &mut keyboard_state, &mut mouse_state, &mut ui_screen) {
            render(&display);
        }
    }
}

fn render(display: &glium::Display) {
    use glium::Surface;
    let mut target = display.draw();
    target.clear_color(1.0, 1.0, 1.0, 1.0);
    target.finish().unwrap();
}


extern crate glium;
extern crate cgmath;

pub mod ui_screen;

static mut GLOB_MOUSE_POS: (i32, i32) = (0, 0);

/// Global keyboard modifiers

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(800, 600)
        .with_title("File Explorer")
        .with_multisampling(4)
        .build_glium()
        .unwrap();

    for event in display.wait_events() {
        if handle_event(&event) {
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

/// Handles the event, updates the UI, then returns if the UI has to be rerendered
fn handle_event(event: &glium::glutin::Event) -> bool {
    use glium::glutin::Event;
    match *event {
        Event::MouseMoved(x, y) => { unsafe { GLOB_MOUSE_POS = (x, y) }; false},
        _ => { println!("{:?}", event); true },
    }
}
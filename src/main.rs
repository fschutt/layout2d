#![feature(test)]

#[macro_use]
extern crate glium;
extern crate rctree;
extern crate simd;
extern crate test;
extern crate image;

pub mod ui_screen;
pub mod input;
pub mod node_data;
pub mod renderer;
pub mod rect;
mod debug;

use rctree::NodeRef;
use node_data::NodeData;
use debug::DebugColor;
use renderer::Renderer;

const INITIAL_HEIGHT: u32 = 600;
const INITIAL_WIDTH: u32 = 800;

fn main() {

    // Initialize keyboard & mouse
    let mut window_state = input::WindowState::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    let renderer = Renderer::new(INITIAL_WIDTH, INITIAL_HEIGHT);

    // Construct the explorer UI
    let mut ui_screen = ui_screen::UiScreen::new(INITIAL_WIDTH, INITIAL_HEIGHT);
        // Top bar, 100 - 200 pixels tall, stretches full window
        ui_screen.root.append(NodeRef::new(NodeData::new(None, Some(100), None, Some(200), Some(300), Some(400), DebugColor::green())));
        // Side bar, max 400 px wide 
        ui_screen.root.append(NodeRef::new(NodeData::new(None, None, Some(400), None, Some(300), Some(400), DebugColor::red())));
        // Main explorer view, stretches all sides
        ui_screen.root.append(NodeRef::new(NodeData::new(None, None, None, None, Some(300), Some(400), DebugColor::blue() )));

        // renderer.display.get_window().unwrap().set_cursor(glium::glutin::MouseCursor::Wait);
        // let texture_angle = renderer.load_image_png(include_bytes!("assets/widget_tree_view_arrow.svg.png"));

        /// when adding animations, change this to poll events
        for event in renderer.display.wait_events() {
            input::handle_event(&event, &mut window_state, &mut ui_screen);
            // renderer.render(&window_state, &ui_screen, Some(&texture_angle));
            renderer.render(&window_state, &ui_screen, None);
        }
}

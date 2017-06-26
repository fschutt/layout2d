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
use node_data::{NodeData, FlexDirection};
use debug::DebugColor;
use renderer::Renderer;
use ui_screen::UiScreen;

const INITIAL_HEIGHT: u32 = 600;
const INITIAL_WIDTH: u32 = 800;

fn main() {

    // Initialize keyboard & mouse
    let mut window_state = input::WindowState::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    let renderer = Renderer::new(INITIAL_WIDTH, INITIAL_HEIGHT, true, true);

    // Construct the explorer UI
    let mut ui_screen = UiScreen::new(INITIAL_WIDTH, INITIAL_HEIGHT)
                            .with_root_as_column();

    // Top bar, 100 - 200 pixels tall, stretches full window
    let top_bar_wrapper = NodeRef::new(NodeData::new(
            None, None, None, None, None, Some(100.0), 
            FlexDirection::Column, DebugColor::green()));

        let top_bar_file = NodeRef::new(NodeData::new(
                None, None, None, None, None, Some(24.0), 
                FlexDirection::Column, DebugColor::blue()));
        top_bar_wrapper.append(top_bar_file);

        let top_bar_ribbon = NodeRef::new(NodeData::new(
                None, None, None, None, None, Some(76.0), 
                FlexDirection::Column, DebugColor::yellow()));
        top_bar_wrapper.append(top_bar_ribbon);
    ui_screen.root.append(top_bar_wrapper);

    // Main explorer view, stretches all sides
    let explorer_wrapper = NodeRef::new(NodeData::new(
        None, None, None, None, None, None, 
        FlexDirection::Row, DebugColor::blue()));

            // navigation side bar
            let navigation_pane = NodeRef::new(NodeData::new(
                None, None, Some(200.0), None, None, None, 
                FlexDirection::Column, DebugColor::red()));
            explorer_wrapper.append(navigation_pane);

            // file list
            let file_list_view = NodeRef::new(NodeData::new(
                None, None, None, None, None, None, 
                FlexDirection::Column, DebugColor::blue()));
            explorer_wrapper.append(file_list_view);
    ui_screen.root.append(explorer_wrapper);

    // ------------------ end constructing UI screen

    // renderer.display.get_window().unwrap().set_cursor(glium::glutin::MouseCursor::Wait);
    // let texture_angle = renderer.load_image_png(include_bytes!("assets/widget_tree_view_arrow.svg.png"));

    /// when adding animations, change this to poll events
    for event in renderer.display.wait_events() {
        input::handle_event(&event, &mut window_state, &mut ui_screen);
        // renderer.render(&window_state, &ui_screen, Some(&texture_angle));
        renderer.render(&window_state, &ui_screen, None);
    }
}

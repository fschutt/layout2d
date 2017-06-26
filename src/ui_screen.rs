//! Ui screen is a single screen that is visible at one time
//! Uses rctree for reference counted nodes
extern crate rctree;
extern crate glium;
extern crate simd;

use rctree::NodeRef;
use node_data::NodeData;
use debug::DebugColor;
use renderer::Vertex;
use rect::Rect;
use input::WindowState;

/// UI screen 
#[derive(Debug)]
pub struct UiScreen {
    /// Root node of the UI tree
    pub root: NodeRef<NodeData>,
}

impl UiScreen {

    /// Creates a new UiScreen
    #[inline]
    pub fn new(initial_width: u32, initial_height: u32) 
    -> Self 
    {
        Self { root: NodeRef::new(NodeData::new(Some(600), Some(400), None, None, Some(initial_width), Some(initial_height), DebugColor::yellow())) }
    }

    /// Refreshes the UiScreen, returns if the frame has to be redrawn or not
    #[inline]
    pub(crate) fn layout(&mut self, window: &WindowState)
    -> bool 
    {
        // todo
        true
    }

    /// Converts the UI into a vertex buffer
    pub fn into_vertex_buffer(&self, display: &glium::Display, window: &WindowState)
    -> glium::VertexBuffer<Vertex>
    {
        // todo: use the &self to convert the final layout into rectangles
        // that are then submitted to the renderer
        let display_list = ui_screen_to_dp_list(&self.root, 0);

        // let rect = Rect::new(200.0, 210.0, 400.0, 410.0, 0);
        let mut vertices: Vec<Vertex> = Vec::<Vertex>::new();
        for rect in display_list {
            vertices.append(&mut rect.into());
        }

        glium::VertexBuffer::new(display, &vertices).unwrap()
    }

}

/// Recursively traverse and convert the node data into a list of rectangles
/// parent: the parent of the node
/// parent_width: the width of the parent node
/// parent_height: the height of the parent node
/// cur_z: The z-index, starts at 0 and increases. Is passed to OpenGL later
/// **WARNING**: The root node have a width and a height
fn ui_screen_to_dp_list(parent: &NodeRef<NodeData>,  
                        cur_z: u32)
-> Vec<Rect>
{
    let mut rectangles = Vec::<Rect>::new();

    let mut width = 0_u32;
    let mut height = 0_u32;

    // root must be initialized!
    if cur_z == 0 { 
        width = parent.borrow().width.unwrap();
        height = parent.borrow().height.unwrap();
    }

    if let Some(w) = parent.borrow().width {
        width = w;
    }

    if let Some(h) = parent.borrow().height {
        height = h;
    }

    for node in parent.children() {

        // offset_left: f32, offset_top: f32, width: f32, height: f32, z: u32
        let mut cur_rect = Rect::new_wh(0.0, 0.0, width as f32, height as f32, cur_z);
        
        if let Some(min_width) = parent.borrow().min_width_rem {
            cur_rect.set_width(min_width as f32);
        }

        if let Some(min_height) = parent.borrow().min_height_rem {
            cur_rect.set_height(min_height as f32);
        }

        if let Some(max_width) = parent.borrow().max_width_rem {
            cur_rect.set_width(max_width as f32);
        }

        if let Some(max_height) = parent.borrow().max_height_rem {
            cur_rect.set_width(max_height as f32);
        }

        rectangles.push(cur_rect);
        rectangles.append(&mut ui_screen_to_dp_list(&node, cur_z + 1)); 
    }

    return rectangles;
}
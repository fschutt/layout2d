//! Ui screen is a single screen that is visible at one time
//! Uses rctree for reference counted nodes
extern crate rctree;
extern crate glium;
extern crate simd;

use rctree::NodeRef;
use node_data::{NodeData, FlexDirection};
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
        Self { root: NodeRef::new(NodeData::new(
                    None, None, None, None, 
                    Some(initial_width), 
                    Some(initial_height), 
                    FlexDirection::Horizontal,
                    DebugColor::yellow() )) 
        }
    }

    /// Refreshes the UiScreen, returns if the frame has to be redrawn or not
    #[inline]
    pub(crate) fn layout(&mut self, window: &WindowState)
    -> bool 
    {
        self.root.borrow_mut().width = Some(window.width);
        self.root.borrow_mut().height = Some(window.height);

        // todo
        true
    }

    /// Converts the UI into a vertex buffer
    pub fn into_vertex_buffer(&self, display: &glium::Display)
    -> glium::VertexBuffer<Vertex>
    {
        // todo: use the &self to convert the final layout into rectangles
        // that are then submitted to the renderer
        let display_list = ui_screen_to_dp_list(&self.root, 0.0, 1);

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
/// cur_z: The z-index, starts at 0 and increases. Is passed to OpenGL later
/// sibling_count: How many siblings does this node have? (for flex distributing)
/// sibling_count is 1 for root
/// **WARNING**: The root node have a width and a height (usually the case when
/// you create the UiScreen via `.new()`)
fn ui_screen_to_dp_list(parent: &NodeRef<NodeData>,  
                        cur_z: f32,
                        sibling_count: u32)
-> Vec<Rect>
{
    // todo: remaining width

    let mut rectangles = Vec::<Rect>::new();

    let mut width = 0_u32;
    let mut height = 0_u32;

    // root must be initialized!
    if cur_z == 0.0 { 
        width = parent.borrow().width.unwrap();
        height = parent.borrow().height.unwrap();
    }

    if let Some(w) = parent.borrow().width { width = w; }

    if let Some(h) = parent.borrow().height { height = h; }

    // initially, set the width + height to whatever the parent is
    // plus take flex direction into account
    if parent.borrow().flex_direction == FlexDirection::Vertical { 
        height /= sibling_count; } 
    else { 
        width /= sibling_count; 
    }

    let mut has_modified_width = false;
    let mut has_modified_height = false;

    // if the width is greater than the maximal specified width, reduce
    if let Some(max_width) = parent.borrow().max_width_rem {
        if width > max_width {
            width = max_width;
            has_modified_width = true;
        }
    }

    if let Some(max_height) = parent.borrow().max_height_rem {
        if height > max_height {
            height = max_height;
            has_modified_height = true;
        }
    }

    // if the width is smaller than the minimal width, overflow the parent
    if let Some(min_width) = parent.borrow().min_width_rem {
        width = min_width;
        has_modified_width = true;
    }

    if let Some(min_height) = parent.borrow().min_height_rem {
        height = min_height;
        has_modified_height = true;
    }

    // construct rectangle and repeat for children
    // mark if min-width or max-width has modified the remaining width for siblings

    // offset_left: f32, offset_top: f32, width: f32, height: f32, z: u32
    let cur_rect = Rect::new_wh(0.0, 0.0, width as f32, height as f32, cur_z, parent.borrow().debug_color);
    rectangles.push(cur_rect);
    
    let sibling_count = parent.children().count();
    for (index, node) in parent.children().enumerate() {
        let advance_z = cur_z + (1.0 / sibling_count as f32 * index as f32);
        rectangles.append(&mut ui_screen_to_dp_list(&node, advance_z, sibling_count as u32)); 
    }

    return rectangles;
}
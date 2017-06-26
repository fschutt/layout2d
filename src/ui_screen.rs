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
                    Some(initial_width as f32), 
                    Some(initial_height as f32), 
                    FlexDirection::Horizontal,
                    DebugColor::yellow() )) 
        }
    }

    /// Refreshes the UiScreen, returns if the frame has to be redrawn or not
    #[inline]
    pub(crate) fn layout(&mut self, window: &WindowState)
    -> bool 
    {
        self.root.borrow_mut().width = Some(window.width as f32);
        self.root.borrow_mut().height = Some(window.height as f32);

        // todo
        true
    }

    /// Converts the UI into a vertex buffer
    pub fn into_vertex_buffer(&self, display: &glium::Display)
    -> glium::VertexBuffer<Vertex>
    {
        // todo: use the &self to convert the final layout into rectangles
        // that are then submitted to the renderer
        let mut max_width = self.root.borrow().width.unwrap().clone();
        let mut max_height = self.root.borrow().width.unwrap().clone();
        
        let display_list = ui_screen_to_dp_list(&self.root, 0.0, 1, 0, 
                           self.root.borrow().width.unwrap(), self.root.borrow().height.unwrap(),
                           &mut max_width, &mut max_height);

        // let rect = Rect::new(200.0, 210.0, 400.0, 410.0, 0);
        let mut vertices: Vec<Vertex> = Vec::<Vertex>::new();
        for rect in display_list {
            vertices.append(&mut rect.into());
        }

        glium::VertexBuffer::new(display, &vertices).unwrap()
    }

}

/// Recursively traverse and convert the node data into a list of rectangles
/// current: the current node
/// cur_z: The z-index, starts at 0 and increases. Is passed to OpenGL later
/// sibling_count: How many siblings does this node have? (for flex distributing)
/// sibling_count is 1 for root
/// **WARNING**: The root node have a width and a height (usually the case when
/// you create the UiScreen via `.new()`)
fn ui_screen_to_dp_list(current: &NodeRef<NodeData>,  
                        cur_z: f32,
                        sibling_count: u32,
                        sibling_index: u32,
                        parent_width: f32,
                        parent_height: f32,
                        remaining_width: &mut f32,
                        remaining_height: &mut f32)
-> Vec<Rect>
{
    let mut rectangles = Vec::<Rect>::new();

    let (mut width, mut height) =  {
        if let Some(parent) = current.parent() {
            // initially, set the width + height to whatever the current node is
            // plus take flex direction into account
            let w;
            let h;

            if parent.borrow().flex_direction == FlexDirection::Vertical { 
                h = *remaining_height / sibling_count as f32;
                w = *remaining_width;
            } else { 
                w = *remaining_width / sibling_count as f32;
                h = *remaining_height;

            }

            (w, h)

        } else {
            // root node
            (*remaining_width, *remaining_height)
        }
    };

    if let Some(w) = current.borrow().width { width = w; }
    if let Some(h) = current.borrow().height { height = h; }

    // if the width is greater than the maximal specified width, reduce
    if let Some(max_width) = current.borrow().max_width_rem {
        if width > max_width {
            width = max_width;
        }
    }

    if let Some(max_height) = current.borrow().max_height_rem {
        if height > max_height {
            height = max_height;
        }
    }

    // if the width is smaller than the minimal width, overflow the current
    if let Some(min_width) = current.borrow().min_width_rem {
        if width < min_width {    
            width = min_width;
        }
    }

    if let Some(min_height) = current.borrow().min_height_rem {
        if height < min_height {   
            height = min_height;
        }
    }

    let offset_top  = parent_height - *remaining_height;
    let offset_left = parent_width - *remaining_width;

    // construct rectangle and repeat for children
    // mark if min-width or max-width has modified the remaining width for siblings

    let cur_rect = Rect::new_wh(offset_left, offset_top, width as f32, height as f32, cur_z, current.borrow().debug_color);
    
    *remaining_width -= offset_left;
    *remaining_height -= offset_top;

    // iterate children nodes
    let sibling_count = current.children().count();

    for (index, node) in current.children().enumerate() {
        let advance_z = cur_z + (1.0 / sibling_count as f32 * index as f32);
        rectangles.append(&mut ui_screen_to_dp_list(&node, advance_z, sibling_count as u32, index as u32, 
                          width, height, remaining_width, remaining_height)); 
    }

    rectangles.push(cur_rect);

    return rectangles;
}
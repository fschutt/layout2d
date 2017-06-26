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
                    FlexDirection::Row,
                    DebugColor::yellow() )) 
        }
    }

    /// Changes the default orientation for the root element from row to column
    #[inline]
    pub fn with_root_as_column(self)
    -> Self
    {
        self.root.borrow_mut().flex_direction = FlexDirection::Column;
        self
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
        let mut max_width = self.root.borrow().width.unwrap().clone();
        let mut max_height = self.root.borrow().height.unwrap().clone();
        let parent_width = self.root.borrow().width.unwrap();
        let parent_height = self.root.borrow().height.unwrap();

        let min_z_index = 0.0;
        let max_z_index = 1.0;
        let root_sibling_count = 0;
        let root_level_children = 1;

        let display_list = ui_screen_to_dp_list(
                                &self.root, min_z_index, max_z_index, 
                                root_level_children, root_sibling_count, 
                                &parent_width, &parent_height,
                                &mut max_width, &mut max_height
                           );

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
                        min_z: f32,
                        max_z: f32,
                        sibling_count: u32,
                        sibling_index: u32,
                        parent_width: &f32,
                        parent_height: &f32,
                        remaining_width: &mut f32,
                        remaining_height: &mut f32)
-> Vec<Rect>
{
    let mut rectangles = Vec::<Rect>::new();

    let (mut width, mut height) =  {
        if let Some(parent) = current.parent() {
            if parent.borrow().flex_direction == FlexDirection::Row { 
                (*remaining_width / (sibling_count - sibling_index) as f32, *remaining_height)
            } else { 
                (*remaining_width, *remaining_height / (sibling_count - sibling_index) as f32)
            }
        } else {
            // root node
            (*remaining_width, *remaining_height)
        }
    };

    // correct width if there are hard constraints on max, min or exact width / height

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

    // if the width is smaller than the minimal width, overflow the parent
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

    // calculate offset for top and left
    let (offset_top, offset_left)  = {
        if let Some(parent) = current.parent() {
            if parent.borrow().flex_direction == FlexDirection::Row {
                let offset_w = parent_width - *remaining_width;
                *remaining_width -= width;
                (0.0, offset_w)
            } else {
                let offset_h = parent_height - *remaining_height;
                *remaining_height -= height;
                (offset_h, 0.0)
            }
        } else { (0.0, 0.0) }
    };

    // z sorting is done by recursively dividing the range between max_z and 
    // min_z into segments proportional to the siblings - this way the children won't overlap the parent
    let cur_z_stepping = (max_z - min_z) / (sibling_count as f32 + 1.0); 
    let z_index_current_node = cur_z_stepping * (sibling_index as f32 + 1.0);

    // construct rectangle and repeat for children
    // mark if min-width or max-width has modified the remaining width for siblings
    let cur_rect = Rect::new_wh(offset_left, offset_top, width as f32, height as f32, 
                                z_index_current_node, current.borrow().debug_color);

    // iterate children nodes
    let children_count = current.children().count();

    let self_width = width.clone();
    let self_height = height.clone();
    let mut max_width = width.clone();
    let mut max_height = height.clone();
    
    let new_max_z = z_index_current_node + cur_z_stepping;

    for (index, node) in current.children().enumerate() {
        rectangles.append(&mut ui_screen_to_dp_list(&node, z_index_current_node, new_max_z, children_count as u32, index as u32, 
                          &self_width, &self_height, &mut max_width, &mut max_height)); 
    }

    rectangles.push(cur_rect);

    return rectangles;
}
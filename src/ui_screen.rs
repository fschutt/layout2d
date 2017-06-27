//! Ui screen is a single screen that is visible at one time
//! Uses rctree for reference counted nodes
extern crate rctree;
extern crate glium;
extern crate simd;
extern crate rand;
extern crate test;

use rctree::NodeRef;
use node_data::{NodeData, FlexDirection};
use debug::DebugColor;
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
    pub fn into_rectangles(&self)
    -> Vec<Rect>
    {
        let mut cur_offset_width = 0.0;
        let mut cur_offset_height = 0.0;
        let parent_width = self.root.borrow().width.unwrap().clone();
        let parent_height = self.root.borrow().height.unwrap().clone();

        let min_z_index = 0.0;
        let max_z_index = 1.0;
        let root_sibling_count = 0;
        let root_level_children = 1;

        ui_screen_to_dp_list(&self.root, min_z_index, max_z_index, 
                             root_level_children, root_sibling_count, 
                             parent_width, parent_height, 0.0, 0.0,
                             &mut cur_offset_width, &mut cur_offset_height)
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
                        parent_width: f32,
                        parent_height: f32,
                        parent_offset_left: f32,
                        parent_offset_top: f32,
                        cur_offset_left: &mut f32,
                        cur_offset_top: &mut f32)
-> Vec<Rect>
{
    let mut rectangles = Vec::<Rect>::new();

    let mut width = parent_width; /* ; */
    let mut height = parent_height; /*   */

    if let Some(parent) = current.parent() {
        if parent.borrow().flex_direction == FlexDirection::Row {
            width /= (sibling_count - sibling_index) as f32;
        } else { 
            height /= (sibling_count - sibling_index) as f32;
        }
    }

    width -= *cur_offset_left;
    height -= *cur_offset_top;

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

    // calculate space top + left
    let offset_top = cur_offset_top.clone() + parent_offset_top;
    let offset_left = cur_offset_left.clone() + parent_offset_left;

/*
    if let Some(parent) = current.parent() {
        if parent.borrow().flex_direction == FlexDirection::Row {
            if (offset_left + width) > *parent_width {
                // wrap into next row
                offset_top += height;
                offset_left = 0.0;
            }
        } else {
            if (offset_top + height) > *parent_height {
                // wrap into next column
                offset_left += width;
                offset_top = 0.0;
            }
        }
    }
*/

    // calculate offset for top and left
    if let Some(parent) = current.parent() {
        if parent.borrow().flex_direction == FlexDirection::Row {
            *cur_offset_left += width;
        } else {
            *cur_offset_top += height;
        }     
    }

    // println!("{:?} - {:?}", offset_top, offset_left);

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
    let new_max_z = z_index_current_node + cur_z_stepping;

    let new_offset_left = offset_left.clone();
    let new_offset_top = offset_top.clone();
    let mut offset_top_zeroed = 0.0;
    let mut offset_left_zeroed = 0.0;
    
    for (index, node) in current.children().enumerate() {
        rectangles.append(&mut ui_screen_to_dp_list(&node, z_index_current_node, new_max_z, 
                                                     children_count as u32, index as u32,
                                                     self_width, self_height,
                                                     new_offset_left, new_offset_top,
                                                     &mut offset_left_zeroed, &mut offset_top_zeroed)); 
    }

    rectangles.push(cur_rect);

    return rectangles;
}

// without rendering: 36 ns / iter
// with rendering (use "vblank_mode=0 cargo bench") to get correct results: ~1.9 ms
#[bench]
fn bench_ui_screen_layout_simple(b: &mut test::Bencher) {

    use input;

    const INITIAL_WIDTH: u32 = 600;
    const INITIAL_HEIGHT: u32 = 800;
    let mut window_state = input::WindowState::new(INITIAL_WIDTH, INITIAL_HEIGHT);

    // Construct the explorer UI
    let mut ui_screen = UiScreen::new(INITIAL_WIDTH, INITIAL_HEIGHT)
                            .with_root_as_column();

    // Top bar, 100 - 200 pixels tall, stretches full window
    let top_bar_wrapper = NodeRef::new(NodeData::new(
            None, None, None, None, None, None, 
            FlexDirection::Column, DebugColor::green()));

    // Main explorer view, stretches all sides
    let explorer_wrapper = NodeRef::new(NodeData::new(
        None, None, None, None, None, None, 
        FlexDirection::Row, DebugColor::blue()));

            // navigation side bar
            let navigation_pane = NodeRef::new(NodeData::new(
                Some(500.0), None, Some(700.0), None, None, None, 
                FlexDirection::Column, DebugColor::red()));

            // file list
            let file_list_view = NodeRef::new(NodeData::new(
                None, None, None, None, Some(50.0), None, 
                FlexDirection::Column, DebugColor::blue()));

    // drawing order
    explorer_wrapper.append(navigation_pane);
    explorer_wrapper.append(file_list_view);
    ui_screen.root.append(top_bar_wrapper);
    ui_screen.root.append(explorer_wrapper);

    b.iter(|| {
            let event = glium::glutin::Event::Resized(rand::random::<u32>(), rand::random::<u32>());
            input::handle_event(&event, &mut window_state, &mut ui_screen);
            let _ = ui_screen.into_rectangles();
        }
    )
}
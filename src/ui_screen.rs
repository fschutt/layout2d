//! Ui screen is a single screen that is visible at one time
//! Uses rctree for reference counted nodes

extern crate rctree;

use rctree::NodeRef;

#[derive(Debug)]
pub struct NodeData {
    pub min_width_rem: u32,
    pub min_height_rem: u32,
    pub max_width_rem: u32,
    pub max_height_rem: u32,
}

impl NodeData {
    pub fn new(min_width_rem: u32, 
               min_height_rem: u32, 
               max_width_rem: u32, 
               max_height_rem: u32)
    -> Self {
        Self {
            min_width_rem,
            min_height_rem,
            max_width_rem,
            max_height_rem,
        }
    }
}

/// UI screen 
#[derive(Debug)]
pub struct UiScreen {
    pub root: NodeRef<NodeData>
}

impl UiScreen {

    /// Creates a new UiScreen
    #[inline]
    pub fn new() -> Self {
        Self {
            // min: 600 x 400, max: 800 x 1024
            root: NodeRef::new(NodeData::new(600, 400, 800, 1024)),
        }
    }

    /// Refreshes the UiScreen, returns if the frame has to be redrawn or not
    #[inline]
    pub(crate) fn layout(&mut self)
    -> bool 
    {
        for ref edge in self.root.traverse() {
            match *edge {
                rctree::NodeEdge::Start(ref data) => { println!("{:?}", data); },
                rctree::NodeEdge::End(ref data) => { println!("{:?}", data); },
            }
        }

        true
    }
}
use debug::DebugColor;

/// Which way the rectangles should flex
#[derive(Debug, PartialEq)]
pub enum FlexDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct NodeData {
    pub min_width_rem: Option<u32>,
    pub min_height_rem: Option<u32>,
    pub max_width_rem: Option<u32>,
    pub max_height_rem: Option<u32>,

    /// Width of the node. 
    /// **WARNING**: Must be initialized for the root node
    pub width: Option<u32>,
    /// Width of the node. 
    /// **WARNING**: Must be initialized for the root node
    pub height: Option<u32>,
    /// What direction the children should flex to
    pub flex_direction: FlexDirection,
    /// **DEBUG** color of the node
    pub debug_color: DebugColor,
}

impl NodeData {
    /// Creates a new node
    pub fn new(min_width_rem: Option<u32>, 
               min_height_rem: Option<u32>, 
               max_width_rem: Option<u32>, 
               max_height_rem: Option<u32>,
               width: Option<u32>,
               height: Option<u32>,
               flex_direction: FlexDirection,
               debug_color: DebugColor)
    -> Self {
        Self {
            min_width_rem,
            min_height_rem,
            max_width_rem,
            max_height_rem,
            width,
            height,
            flex_direction,
            debug_color,
        }
    }
}
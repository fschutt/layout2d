use debug::DebugColor;

#[derive(Debug)]
pub struct NodeData {
    pub min_width_rem: Option<u32>,
    pub min_height_rem: Option<u32>,
    pub max_width_rem: Option<u32>,
    pub max_height_rem: Option<u32>,

    pub debug_color: DebugColor,
}

impl NodeData {
    pub fn new(min_width_rem: Option<u32>, 
               min_height_rem: Option<u32>, 
               max_width_rem: Option<u32>, 
               max_height_rem: Option<u32>,
               debug_color: DebugColor)
    -> Self {
        Self {
            min_width_rem,
            min_height_rem,
            max_width_rem,
            max_height_rem,
            debug_color,
        }
    }
}
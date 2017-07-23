/// Which way the rectangles should flex
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FlexDirection {
    Column,
    Row,
}

#[derive(Debug, Copy, Clone)]
pub struct NodeData<T: Copy + Clone> {
    /// Minimum width of this node
    pub min_width: Option<f32>,
    /// Minimum height of this node
    pub min_height: Option<f32>,
    /// Maximum width of this node
    pub max_width: Option<f32>,
    /// Maximum height of this node
    pub max_height: Option<f32>,
    /// Width of the node (must be initialized for the root node)
    pub width: Option<f32>,
    /// Width of the node. (must be initialized for the root node)
    pub height: Option<f32>,
    /// What direction the children should flex to
    pub flex_direction: FlexDirection,
    /// Abstract data of the node, defined by the renderer / application (not inside this library)
    pub data: T,
}

impl<T: Copy + Clone> NodeData<T> {
    /// Creates a new node
    pub fn new(min_width: Option<f32>, 
               min_height: Option<f32>, 
               max_width: Option<f32>, 
               max_height: Option<f32>,
               width: Option<f32>,
               height: Option<f32>,
               flex_direction: FlexDirection,
               data: T)
    -> Self {
        Self {
            min_width,
            min_height,
            max_width,
            max_height,
            width,
            height,
            flex_direction,
            data,
        }
    }
}
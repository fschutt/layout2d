//! Ui screen is a single screen that is visible at one time
//! Uses rctree for reference counted nodes
extern crate rctree;
extern crate glium;

use rctree::NodeRef;
use node_data::NodeData;
use debug::DebugColor;
use renderer::Vertex;

/// UI screen 
#[derive(Debug)]
pub struct UiScreen {
    pub root: NodeRef<NodeData>
}

impl UiScreen {

    /// Creates a new UiScreen
    #[inline]
    pub fn new() 
    -> Self 
    {
        Self {
            // min: 600 x 400, max: 800 x 1024
            root: NodeRef::new(NodeData::new(Some(600), Some(400), Some(800), Some(1024), DebugColor::yellow())),
        }
    }

    /// Refreshes the UiScreen, returns if the frame has to be redrawn or not
    #[inline]
    pub(crate) fn layout(&mut self)
    -> bool 
    {
        for ref edge in self.root.traverse() {
            match *edge {
                rctree::NodeEdge::Start(ref data) => { println!("start: {:?}", data); },
                rctree::NodeEdge::End(ref data) => { println!("end: {:?}", data); },
            }
        }

        true
    }

    /// Converts the UI into a vertex buffer
    pub fn into_vertex_buffer(&self, display: &glium::Display)
    -> glium::VertexBuffer<Vertex>
    {
        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [ 0.0,  0.5] };
        let vertex3 = Vertex { position: [ 0.5, -0.25] };
        let shape = vec![vertex1, vertex2, vertex3];

        // todo: we cant use uniforms because uniforms are limited
        glium::VertexBuffer::new(display, &shape).unwrap()
    }
}
//! Ui screen is a single screen that is visible at one time
//! Uses rctree for reference counted nodes
extern crate rctree;
extern crate glium;
extern crate cgmath;

use rctree::NodeRef;
use node_data::NodeData;
use debug::DebugColor;
use renderer::Vertex;
use cgmath::{Matrix4, Vector4, Point3};

/// UI screen 
#[derive(Debug)]
pub struct UiScreen {
    pub root: NodeRef<NodeData>
}

/// A finite rectangle in pixel coordinates that will end up on the screen
#[derive(Debug)]
pub struct Rect {
    /// Top left corner
    tl: Point3<f32>,
    /// Bottom right corner
    br: Point3<f32>,
}

impl ::std::convert::Into<Vec<Vertex>> for Rect {
    fn into(self)
    -> Vec<Vertex>
    {
        return vec![
            Vertex { position: [self.tl.y,    self.tl.x] }, /*top left*/
            Vertex { position: [self.br.y,    self.tl.x] }, /*bottom left*/
            Vertex { position: [self.tl.y,    self.br.x] }, /*top right*/
            
            Vertex { position: [self.br.y,    self.br.x] }, /*bottom right*/
            Vertex { position: [self.tl.y,    self.br.x] }, /*top right*/
            Vertex { position: [self.br.y,    self.tl.x] }, /*bottom left*/
        ];
    }
}

/*
impl ::std::ops::Mul<cgmath::Matrix4<f32>> for Rect {
    type Output = Rect;
    fn mul(self, rhs: cgmath::Matrix4<f32>) -> Self::Output {

    }
}
*/

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
        /*
        for ref edge in self.root.traverse() {
            match *edge {
                rctree::NodeEdge::Start(ref data) => { println!("start: {:?}", data); },
                rctree::NodeEdge::End(ref data) => { println!("end: {:?}", data); },
            }
        }
        */

        true
    }

    /// Converts the UI into a vertex buffer
    pub fn into_vertex_buffer(&self, display: &glium::Display)
    -> glium::VertexBuffer<Vertex>
    {
        // todo: use the &self to convert the final layout into rectangles
        // that are then submitted to the renderer

        let rect = Rect { 
            tl: Point3 { y: 200.0, /* top */ x: 400.0 /* left*/, z: 0.0 },     /* todo: z-indexing */
            br: Point3 { y: 400.0, /* bottom */ x: 200.0 /* right*/, z: 0.0 },
        };

        // construct tranformation matrix
        let matrix = Matrix4::<f32> {
            x: Vector4{ x: 1.0, 
                        y: 0.0, 
                        z: 0.0,
                        w: 0.0},

            y: Vector4{ x: 0.0,
                        y: 1.0,
                        z: 0.0,
                        w: 0.0},

            z: Vector4{ x: 0.0,
                        y: 0.0,
                        z: 1.0,
                        w: 0.0,
            },

            w: Vector4 { x: 0.0,
                         y: 0.0,
                         z: 0.0,
                         w: 1.0,
            }
        };

        // let res = rect * matrix;

        let shape: Vec<Vertex> = rect.into();
        glium::VertexBuffer::new(display, &shape).unwrap()
    }
}
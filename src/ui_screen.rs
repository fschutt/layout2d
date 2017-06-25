//! Ui screen is a single screen that is visible at one time
//! Uses rctree for reference counted nodes
extern crate rctree;
extern crate glium;
extern crate simd;

use rctree::NodeRef;
use node_data::NodeData;
use debug::DebugColor;
use renderer::Vertex;

/// UI screen 
#[derive(Debug)]
pub struct UiScreen {
    pub root: NodeRef<NodeData>
}

#[derive(Debug)]
pub struct Point2 {
    x: f32,
    y: f32,
}

/// A finite rectangle in pixel coordinates that will end up on the screen
#[derive(Debug)]
pub struct Rect {
    z_index: f32,
    /// Top left corner
    tl: Point2,
    /// Top right corner
    tr: Point2,
    /// Bottom left corner
    bl: Point2,
    /// Bottom right corner
    br: Point2,
}

impl Rect {
    /// Creates a new rectangle
    pub fn new(top: f32, bottom: f32, left: f32, right: f32, z: f32)
    -> Self
    {
        Self {
            tl: Point2 { x: left, y: top },
            tr: Point2 { x: right, y: top },
            bl: Point2 { x: left, y: bottom },
            br: Point2 { x: right, y: bottom },
            z_index: z,
        }
    }

    // rotates the rectangle around its center
    pub fn rotate_center(&mut self, in_angle: f32)
    {
        let center_y = ((self.tr.y - self.bl.y) * 0.5) + self.bl.y;
        let center_x = ((self.tr.x - self.bl.x) * 0.5) + self.bl.x;

        let mut simd_x_dir = simd::f32x4::new(self.tl.x, self.tr.x, self.bl.x, self.br.x);
        let mut simd_y_dir = simd::f32x4::new(self.tl.y, self.tr.y, self.bl.y, self.br.y);

        // move all points to origin
        simd_x_dir = simd_x_dir - simd::f32x4::splat(center_x);
        simd_y_dir = simd_y_dir - simd::f32x4::splat(center_y);

        // calculate rotation
        let k_angle = in_angle.to_radians();
        let s = k_angle.sin();
        let c = k_angle.cos();

        let mut simd_x_new = (simd_x_dir * simd::f32x4::splat(c)) - (simd_y_dir * simd::f32x4::splat(s));
        let mut simd_y_new = (simd_x_dir * simd::f32x4::splat(s)) + (simd_y_dir * simd::f32x4::splat(c));

        simd_x_new = simd_x_new + simd::f32x4::splat(center_x);
        simd_y_new = simd_y_new + simd::f32x4::splat(center_y);

        self.tl.x = simd_x_new.extract(0); self.tr.x = simd_x_new.extract(1); self.bl.x = simd_x_new.extract(2); self.br.x = simd_x_new.extract(3);
        self.tl.y = simd_y_new.extract(0); self.tr.y = simd_y_new.extract(1); self.bl.y = simd_y_new.extract(2); self.br.y = simd_y_new.extract(3);
    }

    // translates a rectangle
    pub fn translate(&mut self, x: f32, y: f32)
    {
        let mut simd_x_dir = simd::f32x4::new(self.tl.x, self.tr.x, self.bl.x, self.br.x);
        let mut simd_y_dir = simd::f32x4::new(self.tl.y, self.tr.y, self.bl.y, self.br.y);

        simd_x_dir = simd_x_dir + simd::f32x4::splat(x);
        simd_y_dir = simd_y_dir + simd::f32x4::splat(y);

        self.tl.x = simd_x_dir.extract(0); self.tr.x = simd_x_dir.extract(1); self.bl.x = simd_x_dir.extract(2); self.br.x = simd_x_dir.extract(3);
        self.tl.y = simd_y_dir.extract(0); self.tr.y = simd_y_dir.extract(1); self.bl.y = simd_y_dir.extract(2); self.br.y = simd_y_dir.extract(3);
    }
}

impl ::std::convert::Into<Vec<Vertex>> for Rect {
    fn into(self)
    -> Vec<Vertex>
    {
        return vec![
            Vertex { position: [self.tl.x,    self.tl.y, self.z_index] }, /*top left*/
            Vertex { position: [self.bl.x,    self.bl.y, self.z_index] }, /*bottom left*/
            Vertex { position: [self.tr.x,    self.tr.y, self.z_index] }, /*top right*/
            
            Vertex { position: [self.br.x,    self.br.y, self.z_index] }, /*bottom right*/
            Vertex { position: [self.tr.x,    self.tr.y, self.z_index] }, /*top right*/
            Vertex { position: [self.bl.x,    self.bl.y, self.z_index] }, /*bottom left*/
        ];
    }
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
        
        
        let rect = Rect::new(200.0, 400.0, 400.0, 600.0, 0.0);
        let shape: Vec<Vertex> = rect.into();
        glium::VertexBuffer::new(display, &shape).unwrap()
    }
}
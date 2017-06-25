extern crate simd;
extern crate test;
extern crate rand;
extern crate glium;

use renderer::Vertex;
use test::Bencher;

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

    pub fn rotate_center_no_simd(&mut self, in_angle: f32)
    {
        let center_y = ((self.tr.y - self.bl.y) * 0.5) + self.bl.y;
        let center_x = ((self.tr.x - self.bl.x) * 0.5) + self.bl.x;

        self.tl.x -= center_x; self.tr.x -= center_x; self.bl.x -= center_x; self.br.x -= center_x;
        self.tl.y -= center_y; self.tr.y -= center_y; self.bl.y -= center_y; self.br.y -= center_y;

        // calculate rotation
        let k_angle = in_angle.to_radians();
        let s = k_angle.sin();
        let c = k_angle.cos();

        let tl_x = (self.tl.x * c) - (self.tl.y * s);
        let tr_x = (self.tr.x * c) - (self.tr.y * s);
        let bl_x = (self.bl.x * c) - (self.bl.y * s);
        let br_x = (self.br.x * c) - (self.br.y * s);

        let tl_y = (self.tl.x * s) + (self.tl.y * c);
        let tr_y = (self.tr.x * s) + (self.tr.y * c);
        let bl_y = (self.bl.x * s) + (self.bl.y * c);
        let br_y = (self.br.x * s) + (self.br.y * c);

        self.tl.x = tl_x; self.tr.x = tr_x; self.bl.x = bl_x; self.br.x = br_x;
        self.tl.y = tl_y; self.tr.y = tr_y; self.bl.y = bl_y; self.br.y = br_y;

        self.tl.x += center_x; self.tr.x += center_x; self.bl.x += center_x; self.br.x += center_x;
        self.tl.y += center_y; self.tr.y += center_y; self.bl.y += center_y; self.br.y += center_y;
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
        simd_y_dir = (simd_x_dir * simd::f32x4::splat(s)) + (simd_y_dir * simd::f32x4::splat(c));

        simd_x_new = simd_x_new + simd::f32x4::splat(center_x);
        simd_y_dir = simd_y_dir + simd::f32x4::splat(center_y);

        self.tl.x = simd_x_new.extract(0); self.tr.x = simd_x_new.extract(1); self.bl.x = simd_x_new.extract(2); self.br.x = simd_x_new.extract(3);
        self.tl.y = simd_y_dir.extract(0); self.tr.y = simd_y_dir.extract(1); self.bl.y = simd_y_dir.extract(2); self.br.y = simd_y_dir.extract(3);
    }

    pub fn translate_no_simd(&mut self, x: f32, y: f32)
    {
        self.tr.x += x; self.tl.x += x; self.br.x += x; self.bl.x += x;
        self.tr.y += y; self.tl.y += y; self.br.y += x; self.bl.y += y;
    }

    // translates a rectangle
    pub fn translate(&mut self, x: f32, y: f32)
    {
        let simd_x_dir = simd::f32x4::new(self.tl.x, self.tr.x, self.bl.x, self.br.x) + simd::f32x4::splat(x);
        let simd_y_dir = simd::f32x4::new(self.tl.y, self.tr.y, self.bl.y, self.br.y) + simd::f32x4::splat(y);

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

#[bench]
fn bench_rotate_center(b: &mut Bencher) {
    let mut rand_angles = Vec::new();
    for _ in 0..1000 {
        rand_angles.push(rand::random::<f32>());
    }

    let mut rect = Rect::new(200.0, 400.0, 400.0, 600.0, 0.0);

    b.iter(|| { 
        for elem in rand_angles.iter() {
            rect.rotate_center(*elem);
        }
    })
}

#[bench]
fn bench_rotate_center_no_simd(b: &mut Bencher) {
    let mut rand_angles = Vec::new();
    for _ in 0..1000 {
        rand_angles.push(rand::random::<f32>());
    }

    let mut rect = Rect::new(200.0, 400.0, 400.0, 600.0, 0.0);

    b.iter(|| { 
        for elem in rand_angles.iter() {
            rect.rotate_center_no_simd(*elem);
        }
    })
}

#[bench]
fn bench_translate(b: &mut Bencher) {
    let mut rand_angles = Vec::new();
    for _ in 0..1000 {
        rand_angles.push(rand::random::<f32>());
    }

    let mut rect = Rect::new(200.0, 400.0, 400.0, 600.0, 0.0);

    b.iter(|| { 
        for elem in rand_angles.iter() {
            rect.translate(*elem, elem / 2.0);
        }
    })
}

#[bench]
fn bench_translate_no_simd(b: &mut Bencher) {
    let mut rand_angles = Vec::new();
    for _ in 0..1000 {
        rand_angles.push(rand::random::<f32>());
    }

    let mut rect = Rect::new(200.0, 400.0, 400.0, 600.0, 0.0);

    b.iter(|| { 
        for elem in rand_angles.iter() {
            rect.translate_no_simd(*elem, elem / 2.0);
        }
    })
}
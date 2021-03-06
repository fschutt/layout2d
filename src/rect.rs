use node_data::NodeData;

/// A finite rectangle in pixel coordinates that will end up on the screen
#[derive(Debug, Clone)]
pub struct Rect<T: Clone> {
    /// x coordinates - as an array because of simd layout
    /// tl, tr, bl, br
    pub x: [f32; 4],
    /// y coordinates - as an array because of simd layout
    /// tl, tr, bl, br
    pub y: [f32; 4],
    /// Z-index is an int in order to achieve z-order sortability
    pub z: f32,
    /// Internal data of the rectangle
    pub data: NodeData<T>,
}

impl<T: Clone> Rect<T> {

    /// Creates a new rectangle
    #[inline]
    pub fn new(top: f32, bottom: f32, left: f32, right: f32, z: f32, data: NodeData<T>)
    -> Self
    {
        Self {
            x: [left, right, left, right],
            y: [top, top, bottom, bottom],
            z: z,
            data: data,
        }
    }

    /// Creates a new rectangle with width / height instead of top / bottom
    #[inline]
    pub fn new_wh(offset_left: f32, offset_top: f32, width: f32, height: f32, z: f32, data: NodeData<T>)
    -> Self
    {
        let right = offset_left + width;
        let bottom = offset_top + height;
        Self::new(offset_top, bottom, offset_left, right, z, data)
    }

    // Rotates a rectangle around its center, no SIMD
    #[cfg(not(feature = "use_simd"))]
    pub fn rotate_center(&mut self, in_angle: f32)
    {
        let center_y = ((self.y[1] - self.y[2]) * 0.5) + self.y[2];
        let center_x = ((self.x[1] - self.x[2]) * 0.5) + self.x[2];

        self.x[0] -= center_x; self.x[1] -= center_x;
        self.x[2] -= center_x; self.x[3] -= center_x;

        self.y[0] -= center_y; self.y[1] -= center_y;
        self.y[2] -= center_y; self.y[3] -= center_y;

        // calculate rotation
        let k_angle = in_angle.to_radians();
        let s = k_angle.sin();
        let c = k_angle.cos();

        let tl_x = (self.x[0] * c) - (self.y[0] * s);
        let tr_x = (self.x[1] * c) - (self.y[1] * s);
        let bl_x = (self.x[2] * c) - (self.y[2] * s);
        let br_x = (self.x[3] * c) - (self.y[3] * s);

        let tl_y = (self.x[0] * s) + (self.y[0] * c);
        let tr_y = (self.x[1] * s) + (self.y[1] * c);
        let bl_y = (self.x[2] * s) + (self.y[2] * c);
        let br_y = (self.x[3] * s) + (self.y[3] * c);

        self.x[0] = tl_x; self.x[1] = tr_x; self.x[2] = bl_x; self.x[3] = br_x;
        self.y[0] = tl_y; self.y[1] = tr_y; self.y[2] = bl_y; self.y[3] = br_y;

        self.x[0] += center_x; self.x[1] += center_x; self.x[2] += center_x; self.x[3] += center_x;
        self.y[0] += center_y; self.y[1] += center_y; self.y[2] += center_y; self.y[3] += center_y;
    }

    // Rotates the rectangle around its center, using SIMD
    #[cfg(feature = "use_simd")]
    pub fn rotate_center(&mut self, in_angle: f32)
    {
        use simd;

        let center_y = ((self.y[0] - self.y[2]) * 0.5) + self.y[2];
        let center_x = ((self.x[1] - self.x[0]) * 0.5) + self.x[0];

        let mut simd_x_dir = simd::f32x4::load(&self.x, 0);
        let mut simd_y_dir = simd::f32x4::load(&self.y, 0);

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

        simd_x_new.store(&mut self.x, 0);
        simd_y_dir.store(&mut self.y, 0);
    }

    // Translates a rectangle, no SIMD support
    #[cfg(not(feature = "use_simd"))]
    pub fn translate(&mut self, x: f32, y: f32)
    {
        self.x[0] += x; self.x[1] += x; self.x[2] += x; self.x[3] += x;
        self.y[0] += y; self.y[1] += y; self.y[2] += y; self.y[3] += x;
    }

    // Translates a rectangle, with SIMD
    #[cfg(feature = "use_simd")]
    pub fn translate(&mut self, x: f32, y: f32)
    {
        use simd;

        let simd_x_dir = simd::f32x4::load(&self.x, 0) + simd::f32x4::splat(x);
        let simd_y_dir = simd::f32x4::load(&self.y, 0) + simd::f32x4::splat(y);

        simd_x_dir.store(&mut self.x, 0);
        simd_y_dir.store(&mut self.y, 0);
    }

    // Convenience function for abstracting over the weird memory layout thing

    /// Set width from top left corner
    /// Warning: may not work well after rotations
    #[inline]
    pub fn set_width(&mut self, width: f32)
    {
        self.x[1] = self.x[0] + width;
        self.x[3] = self.x[2] + width;
    }

    /// Set height from top left corner
    /// Warning: may not work well after rotations
    #[inline]
    pub fn set_height(&mut self, height: f32)
    {
        self.y[2] = self.y[0] + height;
        self.y[3] = self.y[1] + height;
    }
}

impl<T: Clone> From<NodeData<T>> for Rect<T> {
    fn from(data: NodeData<T>)
    -> Self
    {
        Self {
            x: [0.0; 4],
            y: [0.0; 4],
            z: 0.0,
            data: data,
        }
    }
}

// without SIMD: ~23 µs
// with SIMD: ~14 µs
#[bench]
fn bench_rotate_center(b: &mut test::Bencher) {
    let mut rect = Rect::new(200.0, 400.0, 400.0, 600.0, 0.0, DebugColor::yellow());

    b.iter(|| {
        rect.rotate_center(rand::random::<f32>());
    })
}

// without SIMD: ~3 µs
// with SIMD: ~1.4 µs
#[bench]
fn bench_translate(b: &mut test::Bencher) {
    let mut rect = Rect::new(200.0, 400.0, 400.0, 600.0, 0.0, DebugColor::yellow());

    b.iter(|| {
        rect.translate(rand::random::<f32>(), rand::random::<f32>());
    })
}

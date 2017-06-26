extern crate glium;
extern crate image;

use glium::DisplayBuild;
use input::WindowState;
use ui_screen::UiScreen;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub debug_color: [f32; 4],
}

implement_vertex!(Vertex, position, tex_coords, debug_color);

// shaders to be compiled at renderer startup
const VERTEX_SHADER_SRC_BASIC: &'static str = include_str!("shaders/vertex_basic.glsl");
const VERTEX_SHADER_SRC_IMAGE: &'static str = include_str!("shaders/vertex_image.glsl");
const FRAGMENT_SHADER_SRC_BASIC: &'static str  = include_str!("shaders/fragment_basic.glsl");
const FRAGMENT_SHADER_SRC_IMAGE: &'static str  = include_str!("shaders/fragment_image.glsl");

const INDEX_BUFFER: glium::index::NoIndices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

pub struct Renderer {
    pub display: glium::Display,
    shader_programs: HashMap<&'static str, glium::Program>,
}

impl Renderer {
    
    /// Creates a new renderer
    pub fn new(width: u32, height: u32)
    -> Self
    {
        let disp = glium::glutin::WindowBuilder::new()
            .with_srgb(Some(false))
            .with_min_dimensions(200, 50)
            .with_vsync()
            .with_dimensions(width, height)
            .with_title("File Explorer")
            .with_multisampling(4)
            .build_glium()
            .unwrap();

        // list of shader programs we will use in this 
        let mut shader_programs = HashMap::<&'static str, glium::Program>::new();

        shader_programs.insert("basic", glium::Program::from_source(&disp, VERTEX_SHADER_SRC_BASIC, FRAGMENT_SHADER_SRC_BASIC, None).unwrap());
        shader_programs.insert("image", glium::Program::from_source(&disp, VERTEX_SHADER_SRC_IMAGE, FRAGMENT_SHADER_SRC_IMAGE, None).unwrap());
        
        Self {
            shader_programs: shader_programs,
            display: disp,
        }
    }

    /// Loads an image from an in-memory buffer
    /// Use it: "renderer.load_image_png(include_bytes!("my_image.png"))"
    pub fn load_image_png(&self, input: &'static [u8])
    -> glium::Texture2d 
    {
        use std::io::Cursor;
        let image = image::load(Cursor::new(input),
                                image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        glium::texture::Texture2d::new(&self.display, image).unwrap()
    }

    /// Renders to the screen
    pub fn render(&self, 
                  window_state: &WindowState, 
                  ui_screen: &UiScreen,
                  image: Option<&glium::Texture2d>)
    {
        use glium::Surface;
        use glium::draw_parameters::DrawParameters;

        let mut target = self.display.draw();
        target.clear_color(1.0, 1.0, 1.0, 0.0);

        // get vertices, must be changed every draw call, sadly
        let vertices = ui_screen.into_vertex_buffer(&self.display);

        let current_shader = { if image.is_some() {
            self.shader_programs.get("image").unwrap()
        } else {
            self.shader_programs.get("basic").unwrap()
        }};

        // choose uniforms based on image availability
        // this should be seperated so that every shader has his own uniform list
        let uniforms_normal = uniform!(
                window_width: window_state.width as f32, 
                window_height: window_state.height as f32,
        );

        let uniforms_image = { if let Some(image) = image {
            Some(uniform!( 
                window_width: window_state.width as f32, 
                window_height: window_state.height as f32,
                tex: image,
                transparency: 0.6_f32))
        } else { None }};

        if let Some(uniforms_image) = uniforms_image {
            // draw with image
            target.draw(&vertices, &INDEX_BUFFER, &current_shader, &uniforms_image, &DrawParameters {
                smooth: Some(glium::draw_parameters::Smooth::Nicest),
                blend: glium::draw_parameters::Blend::alpha_blending(),
                .. Default::default()
            }).unwrap();
        } else {
           // draw normal
           target.draw(&vertices, &INDEX_BUFFER, &current_shader, &uniforms_normal, &DrawParameters {
               smooth: Some(glium::draw_parameters::Smooth::Nicest),
               polygon_mode: glium::draw_parameters::PolygonMode::Line,
               .. Default::default()
           }).unwrap(); 
        }

        target.finish().unwrap();
    }
}

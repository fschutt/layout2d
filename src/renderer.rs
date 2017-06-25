extern crate glium;

use glium::DisplayBuild;
use input::WindowState;
use ui_screen::UiScreen;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);

const VERTEX_SHADER_SRC: &'static str = include_str!("shaders/vertex_basic.glsl");
const FRAGMENT_SHADER_SRC: &'static str  = include_str!("shaders/fragment_basic.glsl");
const INDEX_BUFFER: glium::index::NoIndices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

pub struct Renderer {
    pub display: glium::Display,
    shader_program: glium::Program,
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

        Self {
            shader_program: glium::Program::from_source(&disp, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap(),
            display: disp,
        }
    }

    /// Renders to the screen
    pub fn render(&self, 
                  window_state: &WindowState, 
                  ui_screen: &UiScreen)
    {
        use glium::Surface;
        use glium::draw_parameters::DrawParameters;

        let mut target = self.display.draw();
        target.clear_color(1.0, 1.0, 1.0, 0.0);

        // get vertices, must be changed every draw call, sadly
        let vertices = ui_screen.into_vertex_buffer(&self.display);

        let uniforms = uniform!( 
            window_width: window_state.width as f32, 
            window_height: window_state.height as f32
        );

        target.draw(&vertices, &INDEX_BUFFER, &self.shader_program, &uniforms, &DrawParameters {
            smooth: Some(glium::draw_parameters::Smooth::Nicest),
            .. Default::default()
        }).unwrap();

        target.finish().unwrap();
    }
}

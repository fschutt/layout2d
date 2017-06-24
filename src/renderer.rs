extern crate glium;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);

const VERTEX_SHADER_SRC: &'static str = include_str!("shaders/vertex_basic.glsl");
const FRAGMENT_SHADER_SRC: &'static str  = include_str!("shaders/fragment_basic.glsl");
const INDEX_BUFFER: glium::index::NoIndices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

/// Creates a shader program
#[inline]
pub fn setup(display: &glium::Display)
-> glium::Program
{
    return glium::Program::from_source(display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();
}

/// Renders to the screen
pub fn render(display: &glium::Display, 
              program: &glium::Program, 
              vertices: &glium::VertexBuffer<Vertex>)
{
    use glium::Surface;

    let mut target = display.draw();
    target.clear_color(1.0, 1.0, 1.0, 1.0);
    target.draw(vertices, &INDEX_BUFFER, program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
    target.finish().unwrap();
}
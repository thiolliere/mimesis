use glium::{self, Blend, Surface, VertexBuffer, index, vertex, Program, DrawParameters, Depth, DepthTest};
use glium::backend::{Facade, Context};
use glium::backend::glutin_backend::GlutinFacade;
use glium::program::ProgramChooserCreationError;

use std::error::Error;
use std::fmt;
use std::rc::Rc;

#[derive(Clone,Copy)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub struct Graphics {
    _context: Rc<Context>,

    quad_vertex_buffer: VertexBuffer<Vertex>,
    quad_indices: index::NoIndices,
    program: Program,

    draw_parameters: DrawParameters<'static>,
}

#[derive(Debug)]
pub enum GraphicsError {
    ProgramChooserCreation(ProgramChooserCreationError),
    VertexBufferCreation(vertex::BufferCreationError),
    IndexBufferCreation(index::BufferCreationError),
}

impl Error for GraphicsError {
    fn description(&self) -> &str {
        use self::GraphicsError::*;
        match *self {
            ProgramChooserCreation(ref err) => err.description(),
            VertexBufferCreation(ref err) => err.description(),
            IndexBufferCreation(ref err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&Error> {
        use self::GraphicsError::*;
        match *self {
            ProgramChooserCreation(ref e) => e.cause(),
            VertexBufferCreation(ref e) => e.cause(),
            IndexBufferCreation(ref e) => e.cause(),
        }
    }
}
impl fmt::Display for GraphicsError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::GraphicsError::*;
        match *self {
            ProgramChooserCreation(ref e) => write!(fmt, "Glium program chooser creation error: {}", e),
            VertexBufferCreation(ref e) => write!(fmt, "Glium vertex buffer creation error: {}", e),
            IndexBufferCreation(ref e) => write!(fmt, "Glium index buffer creation error: {}", e),
        }
    }
}
impl From<ProgramChooserCreationError> for GraphicsError {
    fn from(err: ProgramChooserCreationError) -> GraphicsError {
        GraphicsError::ProgramChooserCreation(err)
    }
}
impl From<index::BufferCreationError> for GraphicsError {
    fn from(err: index::BufferCreationError) -> GraphicsError {
        GraphicsError::IndexBufferCreation(err)
    }
}
impl From<vertex::BufferCreationError> for GraphicsError {
    fn from(err: vertex::BufferCreationError) -> GraphicsError {
        GraphicsError::VertexBufferCreation(err)
    }
}

impl Graphics {
    pub fn new(facade: &GlutinFacade) -> Result<Graphics, GraphicsError> {
        let quad_vertex = vec![Vertex { position: [-1., -1.] },
                               Vertex { position: [1., -1.] },
                               Vertex { position: [-1., 1.] },
                               Vertex { position: [1., 1.] }];
        let quad_vertex_buffer = VertexBuffer::new(facade, &quad_vertex)?;

        let quad_indices = index::NoIndices(index::PrimitiveType::TriangleStrip);

        let vertex_shader_src = r#"
            #version 100
            attribute vec2 position;
            uniform mat4 trans;
            void main() {
                gl_Position = trans * vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 100
            precision mediump float;
            uniform vec4 color;
            void main() {
                gl_FragColor = color;
            }
        "#;
        let program = program!(facade,
            100 => {
                vertex: vertex_shader_src,
                fragment: fragment_shader_src,
            },
        )?;

        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            depth: Depth {
                test: DepthTest::IfMoreOrEqual,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        Ok(Graphics {
            _context: facade.get_context().clone(),

            quad_vertex_buffer: quad_vertex_buffer,
            quad_indices: quad_indices,
            program: program,

            draw_parameters: draw_parameters,
        })
    }
}

pub struct Frame<'a> {
    pub frame: &'a mut glium::Frame,
    graphics: &'a mut Graphics,
}

impl<'a> Frame<'a> {
    pub fn new(graphics: &'a mut Graphics, frame: &'a mut glium::Frame) -> Frame<'a> {
        Frame {
            frame: frame,
            graphics: graphics,
        }
    }

    pub fn clear(&mut self) {
        self.frame.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 0f32);
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) {
        let trans = {
            [[width / 2.,          0., 0., 0.],
             [        0., height / 2., 0., 0.],
             [        0.,          0., 1., 0.],
             [         x,           y, 0., 1.]]
        };

        let uniform = uniform!{
            trans: trans,
            color: color,
        };

        self.frame
            .draw(&self.graphics.quad_vertex_buffer,
                  &self.graphics.quad_indices,
                  &self.graphics.program,
                  &uniform,
                  &self.graphics.draw_parameters)
            .unwrap();
    }

}

mod shader;
mod vertex;
mod wavefront;

use std::env;
use std::f32::consts::PI;

use cgmath::{perspective, EuclideanSpace, Matrix4, Point3, Rad, Vector3};
use luminance::context::GraphicsContext;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::TessSliceIndex as _;
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};

use crate::shader::ShaderInterface;
use crate::vertex::VertexSemantics;
use crate::wavefront::Obj;

fn render_loop(mut surface: GlfwSurface) {
    let path = env::args().skip(1)
                          .next()
                          .expect("First argument must be the path to the obj.");

    let fov = Rad(PI / 2.0);
    let z_near = 0.1;
    let z_far = 10.0;

    let projection = perspective(fov,
                                 surface.width() as f32 / surface.height() as f32,
                                 z_near,
                                 z_far);

    let mut eye = Point3::new(0.0, 0.5, 4.0);
    let center = Point3::origin();
    let up = Vector3::unit_y();
    let mut view = Matrix4::<f32>::look_at(eye, center, up);

    let mesh = Obj::load(path).unwrap().to_tess(&mut surface).unwrap();

    let vertex_shader = include_str!("vertex.glsl");
    let fragment_shader = include_str!("fragment.glsl");

    let program: Program<VertexSemantics, (), ShaderInterface> =
        Program::from_strings(None, vertex_shader, None, fragment_shader).unwrap()
                                                                         .ignore_warnings();

    let back_buffer = surface.back_buffer().unwrap();

    'run: loop {
        for event in surface.poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'run
                }
                _ => (),
            }
        }

        let color = [0.122, 0.173, 0.227, 1.0];

        surface.pipeline_builder()
               .pipeline(&back_buffer, color, |_, mut shd_gate| {
                   shd_gate.shade(&program, |interface, mut rdr_gate| {
                               interface.projection.update(projection.into());
                               interface.view.update(view.into());

                               rdr_gate.render(RenderState::default(), |mut tess_gate| {
                                           tess_gate.render(mesh.slice(..));
                                       });
                           });
               });

        surface.swap_buffers();
    }
}

fn main() {
    let surface = GlfwSurface::new(WindowDim::Windowed(1200, 900),
                                   "Fragula",
                                   WindowOpt::default());

    if let Ok(surface) = surface {
        render_loop(surface);
    } else {
        panic!("Could not create surface.");
    }
}

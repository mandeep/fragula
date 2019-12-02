mod shader;
mod vertex;
mod wavefront;

use std::env;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

use cgmath::{perspective, EuclideanSpace, Matrix4, Point3, Rad, SquareMatrix, Vector3};
use luminance::context::GraphicsContext;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::TessSliceIndex as _;
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};

use crate::shader::ShaderInterface;
use crate::vertex::VertexSemantics;
use crate::wavefront::Obj;

fn render_loop(mut surface: GlfwSurface) {
    let obj_path = env::args().skip(1)
                              .next()
                              .unwrap_or(String::from("suzanne.obj"));

    let fragment_path = env::args().skip(2)
                                   .next()
                                   .unwrap_or(String::from("src/fragment.glsl"));

    let fov = Rad(PI / 2.0);
    let z_near = 0.1;
    let z_far = 10.0;

    let projection = perspective(fov,
                                 surface.width() as f32 / surface.height() as f32,
                                 z_near,
                                 z_far);

    let eye = Point3::new(0.0, 0.5, 4.0);
    let center = Point3::origin();
    let up = Vector3::unit_y();
    let view = Matrix4::<f32>::look_at(eye, center, up);

    let (mut x_angle, mut y_angle, mut z_angle) = (1.0, 1.0, 1.0);
    let mut rotation: Matrix4<f32> = SquareMatrix::identity();

    let mut xyz_axis = Vector3::new(0.0, 0.0, 0.0);
    let mut translation: Matrix4<f32> = SquareMatrix::identity();

    let mesh = Obj::load(obj_path).unwrap().to_tess(&mut surface).unwrap();

    let vertex_shader = include_str!("vertex.glsl");

    let mut fragment_file = File::open(&fragment_path).unwrap();
    let mut contents = String::new();
    fragment_file.read_to_string(&mut contents).unwrap();
    let fragment_shader = &contents;

    let mut program: Program<VertexSemantics, (), ShaderInterface> =
        Program::from_strings(None, vertex_shader, None, fragment_shader).unwrap()
                                                                         .ignore_warnings();

    let back_buffer = surface.back_buffer().unwrap();

    'run: loop {
        for event in surface.poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'run
                }
                WindowEvent::Key(Key::W, _, Action::Release, _)
                | WindowEvent::Key(Key::W, _, Action::Repeat, _) => {
                    x_angle -= 1.0;
                    let rotation_angle = Rad(x_angle * PI / 180.0);
                    rotation = Matrix4::from_angle_x(rotation_angle);
                }
                WindowEvent::Key(Key::S, _, Action::Release, _)
                | WindowEvent::Key(Key::S, _, Action::Repeat, _) => {
                    x_angle += 1.0;
                    let rotation_angle = Rad(x_angle * PI / 180.0);
                    rotation = Matrix4::from_angle_x(rotation_angle);
                }
                WindowEvent::Key(Key::A, _, Action::Release, _)
                | WindowEvent::Key(Key::A, _, Action::Repeat, _) => {
                    y_angle -= 1.0;
                    let rotation_angle = Rad(y_angle * PI / 180.0);
                    rotation = Matrix4::from_angle_y(rotation_angle);
                }
                WindowEvent::Key(Key::D, _, Action::Release, _)
                | WindowEvent::Key(Key::D, _, Action::Repeat, _) => {
                    y_angle += 1.0;
                    let rotation_angle = Rad(y_angle * PI / 180.0);
                    rotation = Matrix4::from_angle_y(rotation_angle);
                }
                WindowEvent::Key(Key::Q, _, Action::Release, _)
                | WindowEvent::Key(Key::Q, _, Action::Repeat, _) => {
                    z_angle += 1.0;
                    let rotation_angle = Rad(z_angle * PI / 180.0);
                    rotation = Matrix4::from_angle_z(rotation_angle);
                }
                WindowEvent::Key(Key::E, _, Action::Release, _)
                | WindowEvent::Key(Key::E, _, Action::Repeat, _) => {
                    z_angle -= 1.0;
                    let rotation_angle = Rad(z_angle * PI / 180.0);
                    rotation = Matrix4::from_angle_z(rotation_angle);
                }
                WindowEvent::Key(Key::Z, _, Action::Release, _)
                | WindowEvent::Key(Key::Z, _, Action::Repeat, _) => {
                    xyz_axis.z -= 0.1;
                    translation = Matrix4::from_translation(xyz_axis);
                }
                WindowEvent::Key(Key::X, _, Action::Release, _)
                | WindowEvent::Key(Key::X, _, Action::Repeat, _) => {
                    xyz_axis.z += 0.1;
                    translation = Matrix4::from_translation(xyz_axis);
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
                               interface.translation.update(translation.into());
                               interface.rotation.update(rotation.into());

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

use std::path::Path;
use std::time::Instant;

use cgmath::{Deg, Euler, Matrix4, Point3, SquareMatrix, Vector3};
use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use luminance::tess::TessSliceIndex as _;
use luminance_glfw::{Action, GlfwSurface, Key, MouseButton, Surface, WindowEvent};

use crate::shader::{create_fragment_shader, create_shader_program};
use crate::texture::load_image;
use crate::transformations::{create_perspective_matrix, create_view_matrix};
use crate::watch::{create_channels, spawn_watcher};
use crate::wavefront::Obj;

pub fn render_loop(mut surface: GlfwSurface,
                   obj_path: &Path,
                   fragment_path: &Path,
                   texture_path: Option<&Path>,
                   resolution: [u32; 2]) {
    let mesh = Obj::load(obj_path).unwrap().to_tess(&mut surface).unwrap();

    let projection = create_perspective_matrix(0.1, 10.0, surface.width(), surface.height());
    let view = create_view_matrix(Point3::new(0.0, 0.5, 4.0));

    let vertex_shader = include_str!("vertex.glsl");
    let fragment_shader = create_fragment_shader(&fragment_path);
    let mut shader_program = create_shader_program(&vertex_shader.to_string(), &fragment_shader);

    let texture_image = load_image(&mut surface, texture_path);

    let (sender, receiver, messenger, collector) = create_channels();

    spawn_watcher(&fragment_path, sender, receiver, messenger);

    let (mut x_angle, mut y_angle, mut z_angle) = (0.0, 0.0, 0.0);
    let mut rotation: Matrix4<f32> = SquareMatrix::identity();

    let mut xyz_axis = Vector3::new(0.0, 0.0, 0.0);
    let mut translation: Matrix4<f32> = SquareMatrix::identity();

    let mut scalar = 1.0;
    let mut scale: Matrix4<f32> = SquareMatrix::identity();

    let mut back_buffer = surface.back_buffer().unwrap();
    let mut resize_buffer = false;
    let mut cursor_pressed = false;
    let mut cursor_moved = false;
    let (mut x_diff, mut y_diff) = (0.0f64, 0.0f64);

    let now = Instant::now();

    'run: loop {
        if resize_buffer {
            back_buffer = surface.back_buffer().unwrap();
            resize_buffer = false;
        }

        for event in surface.poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'run
                }
                WindowEvent::Key(Key::R, _, Action::Release, _)
                | WindowEvent::Key(Key::R, _, Action::Repeat, _) => {
                    xyz_axis = Vector3::new(0.0, 0.0, 0.0);
                    scalar = 1.0;

                    x_angle = 0.0;
                    y_angle = 0.0;
                    z_angle = 0.0;
                    let rotation_angle = Euler::new(Deg(x_angle), Deg(y_angle), Deg(z_angle));

                    rotation = Matrix4::from(rotation_angle);
                    translation = SquareMatrix::identity();
                    scale = SquareMatrix::identity();
                }
                WindowEvent::Key(Key::W, _, Action::Release, _)
                | WindowEvent::Key(Key::W, _, Action::Repeat, _) => {
                    x_angle -= 1.0;
                    let rotation_angle = Euler::new(Deg(x_angle), Deg(y_angle), Deg(z_angle));
                    rotation = Matrix4::from(rotation_angle);
                }
                WindowEvent::Key(Key::S, _, Action::Release, _)
                | WindowEvent::Key(Key::S, _, Action::Repeat, _) => {
                    x_angle += 1.0;
                    let rotation_angle = Euler::new(Deg(x_angle), Deg(y_angle), Deg(z_angle));
                    rotation = Matrix4::from(rotation_angle);
                }
                WindowEvent::Key(Key::A, _, Action::Release, _)
                | WindowEvent::Key(Key::A, _, Action::Repeat, _) => {
                    y_angle -= 1.0;
                    let rotation_angle = Euler::new(Deg(x_angle), Deg(y_angle), Deg(z_angle));
                    rotation = Matrix4::from(rotation_angle);
                }
                WindowEvent::Key(Key::D, _, Action::Release, _)
                | WindowEvent::Key(Key::D, _, Action::Repeat, _) => {
                    y_angle += 1.0;
                    let rotation_angle = Euler::new(Deg(x_angle), Deg(y_angle), Deg(z_angle));
                    rotation = Matrix4::from(rotation_angle);
                }
                WindowEvent::Key(Key::Q, _, Action::Release, _)
                | WindowEvent::Key(Key::Q, _, Action::Repeat, _) => {
                    z_angle += 1.0;
                    let rotation_angle = Euler::new(Deg(x_angle), Deg(y_angle), Deg(z_angle));
                    rotation = Matrix4::from(rotation_angle);
                }
                WindowEvent::Key(Key::E, _, Action::Release, _)
                | WindowEvent::Key(Key::E, _, Action::Repeat, _) => {
                    z_angle -= 1.0;
                    let rotation_angle = Euler::new(Deg(x_angle), Deg(y_angle), Deg(z_angle));
                    rotation = Matrix4::from(rotation_angle);
                }
                WindowEvent::Key(Key::Left, _, Action::Release, _)
                | WindowEvent::Key(Key::Left, _, Action::Repeat, _) => {
                    xyz_axis.x -= 0.1;
                    translation = Matrix4::from_translation(xyz_axis);
                }
                WindowEvent::Key(Key::Right, _, Action::Release, _)
                | WindowEvent::Key(Key::Right, _, Action::Repeat, _) => {
                    xyz_axis.x += 0.1;
                    translation = Matrix4::from_translation(xyz_axis);
                }
                WindowEvent::Key(Key::Down, _, Action::Release, _)
                | WindowEvent::Key(Key::Down, _, Action::Repeat, _) => {
                    xyz_axis.y -= 0.1;
                    translation = Matrix4::from_translation(xyz_axis);
                }
                WindowEvent::Key(Key::Up, _, Action::Release, _)
                | WindowEvent::Key(Key::Up, _, Action::Repeat, _) => {
                    xyz_axis.y += 0.1;
                    translation = Matrix4::from_translation(xyz_axis);
                }
                WindowEvent::Key(Key::Z, _, Action::Release, _)
                | WindowEvent::Key(Key::Z, _, Action::Repeat, _) => {
                    scalar -= 0.01;
                    scale = Matrix4::from_scale(scalar);
                }
                WindowEvent::Key(Key::X, _, Action::Release, _)
                | WindowEvent::Key(Key::X, _, Action::Repeat, _) => {
                    scalar += 0.01;
                    scale = Matrix4::from_scale(scalar);
                }
                WindowEvent::FramebufferSize(..) => {
                    resize_buffer = true;
                }
                WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
                    cursor_pressed = true;
                    cursor_moved = true;
                }
                WindowEvent::CursorPos(x, y) => {
                    if cursor_pressed {
                        if cursor_moved {
                            x_diff = x - y_angle as f64;
                            y_diff = y - x_angle as f64;
                            cursor_moved = false;
                        }

                        y_angle = (x - x_diff) as f32 * 0.8;
                        x_angle = (y - y_diff) as f32 * 0.8;

                        let rotation_angle = Euler::new(Deg(x_angle), Deg(y_angle), Deg(z_angle));
                        rotation = Matrix4::from(rotation_angle);
                    }
                }
                WindowEvent::MouseButton(MouseButton::Button1, Action::Release, _) => {
                    cursor_pressed = false;
                    cursor_moved = true;
                }
                _ => (),
            }
        }

        if !collector.is_empty() {
            let event = collector.recv().unwrap();
            match event.op {
                Ok(_) => {
                    let updated_fragment_shader = create_fragment_shader(&fragment_path);
                    shader_program =
                        create_shader_program(&vertex_shader.to_string(), &updated_fragment_shader);
                }
                Err(e) => println!("Error with event: {:?}", e),
            }
        }

        let color = [0.122, 0.173, 0.227, 1.0];
        let time = Instant::now().duration_since(now).as_secs_f32();

        surface.pipeline_builder().pipeline(
                                            &back_buffer,
                                            &PipelineState::default().set_clear_color(color),
                                            |pipeline, mut shd_gate| {
                                                shd_gate.shade(&shader_program,
                                                               |interface, mut rdr_gate| {

                               interface.projection.update(projection.into());
                               interface.view.update(view.into());
                               interface.translation.update(translation.into());
                               interface.rotation.update(rotation.into());
                               interface.scale.update(scale.into());
                               interface.time.update(time);
                               interface.resolution.update(resolution);

                               if let Some(image) = &texture_image {
                                   let bounded_texture = pipeline.bind_texture(&image);
                                   interface.texture_image.update(&bounded_texture);
                               }

                               rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                                           tess_gate.render(mesh.slice(..));
                                       });
                           });
                                            },
        );

        surface.swap_buffers();
    }
}

use std::env;

mod render;
mod shader;
mod transformations;
mod vertex;
mod wavefront;

use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowOpt};

use crate::render::render_loop;

fn main() {
    let obj_path = env::args().skip(1)
                              .next()
                              .expect("Error: Invalid OBJ file path.");

    let fragment_path = env::args().skip(2)
                                   .next()
                                   .expect("Error: Invalid fragment shader file path.");

    let surface = GlfwSurface::new(WindowDim::Windowed(1200, 900),
                                   "Fragula",
                                   WindowOpt::default());

    if let Ok(surface) = surface {
        render_loop(surface, obj_path, fragment_path);
    } else {
        panic!("Could not create surface.");
    }
}

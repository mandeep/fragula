#![feature(option_result_contains)]

mod render;
mod shader;
mod vertex;
mod wavefront;

use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowOpt};

use crate::render::render_loop;


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

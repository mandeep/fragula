use std::env;
use std::path::Path;

mod render;
mod shader;
mod transformations;
mod vertex;
mod watch;
mod wavefront;

use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowOpt};

use crate::render::render_loop;

fn main() {
    let resolution = [1200, 900];

    let obj =
        env::args().skip(1).next().expect("Error: Please provide a path to an Obj file.");

    let fragment =
        env::args().skip(2).next().expect("Error: Please provide a path to a fragment shader.");


    let obj_path = Path::new(&obj);
    let fragment_path = Path::new(&fragment);

    if obj_path.is_file() && fragment_path.is_file() {
        let surface = GlfwSurface::new(WindowDim::Windowed(resolution[0], resolution[1]),
                                       "Fragula",
                                       WindowOpt::default());

        if let Ok(surface) = surface {
            render_loop(surface, obj_path, fragment_path, resolution);
        } else {
            panic!("Could not create surface.");
        }
    } else {
        panic!("Error: Either the Obj path or the Fragment shader path is not valid.")
    }
}

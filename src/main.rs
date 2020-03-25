use std::path::Path;

mod render;
mod shader;
mod transformations;
mod vertex;
mod watch;
mod wavefront;

use clap::{Arg, App};
use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowOpt};

use crate::render::render_loop;

fn main() {
    let resolution = [1200, 900];

    let matches = App::new("Fragula")
        .version("0.6.0")
        .author("Mandeep <github.com/mandeep>")
        .arg(Arg::with_name("obj_file")
            .help("Obj file to load.")
            .required(true)
            .index(1)
            )
        .arg(Arg::with_name("fragment_shader")
            .help("Fragment shader to load.")
            .required(true)
            .index(2)
            )
        .get_matches();


    let obj = matches.value_of("obj_file").unwrap();
    let fragment = matches.value_of("fragment_shader").unwrap();


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

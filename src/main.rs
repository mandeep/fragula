use std::path::Path;

#[macro_use]
mod macros;
mod render;
mod shader;
mod texture;
mod transformations;
mod vertex;
mod watch;
mod wavefront;

use clap::{App, Arg};
use luminance_glfw::{GlfwSurface, Surface, WindowDim, WindowOpt};

use crate::render::render_loop;

fn main() {
    let resolution = [1200, 900];

    let matches =
        App::new(env!("CARGO_PKG_NAME"))
                           .version(env!("CARGO_PKG_VERSION"))
                           .author(env!("CARGO_PKG_AUTHORS"))
                           .about(env!("CARGO_PKG_DESCRIPTION"))
                           .arg(Arg::with_name("obj").help("The file path to the Obj file to load")
                                                     .required(true)
                                                     .index(1))
                           .arg(Arg::with_name("shader").help("The file path to the fragment \
                                                               shader to load")
                                                        .required(true)
                                                        .index(2))
                           .arg(Arg::with_name("texture").help("The file path to the texture \
                                                                image to load")
                                                         .short("t")
                                                         .long("texture")
                                                         .takes_value(true))
                           .get_matches();

    let obj = matches.value_of("obj").unwrap();
    let fragment = matches.value_of("shader").unwrap();
    let texture_path = if let Some(texture_image) = matches.value_of("texture") {
        Some(Path::new(texture_image))
    } else {
        None
    };

    let obj_path = Path::new(&obj);
    let fragment_path = Path::new(&fragment);

    if obj_path.is_file() && fragment_path.is_file() {
        let surface = GlfwSurface::new(WindowDim::Windowed(resolution[0], resolution[1]),
                                       "Fragula",
                                       WindowOpt::default());

        if let Ok(surface) = surface {
            render_loop(surface, obj_path, fragment_path, texture_path, resolution);
        } else {
            panic!("Could not create surface.");
        }
    } else {
        exit!("Error: Either the Obj path or the Fragment shader path is not valid.")
    }
}

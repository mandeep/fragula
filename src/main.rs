extern crate luminance;

use luminance::context::GraphicsContext;
use luminance_glfw::{Action, GlfwSurface, Key, Surface, WindowDim, WindowEvent, WindowOpt};
use std::process::exit;


fn main() {
    let surface = GlfwSurface::new(WindowDim::Windowed(1200, 900), "Fragula", WindowOpt::default());

    match surface {
        Ok(surface) => {
            event_loop(surface);
        }

        Err(e) => {
            eprintln!("Error creating surface.\n{}", e);
            exit(1);
        }
    }
}

fn event_loop(mut surface: GlfwSurface) {
    let back_buffer = surface.back_buffer().unwrap();

    'run: loop {
        for event in surface.poll_events() {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'run,
                _ => (),
            }
        }

        let color = [0.122, 0.173, 0.227, 1.0];

        surface.pipeline_builder().pipeline(&back_buffer, color, |_, _| ());
        surface.swap_buffers();
    }
}

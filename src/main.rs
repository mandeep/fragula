extern crate glium;

use glium::glutin;
use glium::Surface;


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_dimensions((400.0, 400.0).into())
        .with_title("Fragula");
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &events_loop).unwrap();

    let mut closed = false;

    while !closed {
        let mut target = display.draw();
        target.clear_color(0.122, 0.173, 0.227, 1.0);
        target.finish().unwrap();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent {event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });

    }
}

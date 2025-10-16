#![forbid(unsafe_code)]

use pixels::{Pixels, SurfaceTexture, Error};
use winit::{
    dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder
};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new().unwrap();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Conway's Game of Life")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels: Option<Pixels> = None;

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let res = event_loop.run(|event, elwt| {
        match event {
            Event::Resumed => {
                if pixels.is_none() {
                    let size = window.inner_size();
                    let surface = SurfaceTexture::new(size.width, size.height, &window);
                    pixels = Some(Pixels::new(WIDTH, HEIGHT, surface).unwrap());
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            },
            Event::AboutToWait => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                window.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
                if let Some(p) = pixels.as_mut() {
                    let frame = p.frame_mut();
                    frame[0] = 0xFF;
                    frame[600] = 0xAA;
                    frame[601] = 0xAA;
                    frame[602] = 0xAA;
                    frame[603] = 0xAA;
                    let _ = p.render();
                }
            },
            _ => ()
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

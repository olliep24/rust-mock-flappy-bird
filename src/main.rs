#![forbid(unsafe_code)]

mod game;
mod config;

use std::time::Instant;

use pixels::{Pixels, SurfaceTexture, Error};
use winit::{
    dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder
};
use game::Game;
use config::{WIDTH, HEIGHT, FIXED_DT};

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new().unwrap();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Flappy")
            .with_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    // Rendering and game state.
    let mut pixels: Option<Pixels> = None;
    let mut game: Game = Game::new();

    // Clock set up
    let mut last: Instant = Instant::now();
    let mut accum: f32 = 0.0;

    event_loop.set_control_flow(ControlFlow::Poll);

    let res = event_loop.run(|event, elwt| {
        match event {
            Event::Resumed => {
                // Have to create pixels inside the closure because it references window
                // If it was created outside the loop and it reference window there, it would be referencing
                // the memory where window was in main's stack (but it got moved to this closure).

                // This is the place to do it because Resumed marks the start of the window.
                if pixels.is_none() {
                    let size = window.inner_size();
                    let surface = SurfaceTexture::new(size.width, size.height, &window);
                    pixels = Some(Pixels::new(WIDTH as u32, HEIGHT as u32, surface).unwrap());
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
                // time
                let now = Instant::now();
                let mut dt = (now - last).as_secs_f32();
                last = now;
                if dt > 0.25 { dt = 0.25; }
                accum += dt;

                while accum >= FIXED_DT {
                    game.update(FIXED_DT);
                    accum -= FIXED_DT;
                }

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

                    // Clear the frame from the previous drawing:
                    frame.fill(0);
                    
                    game.draw(frame);
                    let _ = p.render();
                }
            },
            _ => ()
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

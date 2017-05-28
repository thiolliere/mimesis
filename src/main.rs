extern crate fps_clock;
#[macro_use] extern crate glium;

use glium::glutin;
use glium::DisplayBuild;

#[cfg(target_os = "emscripten")]
pub mod emscripten;
mod app;
mod graphics;
pub mod backtrace_hack;

pub trait OkOrExit {
    type OkType;
    fn ok_or_exit(self) -> Self::OkType;
}
impl<T, E: ::std::fmt::Display> OkOrExit for Result<T,E> {
    type OkType = T;
    fn ok_or_exit(self) -> T {
        match self {
            Ok(t) => t,
            Err(err) => {
                println!("ERROR: {}", err);
                ::std::process::exit(1);
            },
        }
    }
}

fn main() {
    safe_main().ok_or_exit();
}

fn safe_main() -> Result<(), String> {
    configure_fullscreen_strategy();
    let builder = glutin::WindowBuilder::new()
        .with_multitouch()
        .with_title("airjump");

    let window = builder.build_glium().map_err(|e| format!("build glium: {}", e))?;

    let mut graphics = graphics::Graphics::new(&window).map_err(|e| format!("graphics: {}", e))?;

    let mut app = app::App::new();

    // return whereas main loop breaks
    set_main_loop(|dt| -> bool {
        for event in window.poll_events() {
            use glium::glutin::Event::*;
            match event {
                Closed => return true,
                Touch(mut touch) => {
                    let (w, h) = window.get_window().unwrap().get_inner_size_points().unwrap();
                    touch.location.0 -= (w/2) as f64;
                    touch.location.0 /= (w/2) as f64;
                    touch.location.1 -= (h/2) as f64;
                    touch.location.1 /= (h/2) as f64;
                    touch.location.1 *= -1.;

                    app.touch(touch);
                },
                _ => (),
            }
        }

        app.update(dt);

        let mut target = window.draw();
        {
            let mut frame = graphics::Frame::new(&mut graphics, &mut target);
            frame.clear();
            app.draw(&mut frame);
        }
        target.finish().unwrap();

        return false
    });

    Ok(())
}

#[cfg(target_os = "emscripten")]
fn configure_fullscreen_strategy() {
    emscripten::request_soft_fullscreen_strategy();
}

#[cfg(not(target_os = "emscripten"))]
fn configure_fullscreen_strategy() {
}

#[cfg(target_os = "emscripten")]
fn set_main_loop<F: FnMut(f64) -> bool>(mut main_loop: F) {
    let dt = 1.0 / 60f64;
    emscripten::set_main_loop_callback(|| {
        if main_loop(dt) {
            emscripten::cancel_main_loop();
        }
    });
}

// behavior differ from emscripten as it doesn't return
// as long as the main loop doesn't end
#[cfg(all(not(target_os = "emscripten")))]
fn set_main_loop<F: FnMut(f64) -> bool>(mut main_loop: F) {
    // If running out of time then slow down the game
    let mut fps_clock = fps_clock::FpsClock::new(60);
    let dt = 1.0 / 60.0;
    loop {
        if main_loop(dt) {
            break
        }
        fps_clock.tick();
    }
}

extern crate fps_clock;
#[macro_use] extern crate glium;

use glium::glutin;
use glium::DisplayBuild;

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
    let mut builder = glutin::WindowBuilder::new()
        .with_multitouch()
        .with_title("airjump");

    let window = builder.build_glium().map_err(|e| format!("build glium: {}", e))?;

    // return whereas main loop breaks
    set_main_loop(|_dt| -> bool {
        for event in window.poll_events() {
            use glium::glutin::Event::*;
            use glium::glutin::TouchPhase;
            match event {
                Closed => return true,
                Touch(touch) => {
                },
                Refresh => {
                    let mut target = window.draw();
                    // {
                    //     let camera = app.camera();
                    //     let mut frame = graphics::Frame::new(&mut graphics, &mut target, &camera);
                    //     frame.clear();
                    //     app.draw(&mut frame);
                    // }
                    // target.finish().unwrap();
                }
                _ => (),
            }
        }

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

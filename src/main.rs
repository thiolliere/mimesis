#![recursion_limit="500"]
#[macro_use] extern crate stdweb;
extern crate winit;
use std::os::raw::c_void;
use std::ptr;
use std::cell::RefCell;

mod ffi;
mod app;

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(ptr::null_mut()));

fn set_main_loop_callback<F>(callback : F) where F : FnMut() {
    MAIN_LOOP_CALLBACK.with(|log| {
        *log.borrow_mut() = &callback as *const _ as *mut c_void;
    });

    unsafe { ::ffi::emscripten_set_main_loop(Some(wrapper::<F>), 0, 1); }

    unsafe extern "C" fn wrapper<F>() where F : FnMut() {
        MAIN_LOOP_CALLBACK.with(|z| {
            let closure = *z.borrow_mut() as *mut F;
            (*closure)();
        });
    }
}

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
    let mut events_loop = winit::EventsLoop::new();
    let mut app = app::App::new();

    set_main_loop_callback(|| {
        events_loop.poll_events(|event| {
            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::Touch(mut touch), ..
                } => {
                    let height = js! {return window.innerWidth};
                    let width = js! {return window.innerHeight};
                    touch.location.0 -= (width/2) as f64;
                    touch.location.0 /= (width/2) as f64;
                    touch.location.1 -= (height/2) as f64;
                    touch.location.1 /= (height/2) as f64;
                    touch.location.1 *= -1.;

                    app.touch(touch);
                },
                _ => (),
            }
        });

        app.update(1.0/60.0);
        app.draw();
    });

    Ok(())
}

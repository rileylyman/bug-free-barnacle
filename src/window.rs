use glfw::{Glfw, Window, WindowEvent, Context};
use std::cell::Cell;
use std::sync::mpsc::Receiver;

pub struct WindowState {
    glfw: Glfw,
    pub events: Receiver<(f64, WindowEvent)>,
    pub window: Window,
}

fn glfw_error_callback(err: glfw::Error, desc: String, count: &Cell<usize>) -> () {
    panic!("GLFW Error {:?}: {:?}", count.get(), desc);
    count.set(count.get() + 1);
}

impl WindowState {
    pub fn new(width: u32, height: u32, name: &'static str, window_mode: glfw::WindowMode) -> Result<Self, &'static str> {
        let  mut glfw = glfw::init(
            Some(glfw::Callback {
                f: glfw_error_callback as fn(glfw::Error, String, &Cell<usize>),
                data: Cell::new(0),
            })).map_err(|_| "Failed to init glfw!")?;
        let (mut window, events) = glfw.create_window(width, height, name, window_mode).ok_or("Failed to create window!")?;

        window.set_key_polling(true);
        window.make_current();

        Ok(WindowState {
            glfw,
            events,
            window,
        })
    }

    pub fn poll_events(&mut self) -> glfw::FlushedMessages<(f64, WindowEvent)> {
       self.glfw.poll_events();
       glfw::flush_messages(&self.events)
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn close(&mut self) -> () {
        self.window.set_should_close(true)
    }
}

impl Default for WindowState {
    fn default() -> Self {
        WindowState::new(800, 600, "My Window", glfw::WindowMode::Windowed).unwrap()
    }
}

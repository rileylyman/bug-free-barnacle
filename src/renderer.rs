
pub struct Renderer {

}

extern "system" fn gl_debug_callback(source: u32, _type: u32, id: u32, sev: u32, length: i32, msg: *const i8, data: *mut std::ffi::c_void) -> () {
    println!("Reached error");
    error!("jey");
}

impl Renderer {
    pub fn init_only_once(window: &mut glfw::Window) -> Result<Self, &'static str> {
       gl::load_with(|s| window.get_proc_address(s) as *const _ ); 
       unsafe {
           gl::DebugMessageCallback(gl_debug_callback, std::ptr::null());
       }
       Ok(Renderer {})
    }

    pub unsafe fn clear(&self, color: [f32; 4]) -> () {
        gl::ClearColor(color[0], color[1], color[2], color[3]);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn clear_screen(r: &Renderer, local: &super::localstate::LocalState) -> () {
    let clear_color = &local.clear_color;
    unsafe {
        r.clear((*clear_color).clone());
    }
}

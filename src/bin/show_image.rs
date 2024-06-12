
#[path = "../files/mod.rs"]
pub mod files;

#[path = "../window/mod.rs"]
pub mod window;

extern crate sdl2;

use once_cell::sync::Lazy;
use window::canvas::{create_canvas, image, render_window, GREEN};


fn main() {  
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    
    let mut canvas = create_canvas(&sdl_context);
    
    render_window(sdl_context, &mut canvas, *Lazy::force(&GREEN), &|tc|image(tc, "src/assets/lr.jpg"));
}

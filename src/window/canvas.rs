
extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::pixels::{self, Color};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use once_cell::sync::Lazy;

const TEXTURE_SIZE: u32 = 32;

pub static GREEN: Lazy<TextureColor> = Lazy::new(||TextureColor::Green(Color::RGB(0, 255,0)));
pub static BLUE: Lazy<TextureColor> = Lazy::new(||TextureColor::Blue(Color::RGB(0, 0, 255)));
pub static RED: Lazy<TextureColor> = Lazy::new(||TextureColor::Red(Color::RGB(255,0, 0)));

pub fn render_window_2(sdl_context: Sdl, canvas: &mut Canvas<Window>, canvas_color: TextureColor) {
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let green_square = create_texture_rect(canvas, &texture_creator, *Lazy::force(&GREEN), TEXTURE_SIZE);
    let blue_square = create_texture_rect(canvas, &texture_creator, *Lazy::force(&BLUE), TEXTURE_SIZE);
    let timer = SystemTime::now();
    green_square.map(|gsquare|{
        blue_square.map(|bsquare|{
            handle_event(sdl_context, ||{
                canvas.set_draw_color(color_of(canvas_color));
                canvas.clear();
                let square_texture = if show_green(timer) {&gsquare}else {&bsquare};
                let _ = canvas.copy(&square_texture, None, Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE));
                canvas.present();
            })
        })
    });
}

fn show_green(timer: SystemTime) -> bool {
    return match timer.elapsed() {
        Ok(elapsed) => elapsed.as_secs() % 2 == 0,
        Err(_) => true
    }
}

pub fn render_window(sdl_context: Sdl, canvas: &mut Canvas<Window>, canvas_color: TextureColor, image: &dyn Fn(&TextureCreator<WindowContext>) -> Option<Texture>) {
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    //create_texture_rect(canvas, &texture_creator, color, TEXTURE_SIZE)
    image(&texture_creator)    
    .map(|texture|{
        handle_event(sdl_context, ||{
            canvas.set_draw_color(color_of(canvas_color));
            canvas.clear();
            //canvas.copy(&texture, None, Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
            canvas.copy(&texture, None, None)
            .expect("Couldn't copy texture into window");
            canvas.present();
        })
    });
}

fn color_of(color: TextureColor) -> Color {
    return match color{
        TextureColor::Blue(b) => b,
        TextureColor::Green(g) => g,
        TextureColor::Red(r) => r,
   }
}
pub fn create_canvas(sdl_context: &Sdl) -> Canvas<Window> {
    let video_subsystem = sdl_context.video().expect("Could not get SDL video subsystem");
    let window = video_subsystem.window("rust-sdl2 demo",800, 450)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    return window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Failed to convert window into canvas");
}

#[derive(Clone, Copy)]
pub enum TextureColor{
    Green(pixels::Color), Red(pixels::Color), Blue(pixels::Color)
}


pub fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, color: TextureColor, size: u32) -> Option<Texture<'a>>{
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE) {        
        canvas.with_texture_canvas(&mut square_texture, |texture|{
            texture.set_draw_color(color_of(color));
            texture.clear();
        }).expect("Failed to clor a texture");
        Some(square_texture)
    } else {
        None
    }
}

pub fn handle_event<F>(sdl_context: Sdl, mut redraw: F) where F: FnMut() -> () {
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
    'running: loop {
        while let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }
        redraw();
        sleep(Duration::new(0, 1_000_000_000u32/60));
    }
}


pub fn image<'a>(texture_creator: &'a TextureCreator<WindowContext>, file_name: &'a str) -> Option<Texture<'a>> {
    let result = texture_creator.load_texture(file_name);
    if result.is_ok() {
        return result.ok();
    }
    println!("Image texture create failed:{}", result.err().unwrap());
    None
}
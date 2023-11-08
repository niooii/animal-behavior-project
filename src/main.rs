extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::TextureAccess;
use sdl2::rect::{Rect, Point};
use core::option::Option::None;
use sdl2::image;
use sdl2::render;
use std::time::Duration;

struct Vector2 {
    x: u16,
    y: u16
}

impl Vector2 {
    fn new(x: u16, y: u16) -> Vector2 {
        Vector2 { 
            x,
            y 
        }
    }
}

struct MoveTarget {
    target_pos: Vector2,

}

struct LanternFly {
    
}

fn render_lanternflies(flies: &Vec<LanternFly>) {
    
}

fn update_lanternflies(flies: &mut Vec<LanternFly>, click_buf: &mut Vec<Point>) {
    // Process new clicks (clicking on lanternflies)
    if !click_buf.is_empty() {
        println!("comparing {} clicks...", click_buf.len());
    }

    // Update positions of each and shit
    for fly in flies {

    }
}

pub fn main() -> Result<(), String> {
    
    // Fix on kde
    std::env::set_var("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");
    let sdl_context = sdl2::init()?;

    let sdl_image_context = image::init(image::InitFlag::all())?;

    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("lanternfly aahhbahabhab", 800, 600)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    // Initialize textures

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let tex = texture_creator.load_texture("resources/gaulsoodman.jpg")?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut flies = Vec::<LanternFly>::new();
    let mut click_buffer = Vec::<Point>::new();

    // Fix render loop
    'running: loop {

        // Clear clickbuffer
        click_buffer.clear();

        // HANDLE EVENTS
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Mouse button was clicked at ({x}, {y})!");
                    click_buffer.push(Point::new(x, y));
                },
                _ => {}
            }
        }

        canvas.clear();

        // DRAWING CODE
        
        render_lanternflies(&flies);
        canvas.copy(&tex, None, Some(Rect::new(0, 0, tex.query().width, tex.query().height)))?;

        canvas.present();

        // 60 fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        // LOGIC CODE
        
        update_lanternflies(&mut flies, &mut click_buffer);

    }

    Ok(())
}
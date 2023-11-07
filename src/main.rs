extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::sys::*;
use std::time::Duration;

struct LanternFly {

}

fn render_lanternflies(flies: &Vec<LanternFly>) {
    
}

fn update_lanternflies(flies: &mut Vec<LanternFly>, clicks: &mut Vec<SDL_Point>) {
    // Process new clicks (clicking on lanternflies)
    if !clicks.is_empty() {
        println!("comparing {} clicks...", clicks.len());
    }

    // Update positions of each and shit
    for fly in flies {
        
    }
}

pub fn main() -> Result<(), String> {
    std::env::set_var("SDL_VIDEO_X11_NET_WM_BYPASS_COMPOSITOR", "0");
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("lanternfly aahhbahabhab", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    // Initialize textures

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut flies = Vec::<LanternFly>::new();
    let mut click_buffer = Vec::<SDL_Point>::new();

    'running: loop {

        // Clear clickbuffer
        click_buffer.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => {
                    println!("Mouse button was clicked at ({x}, {y})!");
                    click_buffer.push(SDL_Point { x, y });
                },
                _ => {}
            }
        }

        canvas.clear();
        
        render_lanternflies(&flies);
        update_lanternflies(&mut flies, &mut click_buffer);

        canvas.present();

        // 60 fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...

    }

    Ok(())
}
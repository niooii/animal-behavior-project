extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{TextureAccess, Canvas, WindowCanvas};
use sdl2::video::{Window};
use sdl2::rect::{Rect, Point};
use core::option::Option::None;
use sdl2::image;
use sdl2::render::{Texture};
use std::time::Duration;

mod vector2;
use vector2::Vector2;

mod stopwatch;
use stopwatch::Stopwatch;

struct MoveTarget {
    target_pos: Vector2,
    move_time: u16,
}

struct Transform {
    x: u16,
    y: u16,
    rot: u16,
}

struct LanternFly {
    transform: Transform,
    move_target: Option<MoveTarget>,

}

impl LanternFly {
    fn new(x: u16, y: u16) -> LanternFly {
        LanternFly {  
            transform: Transform {
                x,
                y,
                rot: 0
            },
            move_target: None,

        }
    }
}

fn spawn_lanternfly(screen_bounds: &Rect) {
    // Choose a random point 100px outside the bounds, spawn randomly
}

fn render_lanternflies(canvas: &Canvas<Window>, flies: &Vec<LanternFly>, idle_tex: &Texture, wings_out_tex: &Texture) {
    // canvas.copy(texture, src, dst)
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

    // Initialize everything
    let sdl_context = sdl2::init()?;
    let image_context = image::init(image::InitFlag::all())?;
    let ttf_context = sdl2::ttf::init().expect("failed to init sdl_ttf");
    let video_subsystem = sdl_context.video()?;
    video_subsystem.display_bounds(0);

    let window = video_subsystem
        .window("Lanternfly abahabba", 800, 600)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    // Initialize textures

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let idle_tex = texture_creator.load_texture("resources/gaulsoodman.jpg")?;
    let wingsout_tex = texture_creator.load_texture("resources/gaulsoodman.jpg")?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut flies = Vec::<LanternFly>::new();
    let mut click_buffer = Vec::<Point>::new();

    // Initialize a stopwatch for deltatime
    let mut stopwatch = Stopwatch::new();
    let mut delta_time: f64 = 0.0;

    // render loop
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
        
        render_lanternflies(&canvas, &flies, &idle_tex, &wingsout_tex);


        // DRAWING CODE END
        canvas.present();
        

        // LOGIC CODE
        
        update_lanternflies(&mut flies, &mut click_buffer);

        limit_fps(stopwatch.elapsed_seconds(), 90.0);
        

        // get dt && reset stopwatch
        delta_time = stopwatch.elapsed_seconds();
        println!("{delta_time}");
        stopwatch.reset();
    }

    Ok(())
}

fn limit_fps(current_stopwatch_time: f64, fps: f64) {
    let sec_per_frame = 1_f64 / fps;
    if sec_per_frame > current_stopwatch_time {
        // Time to wait: seconds
        let ttw_s = sec_per_frame - current_stopwatch_time;

        std::thread::sleep(Duration::try_from_secs_f64(ttw_s).unwrap());
    }
}
#![allow(clippy::cast_possible_truncation)]

extern crate sdl2;

use core::option::Option::None;
use std::sync::MutexGuard;
use rand::Rng;
use sdl2::event::Event;
use sdl2::image;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use sdl2::render::{Canvas, TextureAccess, WindowCanvas};
use sdl2::video::Window;
use sdl2::mouse::MouseButton;  
use std::time::Duration;

mod vector2;
use vector2::Vector2;

mod stopwatch;
use stopwatch::Stopwatch;

mod statics;
use statics::{FLIES, FIRES, FORK};

const SCREEN_BOUNDS: (u32, u32) = (1200, 900);
const WALK_SPEED: f32 = 2.5;
const FLY_SPEED_LOWER: f32 = 10.0;
const FLY_SPEED_UPPER: f32 = 40.0;



#[derive(Copy, Clone)]
struct MoveTarget {
    target_pos: Vector2,
    original_pos: Vector2,
    speed: f32,
}
#[derive(Copy, Clone)]
pub struct Transform {
    pos: Vector2,
    rot: f32,
}

#[derive(Copy, Clone)]
pub struct LanternFly {
    transform: Transform,
    move_target: Option<MoveTarget>,
    time_moved: f32,
    time_since_move: f32,
    flying: bool,
}

impl LanternFly {
    fn new(x: i32, y: i32) -> LanternFly {
        LanternFly {
            transform: Transform { pos: Vector2::new(x as f32, y as f32), rot: 0.0 },
            move_target: None,
            time_moved: 0.0,
            time_since_move: 0.0,
            flying: false,
        }
    }

    fn fly_to(&mut self, x: f32, y: f32, spd: f32) {
        self.time_moved = 0.0;
        self.time_since_move = 0.0;
        self.move_target = Some(MoveTarget {
            target_pos: Vector2 { x, y },
            original_pos: self.transform.pos,
            // scale up speed
            speed: spd * 20.0
        });

        // calculate angle of rotation
        self.flying = true;
        self.transform.rot = self.transform.pos.lookat_angle(
            &self.move_target.unwrap().target_pos
        );       
    }

    fn walk_to(&mut self, x: f32, y: f32, spd: f32) {
        self.time_moved = 0.0;
        self.time_since_move = 0.0;
        self.move_target = Some(MoveTarget {
            target_pos: Vector2 { x, y },
            original_pos: self.transform.pos,
            speed: spd * 20.0
        });

        // calculate angle of rotation
        self.flying = false;
        self.transform.rot = self.transform.pos.lookat_angle(
            &self.move_target.unwrap().target_pos
        );       
    }
}

pub struct Fork {
    hertz: u32,
    transform: Transform,
}

fn spawn_lanternfly_outside(screen_bounds: &Rect) {
    // Choose a random point 100px outside the bounds, spawn randomly
}

fn spawn_lanternfly(x: i32, y: i32, tex: &Texture) {
    let mut rng = rand::thread_rng();
    let tex_info = tex.query();
    let mut flies = FLIES.lock().unwrap();

    let mut fly = LanternFly::new(x, y,);
    // get random move target
    fly.fly_to(rng.gen_range(tex_info.width as f32..(SCREEN_BOUNDS.0 - tex_info.width) as f32),
                rng.gen_range(tex_info.height as f32..(SCREEN_BOUNDS.1 - tex_info.height) as f32),
                rng.gen_range(FLY_SPEED_LOWER..FLY_SPEED_UPPER));
    flies.push(fly);
}


#[allow(clippy::cast_possible_truncation)]
fn render_scene(
    canvas: &mut Canvas<Window>,
    idle_tex: &Texture,
    flying_tex: &Texture,
    fire_tex: &Texture,
    fork_tex: &Texture
) {
    let flies = FLIES.lock().unwrap();
    let fires = FIRES.lock().unwrap();
    let fork = &FORK.lock().expect("could not get fork from mutex")
    .as_ref().unwrap();

    let fly_texinfo = idle_tex.query();
    let fork_texinfo = fork_tex.query();
    let mut dest = Rect::new(0, 0, fly_texinfo.width, fly_texinfo.height);

    for fly in flies.iter() {
        // change dest rectangle within canvas copy
        dest.x = fly.transform.pos.x as i32;
        dest.y = fly.transform.pos.y as i32;

        // then fly is flying. :=;
        let tex = if fly.move_target.is_some() && fly.flying {
            flying_tex
        } else {
            idle_tex
        };

        canvas.copy_ex(
            tex, 
            None, 
            Some(dest), 
    fly.transform.rot as f64, 
    None, 
    false, 
    false)
        .expect("failed to copy texture");
    }

    let fire_texinfo = fire_tex.query();
    dest.set_width(fire_texinfo.width);
    dest.set_height(fire_texinfo.height);

    for fire in fires.iter() {
        dest.set_x(fire.pos.x as i32);
        dest.set_y(fire.pos.y as i32);

        canvas.copy(
            fire_tex, 
            None, 
            Some(dest)
        )
        .expect("failed to copy texture");
    }
    
    // render fork
    dest.x = fork.transform.pos.x as i32;
    dest.y = fork.transform.pos.y as i32;
    dest.set_width(fork_texinfo.width);
    dest.set_height(fork_texinfo.height);
    canvas.copy(fork_tex, None, dest).expect("failed to copy texture");
}

fn update_scene(click_buf: &Vec<Point>, fly_tex: &Texture, fire_tex: &Texture, delta_time: f64) {
    
    // Process new clicks (clicking on lanternflies)
    
    let mut rng = rand::thread_rng();
    let fly_texinfo = fly_tex.query();
    let fire_texinfo = fire_tex.query();
    let mut query_rect = Rect::new(0, 0, fly_texinfo.width, fly_texinfo.height);
    
    

    let mut clicked: Vec<Transform> = Vec::new();

    for &click in click_buf {
        let mut flies = FLIES.lock().unwrap();
        for fly in flies.iter_mut() {
            query_rect.x = fly.transform.pos.x as i32;
            query_rect.y = fly.transform.pos.y as i32;
            
            if query_rect.contains_point(click) {
                println!("lanternfly has been clicked .>.");

                // get random move
                fly.fly_to(rng.gen_range(0_f32..(SCREEN_BOUNDS.0 - fly_texinfo.width) as f32),
                rng.gen_range(0_f32..(SCREEN_BOUNDS.1 - fly_texinfo.height) as f32),
                rng.gen_range(FLY_SPEED_LOWER..FLY_SPEED_UPPER));

                clicked.push(fly.transform);
            }
        }
    }

    for transform in clicked {
        spawn_lanternfly(transform.pos.x as i32, transform.pos.y as i32, fly_tex);
    }
    
    let mut flies = FLIES.lock().unwrap();
    let fork = FORK.lock().unwrap();
    // Handle fly movements
    for fly in flies.iter_mut() {

        // handles movements if fly is not moving.
        if fly.move_target.is_none() {
            fly.time_since_move += delta_time as f32;
            
            // if fork exists and hz is 60, 
            // move to fork.
            if fork.is_some() {
                let f = fork.as_ref().unwrap();
                if f.hertz == 60 {
                    fly.walk_to(f.transform.pos.x, f.transform.pos.y, WALK_SPEED);
                    continue;
                } 
                else {
                    //TODO: play sound
                }
            }
            // move fly randomly
            if fly.time_since_move > rng.gen_range(1.0..2.5) {
                // get random move
                fly.fly_to(rng.gen_range(0_f32..(SCREEN_BOUNDS.0 - fly_texinfo.width) as f32),
                rng.gen_range(0_f32..(SCREEN_BOUNDS.1 - fly_texinfo.height) as f32),
                rng.gen_range(FLY_SPEED_LOWER..FLY_SPEED_UPPER));
   
            }
            continue;
        }

        // actually handle moving the fly
        if fly.time_since_move > 0.0 {
            fly.time_since_move = 0.0;
        }

        let mt = fly.move_target.as_mut().unwrap();

        let total_move_time = mt.original_pos.distance(&mt.target_pos)/mt.speed;
        fly.transform.pos = Vector2::lerp_new(mt.original_pos, mt.target_pos, fly.time_moved/total_move_time);
        fly.time_moved += delta_time as f32;

        // If fly finished flying, remove the move target.
        if fly.time_moved >= total_move_time {
            fly.move_target = None;
            fly.flying = false;
        }
        // println!("lerping... {:?}", fly.transform.pos);
    }

    let fires = FIRES.lock().unwrap();
    let mut comp_rect = Rect::new(
        0,
        0,
        fire_texinfo.width,
        fire_texinfo.height
    );
    for fire in fires.iter() {
        // if fly is in fire rect, DIE !
        comp_rect.x = fire.pos.x as i32;
        comp_rect.y = fire.pos.y as i32;

        let mut target_idxs = Vec::<usize>::new();
        // O^2 tim but WHO CARES!
        for (i, fly) in flies.iter().enumerate() {
            if comp_rect.contains_rect(Rect::new(
                fly.transform.pos.x as i32,
                fly.transform.pos.y as i32,
                fly_texinfo.width,
                fly_texinfo.height,
            )) {
                target_idxs.push(i);
            }
        }   

        target_idxs.sort_by(|a, b| b.cmp(a));
        for idx in target_idxs   {
            flies.remove(idx);
        }
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
    let audio_subsystem = sdl_context.audio()?;
    // video_subsystem.display_bounds(0);

    let window = video_subsystem
        .window("Lanternfly abahabba", SCREEN_BOUNDS.0, SCREEN_BOUNDS.1)
        .position_centered()
        .opengl()
        // .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    // Initialize textures

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let tree_tex = texture_creator.load_texture("resources/tree.png")?;
    let idle_tex = texture_creator.load_texture("resources/fly_closed.png")?;
    let flying_tex = texture_creator.load_texture("resources/fly_open.png")?;
    let fire_tex = texture_creator.load_texture("resources/fire.png")?;
    let fork_tex = texture_creator.load_texture("resources/fork.png")?;
    let fork_texinfo = fork_tex.query();

    // other vars
    let mut rbutton_down = false;

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let mut event_pump = sdl_context.event_pump()?;

    let mut flies = FLIES.lock().unwrap();
    let mut fires = FIRES.lock().unwrap();
    let mut fork = FORK.lock().unwrap();
    let mut click_buffer = Vec::<Point>::new();

    // Initialize a stopwatch for deltatime
    let mut stopwatch = Stopwatch::new();
    let mut delta_time: f64 = 0.0;

    // add objects in game
    for _ in 0..2_i32.pow(8) {
        flies.push(LanternFly::new(200, 300));
    }

    fires.push(Transform { 
        pos: Vector2 {
            x: 400.0,
            y: 400.0,
        }, 
        rot: 0.0
    });

    *fork = Some(Fork {
        hertz: 60,
        transform: Transform {
            pos: Vector2::new(400.0, 400.0),
            rot: 0.0,
        }
    });

    drop(flies);
    drop(fires);
    drop(fork);

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
                Event::MouseButtonDown { x, y, mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            println!("Mouse button was clicked at ({x}, {y})!");
                            click_buffer.push(Point::new(x, y));
                        },
                        MouseButton::Right => {
                            rbutton_down = true;
                        }
                        _ => println!("unknown button type wtf"),
                    }
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, .. } => {
                    rbutton_down = false;
                }
                // Move fork somewhere
                Event::MouseMotion { x, y, xrel, yrel, .. } => {
                    if !rbutton_down {
                        continue;
                    }

                    // let fork = FORK.lock().expect("failed to get fork struct")
                    // .unwrap();
                    // let query_rect = Rect::new(
                    //     fork.transform.pos.x as i32,
                    //     fork.transform.pos.y as i32,
                    //      )
                },
                _ => {}
            }
        }

        canvas.clear();

        // DRAWING CODE

        render_scene(&mut canvas, &idle_tex, &flying_tex, &fire_tex);

        // DRAWING CODE END
        canvas.present();

        // LOGIC CODE

        update_scene(&click_buffer, &idle_tex, &fire_tex, delta_time);

        limit_fps(stopwatch.elapsed_seconds(), 90.0);

        // get dt && reset stopwatch
        delta_time = stopwatch.elapsed_seconds();
        // println!("{delta_time}");
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

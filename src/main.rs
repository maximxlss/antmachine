use winit::{event::{Event, WindowEvent}, event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder}};
use pixels::{SurfaceTexture, Pixels};
use std::{sync::{Arc, atomic::{AtomicBool, Ordering::Relaxed}}, thread, time::{Instant}};
use antmachine::{ants::World};
//use rand::distributions::{Distribution, Uniform};
use parking_lot::Mutex;

static WIDTH: u32 = 256;
static HEIGHT: u32 = 256;
static ANTS: usize = 256;

fn main() {
    let event_loop = EventLoop::new();
    let win = WindowBuilder::new()
        .with_title("Ants!")
        .with_maximized(true)
        .build(&event_loop).unwrap();
    let size = win.inner_size();

    let width = size.width;
    let height = size.height;

    let surface_texture = SurfaceTexture::new(width, height, &win);
    let pixels = Arc::new(Mutex::new(Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()));

    let event_pix = pixels.clone();

    //let start = Instant::now();
    let world = Arc::new(Mutex::new(World::new(ANTS)));

    let world_clone = world.clone();
    let is_resized = Arc::new(AtomicBool::new(false));
    let is_resized_event = is_resized.clone();

    thread::spawn(move || {
        let mut evolution = 0;
        //let ants = Uniform::from(0..ANTS);
        //let mut rng = rand::thread_rng();
        loop {
            let evo_time = Instant::now();
            {
                let mut world = world.lock();
                world.evolve_threaded(16);
                evolution += 1;

                println!("{} ants\t{} pheromones\t{} evolution\t{} micros to compute",
                         world.ants.len(), world.pheromones.len(), 
                         evolution, evo_time.elapsed().as_micros())
            }
        }
    });

    thread::spawn(move || {
        loop {
            if is_resized.load(std::sync::atomic::Ordering::Relaxed) {
                continue;
            }
            let mut pixels = pixels.lock();
            let mut frame: Vec<Vec<&mut [u8]>> = pixels.get_frame()
                                                       .chunks_exact_mut(WIDTH as usize * 4)
                                                       .map(|x| x.chunks_exact_mut(4).collect())
                                                       .collect();
            for row in frame.iter_mut() {
                for px in row {
                    px.copy_from_slice(&[0x00, 0x00, 0x00, 0xFF]);
                }
            }
            {
                let world = world_clone.lock();
                for ph in &world.pheromones {
                    frame[(ph.pos.y * HEIGHT as f64) as usize]
                         [(WIDTH as f64 - (ph.pos.x * WIDTH as f64)) as usize]
                         .copy_from_slice(&[(ph.pow * 128.) as u8, (ph.pow * 128.) as u8, (ph.pow * 128.) as u8, 0xFF])
                }
                for ant in &world.ants {
                    frame[(ant.pos.y * HEIGHT as f64) as usize]
                         [(WIDTH as f64 - (ant.pos.x * WIDTH as f64)) as usize]
                         .copy_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF])
                }
            }
            pixels.render().unwrap();
        }
    });

    event_loop.run(move |e, _, cf| {
        *cf = ControlFlow::Wait;
        match e {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *cf = ControlFlow::Exit,
            Event::WindowEvent { event: WindowEvent::Resized(size), ..} => {
                is_resized_event.store(true, Relaxed);
                let width = size.width;
                let height = size.height;
                let mut pixels = event_pix.lock();
                pixels.resize_surface(width, height);
                pixels.resize_buffer(WIDTH, HEIGHT)
            }
            _ => {
                if is_resized_event.load(Relaxed) {
                    is_resized_event.store(false, Relaxed)
                };
            },
        }
    })
}

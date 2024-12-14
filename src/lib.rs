#![allow(dead_code)]

use std::{fmt::{Debug, Display}, sync::{Arc, Mutex}};

use sdl::{render::Canvas, video::Window, Sdl, VideoSubsystem};

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = core::result::Result<T, E>;

pub fn init() -> Result<App> {
    let title = Arc::new(format!("Snake {}", env!("CARGO_PKG_VERSION")));
    let size = (800, 600);

    let sdl = sdl::init()?;
    let video = sdl.video()?;

    let window = video
        .window(title.clone().as_str(), size.0, size.1)
        .allow_highdpi()
        //.opengl()
        .position_centered()
        .build()?;

    let window = Arc::new(window);

    let canvas = <sdl::video::Window as Clone>::clone(&window)
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()?;
    
    Ok(App {
        sdl: Arc::new(sdl),
        video_subsystem: Arc::new(video),
        window,
        canvas: Arc::new(Mutex::new(canvas)),

        title,
        size,
    })
}

#[derive(Clone)]
pub struct App {
    sdl: Arc<Sdl>,
    video_subsystem: Arc<VideoSubsystem>,
    window: Arc<Window>,
    canvas: Arc<Mutex<Canvas<Window>>>,

    title: Arc<String>,
    size: (u32, u32),
}

impl App {
    pub fn title(&self) -> String {
        self.title.clone().to_string()
    }

    pub fn set_title(&mut self, title: String) {
        if title.is_empty() {
            eprintln!("WARNING: title shouldn't be empty!");
            self.title = Arc::new("title".to_string());
            return;
        }

        self.title = Arc::new(title);
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn set_size(&mut self, size: (u32, u32)) {
        if size.0 < 50 {
            eprintln!("WARNING: width (size.0) shouldn't be under 50 pixels!");
            self.size.0 = 50;
        }
           
        if size.1 < 50 {
            eprintln!("WARNING: height (size.1) shouldn't be under 50 pixels!");
            self.size.1 = 50;
            return;
        }

        self.size = size;
    }
}

impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "App {{\n  [...],\n\n  title: Arc {{ String {{ \"{}\" }} }},\n  size: ({}, {})\n}}",
                    self.title, self.size.0, self.size.1)
    }
}

impl Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "App \"{}\" ({}x{})", self.title, self.size.0, self.size.1)
    }
}

use vizia::prelude::*;

pub fn start_gui() -> Result<(), Box<dyn std::error::Error>> {
    let mut dvd = DVD::new();
    Application::new(move |cx| {
        dvd.start(cx);
    })
    .title("Home Menu")
    .run()?;
    Ok(())
}

enum AppState {
    Off,
    ScreenSaver(Box<dyn ScreenSaver>),
}

















use std::{thread, time};

pub trait ScreenSaver {
    fn start(&mut self, cx: &mut Context) {
        self.init(cx);
        self.update(cx);
        // thread::sleep(std::time::Duration::from_millis(10));
        self.end(cx);
    }
    fn update(&mut self, cx: &mut Context) -> bool;
    fn init(&mut self, cx: &mut Context);
    fn end(&mut self, cx: &mut Context);
}

pub struct DVD {
    pub pos: (f32, f32),
    dir: (bool, bool),
    speed: f32,
}

impl DVD {
    pub fn new() -> Self {
        DVD {
            pos: (0.0, 0.0),
            dir: (true, true),
            speed: 1.0,
        }
    }
    pub fn new_speed(spd: f32) -> Self {
        DVD {
            pos: (0.0, 0.0),
            dir: (true, true),
            speed: spd
        }
    }
}

impl ScreenSaver for DVD {
    fn update(&mut self, cx: &mut Context) -> bool {
        let width = 500.0;
        let height = 500.0;

        self.pos.0 += if self.dir.0 { self.speed } else { -self.speed };
        self.pos.1 += if self.dir.1 { self.speed } else { -self.speed };
        self.dir.0 ^= self.pos.0 <= -width || self.pos.0 >= width;
        self.dir.1 ^= self.pos.1 <= -height || self.pos.1 >= height;

        Label::new(cx, format!("x: {}  -  y: {}", self.pos.0, self.pos.1));
        false
    }


    fn init(&mut self, cx: &mut Context) {
        // Initialize the DVD screen saver
    }

    fn end(&mut self, cx: &mut Context) {
        // End the DVD screen saver
    }
}
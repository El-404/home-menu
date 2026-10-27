pub struct WindowManager {
    pub ui: i32,
    state: AppState,
}

impl WindowManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(WindowManager {
            ui: 2, 
            state: AppState::Off,
        })
    }
}

enum AppState {
    Off,
    ScreenSaver,
}

















use std::{thread, time};

pub trait ScreenSaver {
    fn start(&mut self, interupt: fn() -> bool) {
        self.init();
        loop {
            if self.update() || interupt() { break; }
            thread::sleep(std::time::Duration::from_millis(10));
        }
        self.end();
    }
    fn update(&mut self) -> bool;
    fn init(&mut self);
    fn end(&mut self);
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
    fn update(&mut self) -> bool {
        let width = 500.0;
        let height = 500.0;

        self.pos.0 += if self.dir.0 { self.speed } else { -self.speed };
        self.pos.1 += if self.dir.1 { self.speed } else { -self.speed };
        self.dir.0 ^= self.pos.0 <= -width || self.pos.0 >= width;
        self.dir.1 ^= self.pos.1 <= -height || self.pos.1 >= height;

        false
    }


    fn init(&mut self) {
        // Initialize the DVD screen saver
    }

    fn end(&mut self) {
        // End the DVD screen saver
    }
}
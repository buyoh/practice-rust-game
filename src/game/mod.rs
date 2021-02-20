pub(crate) mod basic;
pub(crate) mod course;
mod machine;

#[derive(Clone)]
pub struct GameDisplayInfo {
    pub player: basic::Entity,
    pub camera: basic::Entity,
    pub course: course::Course,
    pub frame_sec: f64,
}

impl GameDisplayInfo {
    pub fn default() -> GameDisplayInfo {
        GameDisplayInfo {
            player: basic::Entity {
                x: 0.0,
                y: 0.0,
                angle: 0.0,
            },
            camera: basic::Entity {
                x: 1.0,
                y: 0.0,
                angle: 0.0,
            },
            course: course::Course::default(),
            frame_sec: 0.0,
        }
    }
}

pub struct Game {
    display_info: GameDisplayInfo,
    player: machine::Machine,
    key_up: bool,
    key_down: bool,
    key_left: bool,
    key_right: bool,
}

impl Game {
    pub fn new() -> Game {
        let mut player = machine::Machine::new();
        player.entity.x = 0.0;
        player.entity.y = 20.0;
        Game {
            // displayInfo: Rc::<RefCell<GameDisplayInfo>>::new(RefCell::<GameDisplayInfo>::new(
            //     GameDisplayInfo::new(),
            // )),
            display_info: GameDisplayInfo::default(),
            player: player,
            key_up: false,
            key_down: false,
            key_left: false,
            key_right: false,
        }
    }
    pub fn get_display_info(&self) -> GameDisplayInfo {
        self.display_info.clone()
    }
    pub fn tick(&mut self, frame_sec: f64) {
        self.player.accsel =
            (if self.key_up { 1.0 } else { 0.0 }) + (if self.key_down { -1.0 } else { 0.0 });
        self.player.steer =
            (if self.key_right { 1.0 } else { 0.0 }) + (if self.key_left { -1.0 } else { 0.0 });
        self.player.tick();
        self.display_info.player = self.player.entity.clone();
        self.display_info.camera.x =
            (self.display_info.player.x - 50.0 * self.display_info.player.angle.cos()) * 0.1
                + (self.display_info.camera.x) * 0.9;
        self.display_info.camera.y =
            (self.display_info.player.y - 50.0 * self.display_info.player.angle.sin()) * 0.1
                + (self.display_info.camera.y) * 0.9;
        self.display_info.frame_sec = frame_sec;
    }
    pub fn handle_key_press_event(&mut self, key: gdk::keys::Key) {
        // match key {
        //     gdk::keys::constants::leftarrow => self.key_left = true,
        //     gdk::keys::constants::uparrow => self.key_up = true,
        //     gdk::keys::constants::rightarrow => self.key_right = true,
        //     gdk::keys::constants::downarrow => self.key_down = true,
        //     _ => (),
        // };
        // TODO: 妥協策
        match key.name().unwrap().as_str() {
            "Left" => self.key_left = true,
            "Up" => self.key_up = true,
            "Right" => self.key_right = true,
            "Down" => self.key_down = true,
            _ => (),
        };
    }
    pub fn handle_key_release_event(&mut self, key: gdk::keys::Key) {
        // match key {
        //     gdk::keys::constants::leftarrow => self.key_left = false,
        //     gdk::keys::constants::uparrow => self.key_up = false,
        //     gdk::keys::constants::rightarrow => self.key_right = false,
        //     gdk::keys::constants::downarrow => self.key_down = false,
        //     _ => (),
        // };
        match key.name().unwrap().as_str() {
            "Left" => self.key_left = false,
            "Up" => self.key_up = false,
            "Right" => self.key_right = false,
            "Down" => self.key_down = false,
            _ => (),
        };
    }
    // pub fn handle_mouse_down_event(&mut self, point: (f64, f64)) {}
    // pub fn handle_mouse_up_event(&mut self, point: (f64, f64)) {}
}

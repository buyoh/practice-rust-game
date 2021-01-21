use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

mod basic;
mod machine;

pub struct GameDisplayInfo {
    pub player: basic::Entity,
}

impl GameDisplayInfo {
    pub fn default() -> GameDisplayInfo {
        GameDisplayInfo {
            player: basic::Entity {
                x: 0.0,
                y: 0.0,
                angle: 0.0,
            },
        }
    }
}

pub struct Game {
    // displayInfo: Rc<RefCell<GameDisplayInfo>>,
    player: machine::Machine,
    key_up: bool,
    key_down: bool,
    key_left: bool,
    key_right: bool,
}

impl Game {
    pub fn new() -> Game {
        let mut player = machine::Machine::new();
        player.entity.x = 200.0;
        player.entity.y = 200.0;
        Game {
            // displayInfo: Rc::<RefCell<GameDisplayInfo>>::new(RefCell::<GameDisplayInfo>::new(
            //     GameDisplayInfo::new(),
            // )),
            player: player,
            key_up: false,
            key_down: false,
            key_left: false,
            key_right: false,
        }
    }
    pub fn get_display_info(&self) -> GameDisplayInfo {
        GameDisplayInfo {
            player: self.player.entity.clone(),
        }
    }
    pub fn tick(&mut self) {
        self.player.accsel =
            (if self.key_up { 1.0 } else { 0.0 }) + (if self.key_down { -1.0 } else { 0.0 });
        self.player.steer =
            (if self.key_right { 1.0 } else { 0.0 }) + (if self.key_left { -1.0 } else { 0.0 });
        self.player.tick();
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

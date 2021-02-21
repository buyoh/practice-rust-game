pub(crate) mod basic;
pub(crate) mod course;
mod input;
mod machine;

use input::Key;

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
    // TODO: Game が別スレッドから参照されるのはdisplay_infoとinputだけである
    // フィールドに排他制御を付けるべき（排他制御を最小にするべき）
    display_info: GameDisplayInfo,
    player: machine::Machine,
    input: input::Input,
}

impl Game {
    pub fn new() -> Game {
        let mut player = machine::Machine::new();
        player.entity.x = 0.0;
        player.entity.y = 20.0;
        Game {
            display_info: GameDisplayInfo::default(),
            player: player,
            input: input::Input::new(),
        }
    }
    pub fn get_display_info(&self) -> GameDisplayInfo {
        self.display_info.clone()
    }
    pub fn tick(&mut self, frame_sec: f64) {
        // TODO: Fieldを使って管理する
        self.player.accsel = (if self.input.is_pressed(Key::Up) {
            1.0
        } else {
            0.0
        }) + (if self.input.is_pressed(Key::Down) {
            -1.0
        } else {
            0.0
        });
        self.player.steer = (if self.input.is_pressed(Key::Right) {
            1.0
        } else {
            0.0
        }) + (if self.input.is_pressed(Key::Left) {
            -1.0
        } else {
            0.0
        });
        self.player.tick();
        self.display_info.player = self.player.entity.clone();
        self.display_info.camera.x =
            (self.display_info.player.x - 50.0 * self.display_info.player.angle.cos()) * 0.1
                + (self.display_info.camera.x) * 0.9;
        self.display_info.camera.y =
            (self.display_info.player.y - 50.0 * self.display_info.player.angle.sin()) * 0.1
                + (self.display_info.camera.y) * 0.9;
        self.display_info.frame_sec = frame_sec;
        self.input.tick();
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
            "Left" => self.input.press(Key::Left),
            "Up" => self.input.press(Key::Up),
            "Right" => self.input.press(Key::Right),
            "Down" => self.input.press(Key::Down),
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
            "Left" => self.input.release(Key::Left),
            "Up" => self.input.release(Key::Up),
            "Right" => self.input.release(Key::Right),
            "Down" => self.input.release(Key::Down),
            _ => (),
        };
    }
    // pub fn handle_mouse_down_event(&mut self, point: (f64, f64)) {}
    // pub fn handle_mouse_up_event(&mut self, point: (f64, f64)) {}
}

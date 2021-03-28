pub(crate) mod basic;
pub(crate) mod course;
mod machine;

use crate::input::{Input, InputInfo, Key};

#[derive(Clone)]
pub struct GameRenderInfo {
    pub player: basic::Entity,
    pub camera: basic::Entity,
    pub course: course::Course,
    pub frame_sec: f64,
}

impl GameRenderInfo {
    pub fn default() -> GameRenderInfo {
        GameRenderInfo {
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
    render_info: GameRenderInfo,
    player: machine::Machine,
    // game_input: input::GameInput,
    input: Input, // bad refcell...
    input_info: InputInfo,
}

impl Game {
    pub fn new(input: Input) -> Game {
        let mut player = machine::Machine::new();
        player.entity.x = 0.0;
        player.entity.y = 20.0;
        Game {
            render_info: GameRenderInfo::default(),
            player: player,
            input: input,
            input_info: InputInfo::new(),
        }
    }
    pub fn get_render_info(&self) -> GameRenderInfo {
        self.render_info.clone()
    }
    pub fn tick(&mut self, frame_sec: f64) {
        // self.game_input.tick();
        self.input.down_trigger();
        self.input.procedure_events();
        self.input_info = self.input.owned_input();
        // TODO: Fieldを使って管理する
        self.player.accsel = (if self.input_info.is_pressed(Key::Up) {
            1.0
        } else {
            0.0
        }) + (if self.input_info.is_pressed(Key::Down) {
            -1.0
        } else {
            0.0
        });
        self.player.steer = (if self.input_info.is_pressed(Key::Right) {
            1.0
        } else {
            0.0
        }) + (if self.input_info.is_pressed(Key::Left) {
            -1.0
        } else {
            0.0
        });
        self.player.tick();
        self.render_info.player = self.player.entity.clone();
        self.render_info.camera.x =
            (self.render_info.player.x - 50.0 * self.render_info.player.angle.cos()) * 0.1
                + (self.render_info.camera.x) * 0.9;
        self.render_info.camera.y =
            (self.render_info.player.y - 50.0 * self.render_info.player.angle.sin()) * 0.1
                + (self.render_info.camera.y) * 0.9;
        self.render_info.frame_sec = frame_sec;
    }
}

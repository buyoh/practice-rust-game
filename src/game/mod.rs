#[derive(Clone)]
pub struct Entity {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

struct RigidBody {
    pub vx: f64,
    pub vy: f64,
    pub vangle: f64,
    // pub frictionv: f64,
    // pub frictionh: f64,
    pub friction: f64,
    pub frictiona: f64,
}

impl RigidBody {
    pub fn new() -> RigidBody {
        RigidBody {
            vx: 0.0,
            vy: 0.0,
            vangle: 0.0,
            // frictionv: 0.005,
            // frictionh: 0.02,
            friction: 0.005,
            frictiona: 0.06,
        }
    }
    pub fn tick(&mut self, entity: &mut Entity) {
        entity.x += self.vx;
        entity.y += self.vy;
        entity.angle += self.vangle;
        self.vx -= self.friction * self.vx;
        self.vy -= self.friction * self.vy;
        self.vangle -= self.frictiona * self.vangle;
    }
}

struct Machine {
    pub entity: Entity,
    rigid_body: RigidBody,
    pub accsel: f64,
    pub steer: f64,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            entity: Entity {
                x: 0.0,
                y: 0.0,
                angle: 0.0,
            },
            rigid_body: RigidBody::new(),
            accsel: 0.0,
            steer: 0.0,
        }
    }

    pub fn validate_and_fix(&mut self) {
        self.accsel = self.accsel.min(1.0).max(-1.0);
        self.steer = self.steer.min(1.0).max(-1.0);
    }

    pub fn tick(&mut self) {
        self.validate_and_fix();
        self.rigid_body.vx += self.entity.angle.cos() * self.accsel * 0.1;
        self.rigid_body.vy += self.entity.angle.sin() * self.accsel * 0.1;
        self.rigid_body.vangle += self.steer * 0.01;
        self.rigid_body.tick(&mut self.entity);
    }
}

pub struct Game {
    player: Machine,
    key_up: bool,
    key_down: bool,
    key_left: bool,
    key_right: bool,
}

impl Game {
    pub fn new() -> Game {
        let mut player = Machine::new();
        player.entity.x = 200.0;
        player.entity.y = 200.0;
        Game {
            player: player,
            key_up: false,
            key_down: false,
            key_left: false,
            key_right: false,
        }
    }
    pub fn player(&self) -> &Entity {
        // TODO: remove warning いい方法があるらしい
        &self.player.entity
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

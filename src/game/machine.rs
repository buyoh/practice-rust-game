use super::basic::*;

pub(crate) struct Machine {
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
                angle: std::f64::consts::PI / 2.0,
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

#[derive(Clone)]
pub struct Entity {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

pub struct RigidBody {
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

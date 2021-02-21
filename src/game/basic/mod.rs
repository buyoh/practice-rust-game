use std::ops::DerefMut;

#[derive(Clone)]
pub struct Entity {
    pub x: f64,
    pub y: f64,
    // TODO: z-axis
    pub angle: f64,
}
#[derive(Clone)]
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

pub trait FieldObject {
    fn tick(&mut self, field: &Field);
    fn entity(&self) -> Entity;
    fn rigid_body(&self) -> Option<RigidBody>;
    fn is_camera(&self) -> bool;
}

// ゲームワールド上に存在するものを取りまとめる構造体
pub struct Field(Vec<Box<dyn FieldObject>>);

impl Field {
    pub fn new() -> Field {
        Field(vec![])
    }
    pub fn add(&mut self, o: Box<dyn FieldObject>) {
        self.0.push(o);
    }
    pub fn tick(&mut self) {
        // TODO: 今は以下のようにしているが、更新前と更新後のFieldObjectが混在するのでbad
        // 2つずつ保持するようにする
        for o in self.0.iter_mut() {
            o.deref_mut().tick(self);
        }
    }
}

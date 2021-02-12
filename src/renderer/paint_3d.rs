use nalgebra::{Perspective3, Rotation3, Vector3};

use super::paint_2d;

pub struct Paint3D<'a> {
    d2: paint_2d::Paint2D<'a>,
    cam_pos: Vector3<f64>,
    cam_rot: Rotation3<f64>,
    pars: Perspective3<f64>,
}

impl Paint3D<'_> {
    pub fn new<'a>(context: &'a cairo::Context, width: f64, height: f64) -> Paint3D<'a> {
        Paint3D {
            d2: paint_2d::Paint2D::new(context, width, height),
            cam_pos: Vector3::<f64>::new(0.0, -5.0, 0.0),
            cam_rot: Rotation3::from_euler_angles(-0.1, 0.0, 0.0),
            pars: Perspective3::new(width / height, 3.14 / 4.0, 1.0, 10000.0),
        }
    }

    fn tr_vec(&self, vec: Vector3<f64>) -> Vector3<f64> {
        self.pars
            .project_vector(&(self.cam_rot * (vec + self.cam_pos)))
    }

    fn tr(&self, x: f64, y: f64, z: f64) -> Vector3<f64> {
        self.tr_vec(Vector3::<f64>::new(x, y, z))
    }

    fn set_camera_position(&mut self, x: f64, y: f64, z: f64) {
        self.cam_pos = Vector3::<f64>::new(x, y, z);
    }

    fn set_camera_rotation(&mut self, roll: f64, pitch: f64, yaw: f64) {
        self.cam_rot = Rotation3::from_euler_angles(roll, pitch, yaw);
    }

    pub fn stroke(&self) {
        self.d2.stroke();
    }

    pub fn move_to(&self, x: f64, y: f64, z: f64) {
        let v = self.tr(x, y, z);
        self.d2.move_to(v.x, v.y);
    }

    pub fn line_to(&self, x: f64, y: f64, z: f64) {
        let v = self.tr(x, y, z);
        self.d2.line_to(v.x, v.y);
    }
}

use nalgebra::{Perspective3, Rotation3, Vector3};

use super::paint_2d;

pub struct Paint3D<'a> {
    d2: paint_2d::Paint2D<'a>,
    cam_pos: Vector3<f64>,
    cam_rot: Rotation3<f64>,
    pars: Perspective3<f64>,
    pointer: (f64, f64, f64),
}

impl Paint3D<'_> {
    pub fn new<'a>(context: &'a cairo::Context, width: f64, height: f64) -> Paint3D<'a> {
        Paint3D {
            d2: paint_2d::Paint2D::new(context, width, height),
            cam_pos: Vector3::<f64>::new(0.0, -5.0, 0.0),
            cam_rot: Rotation3::from_euler_angles(-0.1, 0.0, 0.0),
            pars: Perspective3::new(width / height, 3.14 / 4.0, 0.0, 10000.0),
            pointer: (0.0, 0.0, 0.0),
        }
    }

    fn is_renderable_vec(&self, vec: Vector3<f64>) -> bool {
        let v = self.cam_rot * (vec + self.cam_pos);
        v.z >= 0.0
    }

    fn is_renderable(&self, x: f64, y: f64, z: f64) -> bool {
        self.is_renderable_vec(Vector3::<f64>::new(x, y, z))
    }

    fn tr_vec(&self, vec: Vector3<f64>) -> Option<Vector3<f64>> {
        let v = self.cam_rot * (vec + self.cam_pos);
        if v.z < 0.0 {
            return None;
        }
        Some(self.pars.project_vector(&v))
    }

    fn tr(&self, x: f64, y: f64, z: f64) -> Option<Vector3<f64>> {
        self.tr_vec(Vector3::<f64>::new(x, y, z))
    }

    pub fn set_camera_position(&mut self, x: f64, y: f64, z: f64) {
        self.cam_pos = Vector3::<f64>::new(x, y, z);
    }

    pub fn set_camera_rotation(&mut self, roll: f64, pitch: f64, yaw: f64) {
        self.cam_rot = Rotation3::from_euler_angles(roll, pitch, yaw);
    }

    pub fn set_camera_rotation_face_towards(&mut self, tx: f64, ty: f64, tz: f64) {
        self.cam_rot =
            Rotation3::face_towards(&(Vector3::new(tx, ty, tz) - self.cam_pos), &Vector3::y());
    }

    pub fn stroke(&self) {
        self.d2.stroke();
    }

    fn clamp_outside_point(
        &self,
        x: f64,
        y: f64,
        z: f64,
        xout: f64,
        yout: f64,
        zout: f64,
    ) -> (f64, f64, f64) {
        let mut xi = x;
        let mut yi = y;
        let mut zi = z;
        let mut xo = xout;
        let mut yo = yout;
        let mut zo = zout;
        for _ in 0..50 {
            let mut xm = (xo + xi) / 2.0;
            let mut ym = (yo + yi) / 2.0;
            let mut zm = (zo + zi) / 2.0;
            if self.is_renderable(x, y, z) {
                xi = xm;
                yi = ym;
                zi = zm;
            } else {
                xo = xm;
                yo = ym;
                zo = zm;
            }
        }
        (xi, yi, zi)
    }

    pub fn line(&self, x: f64, y: f64, z: f64, x2: f64, y2: f64, z2: f64) {
        let v = self.tr(x, y, z);
        let v2 = self.tr(x2, y2, z2);
        if let Some(u) = v {
            if let Some(u2) = v2 {
                self.d2.linec(u.x, u.y, u2.x, u2.y);
            } else {
                let u2 = self.clamp_outside_point(x, y, z, x2, y2, z2);
                self.d2.linec(u.x, u.y, u2.0, u2.1);
            }
        } else {
            if let Some(u2) = v2 {
                let u = self.clamp_outside_point(x2, y2, z2, x, y, z);
                self.d2.linec(u.0, u.1, u2.x, u2.y);
            } else {
                // nop
            }
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64, z: f64) {
        // let v = self.tr(x, y, z);
        // 大きく枠を超えることがあるので、うまくクランプしておく
        // move_to 辺りのステート操作は自前でやる
        self.pointer = (x, y, z);
    }

    pub fn line_to(&mut self, x: f64, y: f64, z: f64) {
        self.line(self.pointer.0, self.pointer.1, self.pointer.2, x, y, z);
        self.pointer = (x, y, z);
    }
}

use nalgebra::{Perspective3, Point3, Rotation3, Vector3};

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
            cam_rot: Rotation3::from_euler_angles(0.0, 0.0, 0.0),
            pars: Perspective3::new(width / height, 3.14 / 4.0, 0.01, 10000.0),
            pointer: (0.0, 0.0, 0.0),
        }
    }

    fn tr_vec(&self, vec: Vector3<f64>) -> Option<Vector3<f64>> {
        let v = self.cam_rot.transform_vector(&(vec - self.cam_pos));
        if v.z < self.pars.znear() || self.pars.zfar() < v.z {
            return None;
        }
        let v2 = self.pars.project_vector(&v);
        if -1.0 <= v2.x && v2.x <= 1.0 && -1.0 <= v2.y && v2.y <= 1.0 {
            Some(self.pars.project_vector(&v))
        } else {
            None
        }
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
            Rotation3::face_towards(&(Vector3::new(tx, ty, tz) - self.cam_pos), &Vector3::y())
                .inverse();
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
            let xm = (xo + xi) / 2.0;
            let ym = (yo + yi) / 2.0;
            let zm = (zo + zi) / 2.0;
            if self.tr(xm, ym, zm).is_some() {
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

    fn try_render_invisible_line(&self, x: f64, y: f64, z: f64, x2: f64, y2: f64, z2: f64) {
        let mut v1 = self
            .cam_rot
            .transform_vector(&(Vector3::new(x, y, z) - self.cam_pos));
        let mut v2 = self
            .cam_rot
            .transform_vector(&(Vector3::new(x2, y2, z2) - self.cam_pos));
        if v1.x > v2.x {
            // (v2, v1) = (v1, v2);
            let vt = v2;
            v2 = v1;
            v1 = vt;
        }
        if v1.z < self.pars.znear() && v2.z < self.pars.znear() {
            return; // never
        }
        if (v2.x - v1.x).abs() <= f64::EPSILON {
            return; // avoid zero-division
        }
        let fx = (-v1.x) / (v2.x - v1.x);
        if fx < 0.0 || 1.0 < fx {
            return; // we consider only the line acrossing with x=0
        }
        let xm = x * fx + x2 * (1.0 - fx);
        let ym = y * fx + y2 * (1.0 - fx);
        let zm = z * fx + z2 * (1.0 - fx);
        let vm = self.tr(xm, ym, zm);
        if vm.is_some() {
            // It doesn't loop infinity because vm is renderable
            self.line(x, y, z, xm, ym, zm);
            self.line(xm, ym, zm, x2, y2, z2);
        }
    }

    pub fn line(&self, x: f64, y: f64, z: f64, x2: f64, y2: f64, z2: f64) {
        let v = self.tr(x, y, z);
        let v2 = self.tr(x2, y2, z2);
        if let Some(u) = v {
            if let Some(u2) = v2 {
                self.d2.line(u.x, u.y, u2.x, u2.y);
            } else {
                let v2c = self.clamp_outside_point(x, y, z, x2, y2, z2);
                let u2 = self.tr(v2c.0, v2c.1, v2c.2).unwrap();
                self.d2.line(u.x, u.y, u2.x, u2.y);
            }
        } else {
            if let Some(u2) = v2 {
                let vc = self.clamp_outside_point(x2, y2, z2, x, y, z);
                let u = self.tr(vc.0, vc.1, vc.2).unwrap();
                self.d2.line(u.x, u.y, u2.x, u2.y);
            } else {
                self.try_render_invisible_line(x, y, z, x2, y2, z2);
            }
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64, z: f64) {
        self.pointer = (x, y, z);
    }

    pub fn line_to(&mut self, x: f64, y: f64, z: f64) {
        self.line(self.pointer.0, self.pointer.1, self.pointer.2, x, y, z);
        self.pointer = (x, y, z);
    }
}

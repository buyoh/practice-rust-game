use glib::translate::ToGlibPtr;
use nalgebra::clamp;

pub struct Paint2D<'a> {
    context: &'a cairo::Context,
    width: f64,
    height: f64,
    center_x: f64,
    center_y: f64,
    zoom_x: f64,
    zoom_y: f64,
    pointer: (f64, f64),
}

impl Paint2D<'_> {
    pub fn new<'a>(context: &'a cairo::Context, width: f64, height: f64) -> Paint2D<'a> {
        Paint2D {
            context: context,
            width: width,
            height: height,
            center_x: width / 2.0,
            center_y: height / 2.0,
            zoom_x: width / 2.0,
            zoom_y: -height / 2.0,
            pointer: (0.0, 0.0),
        }
    }

    pub fn new_as_ui<'a>(context: &'a cairo::Context, width: f64, height: f64) -> Paint2D<'a> {
        Paint2D {
            context: context,
            width: width,
            height: height,
            center_x: 0.0,
            center_y: 0.0,
            zoom_x: 1.0,
            zoom_y: 1.0,
            pointer: (0.0, 0.0),
        }
    }

    /// Set the center position ([-1, 1], [-1, 1]). (-1, -1) is top-left. (1, 1) is bottom-right.
    /// These arguments are not affected by zoom values.
    /// When it's specified (0, 0), positions are shifted x+=width*(center_x+1)/2, and y axis are shifted as well.
    /// TODO: This definition may be bad because I won't use this method.
    // pub fn set_center(&mut self, center_ratio_x: f64, center_ratio_y: f64) {
    //     self.center_x = self.width * (center_ratio_x + 1.0) / 2.0;
    //     self.center_y = self.height * (center_ratio_y + 1.0) / 2.0;
    // }

    /// Set the zoom values.
    // pub fn set_zoom(&mut self, zoom_ratio_x: f64, zoom_ratio_y: f64) {
    //     self.zoom_x = self.width / 2.0 / zoom_ratio_x;
    //     self.zoom_y = self.height / 2.0 / zoom_ratio_y;
    // }

    pub fn stroke(&self) {
        self.context.stroke();
    }
    pub fn rectangle(&self, x: f64, y: f64, width: f64, height: f64) {
        self.context.rectangle(
            x * self.zoom_x + self.center_x,
            y * self.zoom_y + self.center_y,
            width,
            height,
        );
    }
    pub fn move_to(&self, x: f64, y: f64) {
        self.context.move_to(
            x * self.zoom_x + self.center_x,
            y * self.zoom_y + self.center_y,
        );
    }
    pub fn line_to(&self, x: f64, y: f64) {
        self.context.line_to(
            x * self.zoom_x + self.center_x,
            y * self.zoom_y + self.center_y,
        );
    }

    pub fn linec(&self, x: f64, y: f64, x2: f64, y2: f64) {
        // TODO: use width
        if let Some(((cx, cy), (cx2, cy2))) = clamp_line(x, y, x2, y2) {
            self.move_to(cx, cy);
            self.line_to(cx2, cy2);
        }
    }
    pub fn movec_to(&mut self, x: f64, y: f64) {
        self.pointer = (x, y);
    }
    pub fn linec_to(&mut self, x: f64, y: f64) {
        self.linec(x, y, self.pointer.0, self.pointer.1);
        self.pointer = (x, y);
    }
}

/// x = x1: return 0.0,
/// x = x2: return 1.0,
fn linear(x1: f64, x2: f64, x: f64) -> f64 {
    (x - x1) / (x2 - x1)
}

fn inside(x: f64, y: f64) -> bool {
    -1.0 <= x && x <= 1.0 && -1.0 <= y && y <= 1.0
}

// (x, y) (x2, y2) are outside
fn clamp_outside_p2(x: f64, y: f64, x2: f64, y2: f64) -> Option<((f64, f64), (f64, f64))> {
    let left_y = y2 * linear(x, x2, -1.0) + y * linear(x2, x, -1.0);
    let top_x = x2 * linear(y, y2, -1.0) + x * linear(y2, y, -1.0);
    let right_y = y2 * linear(x, x2, -1.0) + y * linear(x2, x, -1.0);
    let bottom_x = x2 * linear(y, y2, 1.0) + x * linear(y2, y, 1.0);
    let mut p1 = Option::<(f64, f64)>::None;
    let mut p2 = Option::<(f64, f64)>::None;
    if -1.0 < left_y && left_y < 1.0 {
        // we cant consider about corner cases such as (-1, -1) (1, 1)
        let p = Option::Some((-1.0, left_y));
        if p1.is_none() {
            p1 = p;
        } else {
            p2 = p;
        }
    }
    if -1.0 < top_x && top_x < 1.0 {
        let p = Option::Some((top_x, -1.0));
        if p1.is_none() {
            p1 = p;
        } else {
            p2 = p;
        }
    }
    if -1.0 < right_y && right_y < 1.0 {
        let p = Option::Some((1.0, right_y));
        if p1.is_none() {
            p1 = p;
        } else {
            p2 = p;
        }
    }
    if -1.0 < bottom_x && bottom_x < 1.0 {
        let p = Option::Some((bottom_x, 1.0));
        if p1.is_none() {
            p1 = p;
        } else {
            p2 = p;
        }
    }
    if p1.is_some() && p2.is_some() {
        Some((p1.unwrap(), p2.unwrap()))
    } else {
        None
    }
}

fn clamp_outside_p1(x: f64, y: f64, x2o: f64, y2o: f64) -> ((f64, f64), (f64, f64)) {
    let mut xi = x;
    let mut yi = y;
    let mut xo = x2o;
    let mut yo = y2o;
    for i in 0..20 {
        let mut xt = (xi + xo) / 2.0;
        let mut yt = (yi + yo) / 2.0;
        if inside(xt, yt) {
            xi = xt;
            yi = yt;
        } else {
            xo = xt;
            yo = yt;
        }
    }
    ((x, y), (xo, yo))
}

fn clamp_line(x: f64, y: f64, x2: f64, y2: f64) -> Option<((f64, f64), (f64, f64))> {
    let i1 = inside(x, y);
    let i2 = inside(x2, y2);
    if !i1 && !i2 {
        return clamp_outside_p2(x, y, x2, y2);
    } else if i1 && !i2 {
        return Some(clamp_outside_p1(x, y, x2, y2));
    } else if !i1 && i2 {
        return Some(clamp_outside_p1(x2, y2, x, y));
    } else {
        return Some(((x, y), (x2, y2)));
    }
}

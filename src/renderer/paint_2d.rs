pub struct Paint2D<'a> {
    context: &'a cairo::Context,
    width: f64,
    height: f64,
    center_x: f64,
    center_y: f64,
    zoom_x: f64,
    zoom_y: f64,
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
            zoom_y: height / 2.0,
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

    pub fn line(&self, x: f64, y: f64, x2: f64, y2: f64) {
        self.move_to(x, y);
        self.line_to(x2, y2);
    }
}

use crate::game::{Game, GameDisplayInfo};

struct Paint {
    context: cairo::Context,
    border_left: f64,
    border_top: f64,
    border_right: f64,
    border_bottom: f64,
    cv_x_x: f64,
    cv_x_y: f64,
    cv_x_a: f64,
    cv_y_x: f64,
    cv_y_y: f64,
    cv_y_a: f64,
}

impl Paint {
    pub fn new_as_ui(context: cairo::Context) -> Paint {
        Paint {
            context: context,
            border_left: 0.0,
            border_top: 0.0,
            border_right: 400.0,
            border_bottom: 400.0,
            cv_x_x: 1.0,
            cv_x_y: 0.0,
            cv_x_a: 0.0,
            cv_y_x: 1.0,
            cv_y_y: 0.0,
            cv_y_a: 0.0,
        }
    }
    pub fn new_as_game(context: cairo::Context, game: &GameDisplayInfo) -> Paint {
        let (cos, sin) = game.camera.angle.sin_cos();

        Paint {
            context: context,
            border_left: 0.0,
            border_top: 0.0,
            border_right: 400.0,
            border_bottom: 400.0,
            cv_x_x: cos,
            cv_x_y: sin,
            cv_x_a: game.camera.x,
            cv_y_x: -sin,
            cv_y_y: cos,
            cv_y_a: game.camera.y,
        }
    }

    pub fn stroke(&self) {}
    pub fn rectangle(&self) {}
    pub fn move_to(&self) {}
    pub fn line_to(&self) {}
}

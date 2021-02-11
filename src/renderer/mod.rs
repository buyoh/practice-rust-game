use std::sync::mpsc::Receiver;

use nalgebra::{Perspective3, Rotation3, Vector3};

use crate::game::GameDisplayInfo;

use std::cell::*;
use std::rc::*;

fn trans(vec: Vector3<f64>) -> Vector3<f64> {
    // Arguments order: aspect, fovy, znear, zfar.
    let p = &Vector3::<f64>::new(0.0, -5.0, 0.0) + vec;
    let proj = Perspective3::new(1.0 / 1.0, 3.14 / 4.0, 1.0, 10000.0);
    proj.project_vector(&(Rotation3::from_euler_angles(-0.1, 0.0, 0.0) * p))
}

pub struct Renderer {
    width: i32,
    height: i32,
    game_display_rx: Receiver<GameDisplayInfo>,
}

pub(crate) struct RendererHolder {
    renderer: Renderer,
    game_display: Rc<RefCell<GameDisplayInfo>>,
}

impl Renderer {
    pub fn new(width: i32, height: i32, game_display_rx: Receiver<GameDisplayInfo>) -> Renderer {
        Renderer {
            width: width,
            height: height,
            game_display_rx: game_display_rx,
        }
    }
}

impl RendererHolder {
    pub fn new(renderer: Renderer) -> RendererHolder {
        RendererHolder {
            renderer: renderer,
            game_display: Rc::new(RefCell::new(GameDisplayInfo::default())),
        }
    }

    pub fn paint_game(&mut self, context: &cairo::Context, cnt: i32) {
        // TODO: replace cnt to time
        // update display here
        for new_game_display in self.renderer.game_display_rx.try_iter() {
            self.game_display.replace(new_game_display);
        }
        let game = self.game_display.borrow();
        context.set_source_rgb(1.0, 1.0, cnt as f64 / 100.0);
        context.paint();
        self.paint_player(context, &game);
        self.paint_course(context, &game);
    }

    fn paint_player(&self, context: &cairo::Context, game: &GameDisplayInfo) {
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.rectangle(game.player.x, game.player.y, 20.0, 20.0);

        let mut p1 = Vector3::<f64>::new(game.player.x - 10.0, 0.1, game.player.y);
        let mut p2 = Vector3::<f64>::new(game.player.x, 0.1, game.player.y - 10.0);
        let mut p3 = Vector3::<f64>::new(game.player.x + 10.0, 0.1, game.player.y);
        let mut p4 = Vector3::<f64>::new(game.player.x, 0.1, game.player.y + 10.0);
        p1 = trans(p1);
        p2 = trans(p2);
        p3 = trans(p3);
        p4 = trans(p4);
        let cx = self.renderer.width as f64 / 2.0;
        let cy = self.renderer.height as f64 / 2.0;
        context.move_to(cx + p1.x * cx, cy + p1.y * cy);
        context.line_to(cx + p2.x * cx, cy + p2.y * cy);
        context.line_to(cx + p3.x * cx, cy + p3.y * cy);
        context.line_to(cx + p4.x * cx, cy + p4.y * cy);
        context.line_to(cx + p1.x * cx, cy + p1.y * cy);
        context.stroke();
    }
    fn paint_course(&self, context: &cairo::Context, game: &GameDisplayInfo) {
        context.set_source_rgb(0.0, 0.0, 0.0);
        let last = game.course.points.last().unwrap().clone();
        context.move_to(last.x, last.y);
        for p in &game.course.points {
            context.line_to(p.x, p.y);
        }
        context.stroke();
    }
}

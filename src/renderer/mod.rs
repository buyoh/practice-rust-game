use std::sync::mpsc::Receiver;

use crate::game::GameDisplayInfo;

use std::cell::*;
use std::rc::*;

pub struct Renderer {
    game_display_rx: Receiver<GameDisplayInfo>,
}

pub(crate) struct RendererHolder {
    renderer: Renderer,
    game_display: Rc<RefCell<GameDisplayInfo>>,
}

impl Renderer {
    pub fn new(game_display_rx: Receiver<GameDisplayInfo>) -> Renderer {
        Renderer {
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

    fn paint_player(& self, context: &cairo::Context, game: &GameDisplayInfo) {
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.rectangle(game.player.x, game.player.y, 20.0, 20.0);
        context.stroke();

    }
    fn paint_course(& self, context: &cairo::Context, game: &GameDisplayInfo) {
        context.set_source_rgb(0.0, 0.0, 0.0);
        let last = game.course.points.last().unwrap().clone();
        context.move_to(last.x, last.y);
        for p in &game.course.points {
            context.line_to(p.x, p.y);
        }
        context.stroke();
    }
}

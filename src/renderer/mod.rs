use std::sync::mpsc::Receiver;

use crate::game::GameDisplayInfo;

use std::cell::*;
use std::rc::*;

mod game;
mod paint_2d;
mod paint_3d;

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

    pub fn paint_game(&mut self, context: &cairo::Context) {
        // TODO: replace cnt to time
        // Take display data from queue
        for new_game_display in self.renderer.game_display_rx.try_iter() {
            self.game_display.replace(new_game_display);
        }
        let game = self.game_display.borrow();
        game::paint_entry(context, &self.renderer, &game);
    }
}

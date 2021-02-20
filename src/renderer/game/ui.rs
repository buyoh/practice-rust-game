use crate::game::GameDisplayInfo;

use super::{paint_2d, paint_3d, Renderer};

pub fn paint_ui(context: &cairo::Context, renderer: &Renderer, game: &GameDisplayInfo) {
    let px = paint_2d::Paint2D::new_as_ui(context, renderer.width as f64, renderer.height as f64);
    // car position
    px.rectangle(game.player.x, game.player.y, 20.0, 20.0);

    // course
    context.set_source_rgb(0.0, 0.0, 0.0);
    let last = game.course.points.last().unwrap().clone();
    px.move_to(last.x, last.y);
    for p in &game.course.points {
        px.line_to(p.x, p.y);
    }
    px.stroke();
}

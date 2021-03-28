use crate::game::GameRenderInfo;

use super::{paint_2d, paint_3d, Renderer};
mod component3d;

fn paint_game(context: &cairo::Context, renderer: &Renderer, game: &GameRenderInfo) {
    context.set_source_rgb(0.0, 0.0, 0.0);

    let mut px = paint_3d::Paint3D::new(context, renderer.width as f64, renderer.height as f64);
    // let r = game.frame_sec / 1.0;
    px.set_camera_position(game.camera.x, 20.0, game.camera.y);
    px.set_camera_rotation_face_towards(game.player.x, 0.1, game.player.y);

    context.set_source_rgb(0.0, 0.0, 0.0);
    component3d::paint_machine(&mut px, &game.player);
    component3d::paint_course(&mut px, &game.course);

    for z in -5..6 {
        for x in -5..6 {
            px.move_to(x as f64 * 10.0, 0.0, z as f64 * 10.0);
            px.line_to(x as f64 * 10.0, 0.0, z as f64 * 10.0 + 10.0);
            px.line_to(x as f64 * 10.0 + 10.0, 0.0, z as f64 * 10.0 + 10.0);
        }
    }
    px.stroke();
}

fn paint_ui(context: &cairo::Context, renderer: &Renderer, game: &GameRenderInfo) {
    // TODO
    // let px = paint_2d::Paint2D::new_as_ui(context, renderer.width as f64, renderer.height as f64);
    // // car position
    // px.rectangle(game.player.x, game.player.y, 20.0, 20.0);

    // // course
    // context.set_source_rgb(0.0, 0.0, 0.0);
    // let last = game.course.points.last().unwrap().clone();
    // px.move_to(last.x, last.y);
    // for p in &game.course.points {
    //     px.line_to(p.x, p.y);
    // }
    // px.stroke();
}

pub fn paint_entry(context: &cairo::Context, renderer: &Renderer, game: &GameRenderInfo) {
    context.set_source_rgb(0.9, 0.9, 0.9);
    context.paint();
    paint_game(context, renderer, game);
    paint_ui(context, renderer, game);
}

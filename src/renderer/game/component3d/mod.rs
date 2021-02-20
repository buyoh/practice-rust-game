use crate::game::{basic, course};

use super::paint_3d;

pub fn paint_course(px: &mut paint_3d::Paint3D, course: &course::Course) {
    let last = course.points.last().unwrap().clone();
    px.move_to(last.x, 0.0, last.y);
    for p in &course.points {
        px.line_to(p.x, 0.0, p.y);
    }
    px.stroke();
}

pub fn paint_machine(px: &mut paint_3d::Paint3D, entity: &basic::Entity) {
    {
        let x = entity.x;
        let y = entity.y;
        let s = entity.angle.sin();
        let c = entity.angle.cos();
        px.move_to(x + 10.0 * c, 0.1, y + 10.0 * s);
        px.line_to(x - 5.0 * s, 0.1, y + 5.0 * c);
        px.line_to(x - 5.0 * c, 0.1, y - 5.0 * s);
        px.line_to(x + 5.0 * s, 0.1, y - 5.0 * c);
        px.line_to(x + 10.0 * c, 0.1, y + 10.0 * s);
    }

    px.stroke();
}

use comn::{
    controls::{Heading, Looking},
    prelude::*,
    specs,
};
use log::*;
use specs::prelude::*;

pub struct MoveHeadings;
impl<'a> System<'a> for MoveHeadings {
    type SystemData = (
        WriteStorage<'a, SimplePosition<f32>>,
        ReadStorage<'a, Heading>,
        ReadStorage<'a, Looking>,
    );

    fn run(&mut self, (mut isos, heads, _): Self::SystemData) {
        for (mut iso, heading) in (&mut isos, &heads).join() {
            if heading.dir.magnitude() > 0.0 {
                let m = heading.dir.into_inner();
                iso.0.translation.vector += na::Vector3::new(m.x, 0.0, m.z).normalize() * 0.1;
            }
        }
    }
}

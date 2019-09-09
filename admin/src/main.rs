// util
use log::*;
use std::f32::consts::PI;
// ecs
use specs::WorldExt;
// our code/reexports
use comn::{
    na,
    specs::{self, prelude::*},
    specs_physics::SimplePosition,
};

mod net;

fn main() {
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Debug)
        .init();

    let mut world = specs::World::new();
    #[rustfmt::skip]
    let dispatcher_builder = DispatcherBuilder::new()
        .with(net::HandleClientPackets,     "client packets",   &[])
        .with(net::SendWorldToNewPlayers,   "send world",       &[]);
    let mut dispatcher = dispatcher_builder.build();

    dispatcher.setup(&mut world);

    // add some dummy ents
    world
        .create_entity()
        .with(comn::art::Appearance {
            filepath: String::from("hi"),
        })
        .with(SimplePosition(na::Isometry3::rotation(
            na::Vector3::repeat(PI * 0.25),
        )))
        .build();

    world
        .create_entity()
        .with(comn::art::Appearance {
            filepath: String::from("hi"),
        })
        .with(SimplePosition(na::Isometry3::<f32>::translation(
            0.0, 0.1, 0.1,
        )))
        .build();

    info!("starting game loop!");

    let mut fixedstep = fixedstep::FixedStep::start(60.0); // 60.0Hz

    loop {
        while fixedstep.update() {
            dispatcher.dispatch(&mut world);
            world.maintain();
        }
    }
}

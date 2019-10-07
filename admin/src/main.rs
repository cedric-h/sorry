// util
use log::*;
use std::f32::consts::PI;
// ecs
use specs::WorldExt;
// our code/reexports
use comn::{
    prelude::*,
    specs::{self, prelude::*},
};

mod movement;
mod net;

fn main() {
    {
        use log::LevelFilter::*;

        #[rustfmt::skip]
        pretty_env_logger::formatted_builder()
            .filter(Some("specs_physics"),  Error)
            .filter(None,                   Debug)
            .init();
    }

    let mut world = specs::World::new();
    #[rustfmt::skip]
    let mut dispatcher_builder = DispatcherBuilder::new()
        .with(movement::MoveHeadings,       "heading",          &[])
        .with(net::SendWorldToNewPlayers,   "send world",       &[])
        .with(net::HandleClientPackets,     "client packets",   &["send world"])
        .with(net::SendNewPositions,        "send pos",         &[]);
    register_physics_systems::<f32, SimplePosition<f32>>(&mut dispatcher_builder);
    let mut dispatcher = dispatcher_builder.build();

    dispatcher.setup(&mut world);

    world.insert(specs_physics::parameters::Gravity(
        na::Vector3::<f32>::y() * -9.82,
    ));

    // add some dummy ents
    {
        use comn::art::Appearance;
        use na::{Isometry3, Vector3};
        use specs_physics::SimplePosition as Pos;
        type Iso3 = Isometry3<f32>;
        type Vec3 = Vector3<f32>;

        for x in -1..1 {
            for y in -1..1 {
                for z in -1..1 {
                    info!("{}, {}, {}", x, y, z);
                    world
                        .create_entity()
                        .with(Appearance::new("hi"))
                        .with(Pos(Iso3::translation(
                            x as f32 * 0.1,
                            y as f32 * 0.1 + 10.0,
                            z as f32 * 0.1,
                        )))
                        .with(
                            PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
                                .gravity_enabled(true)
                                .mass(0.0)
                                .build(),
                        )
                        .with(
                            PhysicsColliderBuilder::from(Shape::Cuboid {
                                half_extents: na::Vector3::<f32>::repeat(0.1),
                            })
                            .density(0.18)
                            .build(),
                        )
                        .build();
                }
            }
        }
        world
            .create_entity()
            .with(Pos(Iso3::translation(0.0, -1.0, 0.0)))
            .with(PhysicsBodyBuilder::<f32>::from(BodyStatus::Static).build())
            .with(
                PhysicsColliderBuilder::from(Shape::Cuboid {
                    half_extents: na::Vector3::<f32>::new(10.0, 0.5, 10.0),
                })
                .build(),
            )
            .build();
    }

    info!("starting game loop!");

    let mut fixedstep = fixedstep::FixedStep::start(20.0); // 20.0Hz

    loop {
        while fixedstep.update() {
            dispatcher.dispatch(&mut world);
            world.maintain();
        }
    }
}

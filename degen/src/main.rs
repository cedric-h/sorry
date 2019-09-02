#![recursion_limit = "256"]
#[macro_use]
extern crate stdweb;
use stdweb::web::window;

pub mod prelude {
    pub use specs::{prelude::*, Component};
    pub use comn::rmps;
    // specs physics
    pub use specs_physics::{
        colliders::Shape, nalgebra as na, ncollide::query::Ray, nphysics::object::BodyStatus,
        register_physics_systems, Physics, PhysicsBody, PhysicsBodyBuilder, PhysicsCollider,
        PhysicsColliderBuilder, SimplePosition,
    };
}
use prelude::*;

mod renderer {
    use crate::prelude::*;
    use comn::art::Appearance;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct MeshBundle {
        ent: u32,
        appearance: Appearance,
        iso: SimplePosition<f32>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RenderData {
        ents: Vec<MeshBundle>,
    }
    js_serializable!(RenderData);

    pub struct Render;
    impl<'a> System<'a> for Render {
        type SystemData = (
            Entities<'a>,
            ReadStorage<'a, Appearance>,
            ReadStorage<'a, SimplePosition<f32>>,
        );

        fn run(&mut self, (ents, appearances, isos): Self::SystemData) {
            let render_data = RenderData {
                ents: (&*ents, &appearances, &isos)
                    .join()
                    .map(|(e, a, i)| MeshBundle {
                        ent: e.id(),
                        appearance: a.clone(),
                        iso: i.clone(),
                    })
                    .collect::<Vec<_>>(),
            };

            js! {
                render(@{render_data})
            }
        }
    }
}

mod net {
    use specs::prelude::*;
    use std::{collections::HashMap, sync::{Arc, Mutex}};
    use stdweb::{
        console,
        traits::*,
        web::{
            event::{SocketCloseEvent, SocketErrorEvent, SocketMessageEvent, SocketOpenEvent},
            IEventTarget, WebSocket,
        },
    };

    pub struct ServerConnection {
        ws: WebSocket,
        pub message_queue: Arc<Mutex<Vec<comn::NetMessage>>>,
    }

    impl Default for ServerConnection {
        fn default() -> Self {
            let ws = WebSocket::new("ws://127.0.0.1:3012")
                .unwrap_or_else(|e| panic!("couldn't reach server: {}", e));
            let message_queue = Arc::new(Mutex::new(Vec::new()));

            ws.add_event_listener(|_: SocketOpenEvent| {
                console!(log, "Connected to server!");
            });

            ws.add_event_listener(|e: SocketErrorEvent| {
                js! {
                    console.error("Errror connecting to %s", @{e}.target.url);
                };
            });

            ws.add_event_listener(|e: SocketCloseEvent| {
                console!(error, "Server Connection Closed: %s", e.reason());
            });

            ws.add_event_listener({
                let msgs = message_queue.clone();

                move |msg: SocketMessageEvent| {
                    console!(log, "ServerMessage");

                    let buf: Vec<u8> = msg
                        .data()
                        .into_array_buffer()
                        .expect("We got a message that isn't an array buf!")
                        .into();

                    let mut msgs = msgs.lock().expect("The Server Message Queue is locked!");
                    msgs.push(comn::rmps::from_read_ref(&buf).expect("couldn't read net message bytes"));
                }
            });

            Self { ws, message_queue }
        }
    }

    #[derive(Default)]
    pub struct HandleServerPackets {
        pub server_to_local_ids: HashMap<u32, u32>,
    }
    impl<'a> System<'a> for HandleServerPackets {
        type SystemData = (
            Entities<'a>,
            Read<'a, LazyUpdate>,
            Read<'a, ServerConnection>,
        );

        fn run(&mut self, (ents, lu, sc): Self::SystemData) {
            if let Ok(mut msgs) = sc.message_queue.try_lock() {
                for msg in msgs.drain(0..) {
                    use comn::NetMessage::*;

                    match msg {
                        NewEnt(server) => {
                            let local: u32 = ents.create().id();
                            self.server_to_local_ids.insert(server, local);
                        },
                        InsertComp(id, net_comp) => {
                            let ent = self
                                .server_to_local_ids
                                .get(&id)
                                .map(|ent| ents.entity(*ent))
                                .filter(|ent| {
                                    if !ents.is_alive(*ent) {
                                        console!(log, "filtering out dead ent");
                                    }
                                    ents.is_alive(*ent)
                                });

                            if let Some(ent) = ent {
                                net_comp.insert(&lu, ent);
                            } else {
                                console!(error, "Can't insert component for dead entity");
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    use std::f32::consts::PI;

    stdweb::initialize();
    // https://github.com/rustwasm/console_error_panic_hook/blob/master/src/lib.rs ?

    // instantiate an ECS world to hold all of the systems, resources, and components.
    let mut world = World::new();

    // add systems and instantiate and order the other systems.
    #[rustfmt::skip]
    let dispatcher_builder = DispatcherBuilder::new()
        .with(renderer::Render,                     "render",   &[])
        .with(net::HandleServerPackets::default(),  "packets",  &[]);
    let mut dispatcher = dispatcher_builder.build();

    // go through all of the systems and register components and resources accordingly
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

    fn game_loop(mut dispatcher: specs::Dispatcher<'static, 'static>, mut world: specs::World) {
        // run all of the ECS systems
        dispatcher.dispatch(&mut world);
        world.maintain();

        // tell browser to repeat me the next time the monitor is going to refresh
        window().request_animation_frame(|_| game_loop(dispatcher, world));
    }

    game_loop(dispatcher, world);

    stdweb::event_loop();
}

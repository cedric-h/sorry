#![recursion_limit = "256"]
#[macro_use]
extern crate stdweb;
use stdweb::web::window;

pub mod prelude {
    pub use comn::prelude::*;
    pub use comn::rmps;
    pub use specs::{prelude::*, Component};
}
use prelude::*;

mod renderer {
    use super::controls;
    use crate::prelude::*;
    use comn::art::Appearance;
    use comn::controls::Camera as CameraGuide;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct MeshBundle {
        ent: u32,
        appearance: Appearance,
        iso: SimplePosition<f32>,
        camera_guide: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RenderData {
        ents: Vec<MeshBundle>,
        camera_direction: na::Vector3<f32>,
    }
    js_serializable!(RenderData);

    pub struct Render;
    impl<'a> System<'a> for Render {
        type SystemData = (
            Entities<'a>,
            ReadStorage<'a, Appearance>,
            ReadStorage<'a, SimplePosition<f32>>,
            ReadStorage<'a, CameraGuide>,
            Read<'a, controls::Camera>,
        );

        fn run(&mut self, (ents, appearances, isos, cam_guides, cam): Self::SystemData) {
            let render_data = RenderData {
                ents: (&*ents, &appearances, &isos, cam_guides.maybe())
                    .join()
                    .map(|(e, a, i, cg)| MeshBundle {
                        ent: e.id(),
                        appearance: a.clone(),
                        iso: i.clone(),
                        camera_guide: cg.is_some(),
                    })
                    .collect::<Vec<_>>(),
                camera_direction: cam.make_front(),
            };

            js! {
                render(@{render_data})
            }
        }
    }
}

mod net {
    use comn::{rmps, NetComponent, NetMessage};
    use specs::prelude::*;
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };
    use stdweb::{
        console,
        unstable::TryInto,
        web::{
            event::{SocketCloseEvent, SocketErrorEvent, SocketMessageEvent, SocketOpenEvent},
            ArrayBuffer, IEventTarget, WebSocket,
        },
        Value,
    };

    pub struct ServerConnection {
        ws: WebSocket,
        pub message_queue: Arc<Mutex<Vec<NetMessage>>>,
    }
    impl ServerConnection {
        #[inline]
        fn send(&self, msg: NetMessage) {
            self.ws
                .send_bytes(&rmps::encode::to_vec(&msg).expect("Couldn't encode NetMessage!"))
                .expect("Couldn't send NetMessage to server!");
        }

        /* I'm not sure why/when/how you'd ever even actually use this on the client.
         * The server should definitely be in control of when new things are made,
         * even if indirectly the Client ends up requesting that to happen.
         * For that reason, this is prevented from working on the serverside.
         * Instead, it's used internally to register a new player; if you send
         * this request through hacking or some other means, you'll just get
         * your player reset :grin:
        #[inline]
        pub fn new_ent(&self, ent: specs::Entity) {
            self.send(NetMessage::NewEnt(ent.id()));
        }*/

        #[inline]
        pub fn insert_comp<C: Into<NetComponent>>(
            &self,
            // The client can only request that components are
            // inserted onto itself.
            // ent: specs::Entity,
            comp: C,
        ) {
            // just using a 0 here for the entity ID since they can
            // only insert components onto their own entity.
            self.send(NetMessage::InsertComp(0, comp.into()));
        }
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
                    let msgs = msgs.clone();

                    let parse_msg_data = move |data: Value| {
                        let buf: ArrayBuffer = data
                            .try_into()
                            .expect("Couldn't turn server message into array buffer!");

                        let mut msgs = msgs.lock().expect("The Server Message Queue is locked!");
                        msgs.push(
                            rmps::from_read_ref::<Vec<u8>, _>(&buf.into())
                                .expect("couldn't read net message bytes"),
                        );
                    };

                    js! {
                        let reader = new FileReader();
                        reader.addEventListener("loadend", () => {
                            let parse = @{parse_msg_data};
                            parse(reader.result);
                            parse.drop();
                        });
                        reader.readAsArrayBuffer(@{msg}.data);
                    };
                }
            });

            Self { ws, message_queue }
        }
    }

    use comn::net::UpdatePosition;
    use specs_physics::SimplePosition;
    pub struct SyncPositions;
    impl<'a> System<'a> for SyncPositions {
        type SystemData = (
            WriteStorage<'a, SimplePosition<f32>>,
            ReadStorage<'a, UpdatePosition>,
        );

        // the idea here is to get wherever the client thinks something is to where the server has
        // it at within 10 ms.
        // You want to do that transition gradually to avoid sudden jerking.
        // If the internet is being slow and the update is from a while ago, however, it's probably
        // more apt to just rely on the physics simulation on the client than on the last position
        // the server sent; that way things in the simulation will still move.
        fn run(&mut self, (mut currents, updates): Self::SystemData) {
            for (SimplePosition(current), UpdatePosition { iso: update, .. }) in
                (&mut currents, &updates).join()
            {
                // this is very lazy and bad, at some point try to keep track of the server clock
                // and then use that to ignore old irrelevant server positions when the 'net cuts
                // out/slows down and let the client physics sim take over then.
                // The challenging part then is just figuring out how to keep clocks synced and
                // figuring out how to factor in ping.
                current.translation.vector = current
                    .translation
                    .vector
                    .lerp(&update.translation.vector, 0.08);

                current.rotation = current.rotation.slerp(&update.rotation, 0.06);
            }
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
                    use NetMessage::*;

                    match msg {
                        NewEnt(server) => {
                            let local: u32 = ents.create().id();
                            self.server_to_local_ids.insert(server, local);
                        }
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
                                net_comp.insert(ent, &lu);
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

mod controls {
    use super::net::ServerConnection;
    use crate::prelude::*;
    use comn::controls::{Heading, Looking};
    use stdweb::{
        traits::IKeyboardEvent,
        unstable::TryInto,
        web::{
            document,
            event::{ConcreteEvent, KeyPressEvent, KeyUpEvent},
            IEventTarget,
        },
    };

    #[derive(Default)]
    pub struct Camera {
        pub pitch_deg: f32,
        pub yaw_deg: f32,
    }
    impl Camera {
        pub fn make_front(&self) -> na::Vector3<f32> {
            let pitch_rad = f32::to_radians(self.pitch_deg);
            let yaw_rad = f32::to_radians(self.yaw_deg);
            na::Vector3::new(
                yaw_rad.sin() * pitch_rad.cos(),
                pitch_rad.sin(),
                yaw_rad.cos() * pitch_rad.cos(),
            )
            .normalize()
        }

        pub fn update_orientation(&mut self, d_pitch_deg: f32, d_yaw_deg: f32) {
            self.pitch_deg = (self.pitch_deg + d_pitch_deg).max(-89.0).min(89.0);
            self.yaw_deg = (self.yaw_deg + d_yaw_deg) % 360.0;
        }
    }

    // Camera Controls System.
    pub struct CameraControl;
    impl<'a> System<'a> for CameraControl {
        type SystemData = (Write<'a, Camera>, Read<'a, ServerConnection>);

        fn run(&mut self, (mut cam, sc): Self::SystemData) {
            // This JS method returns how much the mouse on the [x, y] since
            // the last time the method was called.
            let movement: Vec<f64> = js! { return camMovement(); }.try_into().unwrap();

            if movement.iter().sum::<f64>() != 0.0 {
                let movement = movement
                    .iter()
                    // inverse both, make them much smaller, and cast 'em to f32s.
                    // TODO: replace this 0.05 with a configurable resource
                    .map(|x| *x as f32 * -0.05)
                    .collect::<Vec<f32>>();

                cam.update_orientation(movement[1], movement[0]);

                sc.insert_comp(Looking {
                    dir: na::Unit::new_normalize(cam.make_front()),
                });
            }
        }
    }

    // requires for MovementControl
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex}
    };

    //(key direction, key down)
    type KeyMap = Arc<Mutex<HashMap<char, bool>>>;

    pub struct MovementControl {
        keys: KeyMap,
    }
    impl MovementControl {
        fn handle_key_event<K: IKeyboardEvent + ConcreteEvent>(keys: KeyMap, key_down: bool) {
            document().add_event_listener(move |e: K| {
                if !e.repeat() {
                    let first_letter = e.key().chars().next().expect("zero length key name");
                    if "wsad".contains(first_letter) {
                        keys.lock().expect("Can't lock keys").insert(first_letter, key_down);
                    }
                }
            });
        }
    }
    impl Default for MovementControl {
        fn default() -> Self {
            let keys = Arc::new(Mutex::new(HashMap::new()));

            Self::handle_key_event::<KeyPressEvent>(keys.clone(), true);
            Self::handle_key_event::<KeyUpEvent>(keys.clone(), false);

            MovementControl { keys }
        }
    }
    impl<'a> System<'a> for MovementControl {
        type SystemData = (Read<'a, ServerConnection>, Read<'a, Camera>);

        fn run(&mut self, (sc, cam): Self::SystemData) {
            // if keys isn't being used by the listener,
            if let Ok(keys) = self.keys.try_lock() {
                // these variables are needed to determine direction from key names.
                let forward = cam.make_front();
                let cross_normalized = forward.cross(&na::Vector3::y()).normalize();
                let zero = na::zero();

                if keys.len() > 0 {
                    let move_vec = keys
                        .iter()
                        .fold(zero, |vec: na::Vector3<f32>, key| match key {
                            ('w', true) => vec + forward,
                            ('s', true) => vec - forward,
                            ('a', true) => vec - cross_normalized,
                            ('d', true) => vec + cross_normalized,
                            _ => vec,
                        });

                    // now that we know, tell the server where we'd like to go
                    sc.insert_comp(Heading {
                        dir: na::Unit::new_normalize(move_vec),
                    });
                }
            }
        }
    }
}

fn main() {
    stdweb::initialize();
    // https://github.com/rustwasm/console_error_panic_hook/blob/master/src/lib.rs ?

    // instantiate an ECS world to hold all of the systems, resources, and components.
    let mut world = World::new();

    // add systems and instantiate and order the other systems.
    #[rustfmt::skip]
    let mut dispatcher_builder = DispatcherBuilder::new()
        .with(controls::MovementControl::default(), "move",         &[])
        .with(controls::CameraControl,              "cam",          &[])
        .with(renderer::Render,                     "render",       &["cam"])
        .with(net::HandleServerPackets::default(),  "packets",      &[])
        .with(net::SyncPositions,                   "sync phys",    &[]);
    register_physics_systems::<f32, SimplePosition<f32>>(&mut dispatcher_builder);
    let mut dispatcher = dispatcher_builder.build();

    // go through all of the systems and register components and resources accordingly
    dispatcher.setup(&mut world);

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

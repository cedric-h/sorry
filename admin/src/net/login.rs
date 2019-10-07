// our code
use super::prelude::*;
use log::*;
// crates
use comn::{specs::prelude::*, specs_physics::SimplePosition};

/// This system sends the world to all clients with the LoggingIn component.
pub struct SendWorldToNewPlayers;
impl<'a> System<'a> for SendWorldToNewPlayers {
    type SystemData = (
        // things we need to do networking
        Read<'a, ConnectionManager>,
        WriteStorage<'a, LoggingIn>,
        ReadStorage<'a, Client>,
        // things we need to tell new players about
        Entities<'a>,
        ReadStorage<'a, comn::art::Appearance>,
        ReadStorage<'a, SimplePosition<f32>>,
    );

    fn run(&mut self, (cm, mut logging_ins, clients, ents, appearances, isos): Self::SystemData) {
        for (_, Client(addr)) in (logging_ins.drain(), &clients).join() {
            debug!("We're about to tell a new player about the world.");
            // tell them about each new entity they need to add, and about
            // some crucial components it has.
            for (iso, appearance, ent) in (&isos, appearances.maybe(), &*ents).join() {
                trace!("telling new player about an existing entity");
                cm.new_ent(*addr, ent);
                cm.insert_comp(*addr, ent, iso.clone());
                if let Some(appearance) = appearance {
                    cm.insert_comp(*addr, ent, appearance.clone());
                }
            }

            // tell them which of all of those entities
            // they should be looking out of
            cm.insert_comp(
                *addr,
                ents.entity(cm.addr_to_ent[&addr]),
                comn::controls::Camera,
            );
        }
    }
}

/*
pub struct RegisterNewPlayers;
impl<'a> System<'a> for RegisterNewPlayers {
    type SystemData = (
        Write<'a, ConnectionManager>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, crate::quest::Quests>,
        WriteStorage<'a, RegisterPlayer>,
        ReadStorage<'a, Client>,
    );

    fn run(
        &mut self,
        (mut cm, ents, lu, quests, mut players_to_register, clients): Self::SystemData,
    ) {
        for (
            _,
            ent,
            Client(new_player_addr),
        ) in (players_to_register.drain(), &*ents, &clients).join()
        {
            // these are the components the entity will have.
            let appearance = Appearance::Player;
            let iso = SimplePosition(na::Isometry3::new(na::Vector3::y(), na::zero()));
            //let cam = Camera::default();

            // Note that the Appearance component on the client, is only an event.
            // It's immediately consumed, and composes an appropriate Mesh component for
            // the given entity. On the server, however, the component sticks around.
            // This way, when new clients connect they can simply be sent the Appearance
            // component the server has cached.

            // give them player components
            lu.insert(ent, iso.clone());
            lu.insert(ent, appearance.clone());
            //lu.insert(ent, cam.clone());

            // sooooo.. tell everyone that happened
            for Client(addr) in (&clients).join() {
                cm.new_ent(*addr, ent);
                cm.insert_comp(*addr, ent, iso.clone());
                if addr != new_player_addr {
                    cm.insert_comp(*addr, ent, appearance.clone());
                } else {
                    //cm.insert_comp(*addr, ent, cam.clone());
                }
            }
        }
    }
}*/

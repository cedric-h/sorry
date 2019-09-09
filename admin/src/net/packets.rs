use super::prelude::*;
use comn::{specs, NetMessage};
use log::*;
use specs::prelude::*;

pub struct HandleClientPackets;
impl<'a> System<'a> for HandleClientPackets {
    type SystemData = (
        Write<'a, ConnectionManager>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        WriteStorage<'a, Client>,
        WriteStorage<'a, LoggingIn>,
        //WriteStorage<'a, RegisterPlayer>,
    );

    fn run(
        &mut self,
        (mut cm, ents, lu, mut clients, mut logging_ins/*, mut register_players*/): Self::SystemData,
    ) {
        while let Ok((addr, net_msg)) = cm.from_clients.try_recv() {
            match net_msg {
                // The internal networking system sends this over the channel
                // when a connection to a client has been established.
                NetMessage::NewEnt(_) => {
                    // if their address is already registered, that means this is their
                    // second attempt at making a connection.
                    if let Some(_ent) = cm.addr_to_ent.get(&addr) {
                        // if that's the case, that means that their connection has
                        // stabilized, so I reckon it's time to give them some more
                        // components!
                        /*register_players
                        .insert(ents.entity(*ent), RegisterPlayer)
                        .unwrap(); */
                    } else {
                        // otherwise, welcome!
                        let ent = ents.create();
                        info!("New Player joined, assigned entity {}", ent.id());

                        clients.insert(ent, Client(addr.clone())).unwrap();
                        logging_ins.insert(ent, LoggingIn).unwrap();
                        cm.addr_to_ent.insert(addr, ent.id());
                    }
                }

                // We need to devise some way to prevent the client
                // from inserting certain components onto themselves.
                NetMessage::InsertComp(_, comp) => {
                    let id = cm.addr_to_ent[&addr];
                    trace!("inserting component to Client{}", id);
                    let ent = ents.entity(id);
                    if !ents.is_alive(ent) {
                        panic!("Cannot insert for disconnected client!?");
                    } else {
                        comp.insert(ent, &lu);
                    }
                }
            }
        }
    }
}

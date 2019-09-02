#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub use nalgebra as na;
pub use serde;
pub use rmp_serde as rmps;

pub mod net {
    pub use msg::NetMessage;
    pub use comp::NetComponent;

    mod msg {
        use super::NetComponent;
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize)]
        pub enum NetMessage {
            NewEnt(u32),
            InsertComp(u32, NetComponent),
        }
    }

    mod comp {
        use crate::art::Appearance;
        use specs_physics::SimplePosition;
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize)]
        pub enum NetComponent {
            Appearance(Appearance),
            SimplePosition(SimplePosition<f32>),
        }

        impl NetComponent {
            pub fn insert(self, lu: &specs::LazyUpdate, ent: specs::Entity) {
                match self {
                    NetComponent::Appearance(c) => lu.insert(ent, c),
                    NetComponent::SimplePosition(c) => lu.insert(ent, c),
                }
            }

            pub fn from_comp<C: specs::Component>(comp: C) -> Self {
                use core::any::Any;

                let comp = &comp as &dyn Any;

                // using stupid if and return syntax here instead of implicit return
                // for easier macroing later
                if let Some(c) = comp.downcast_ref::<Appearance>() {
                    return NetComponent::Appearance(c.clone());
                }
                if let Some(c) = comp.downcast_ref::<SimplePosition<f32>>() {
                    return NetComponent::SimplePosition(c.clone());
                }

                unreachable!("Unregistered Component!")
            }
        }
    }
}
pub use net::{NetComponent, NetMessage};

pub mod art {
    use specs::{prelude::*, Component};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Component, Serialize, Deserialize)]
    pub struct Appearance {
        pub filepath: String,
    }
}

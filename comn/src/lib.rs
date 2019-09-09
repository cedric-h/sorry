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
pub use specs;
pub use specs_physics;

pub mod net {
    pub use msg::NetMessage;
    pub use comp::NetComponent;

    mod msg {
        use super::NetComponent;
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Debug)]
        pub enum NetMessage {
            NewEnt(u32),
            InsertComp(u32, NetComponent),
        }
    }

    mod comp {
        use crate::art::Appearance;
        use specs::{Entity, LazyUpdate};
        use specs_physics::SimplePosition;
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, Debug)]
        pub enum NetComponent {
            Appearance(Appearance),
            SimplePosition(SimplePosition<f32>),
        }

        impl From<Appearance> for NetComponent {
            fn from(c: Appearance) -> Self {
                NetComponent::Appearance(c)
            }
        }

        impl From<SimplePosition<f32>> for NetComponent {
            fn from(c: SimplePosition<f32>) -> Self {
                NetComponent::SimplePosition(c)
            }
        }

        impl NetComponent {
            pub fn insert(self, ent: Entity, lu: &LazyUpdate) {
                match self {
                    NetComponent::Appearance(c) => lu.insert(ent, c),
                    NetComponent::SimplePosition(c) => lu.insert(ent, c),
                }
            }
        }
    }
}
pub use net::{NetComponent, NetMessage};

pub mod art {
    use specs::{prelude::*, Component};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Component, Serialize, Deserialize)]
    pub struct Appearance {
        pub filepath: String,
    }
}

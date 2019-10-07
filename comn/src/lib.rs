pub use nalgebra as na;
pub use rmp_serde as rmps;
pub use serde;
pub use specs;
pub use specs_physics;

pub mod prelude {
    pub use super::na;
    pub use super::specs;
    pub use super::specs_physics;
    // specs physics
    pub use specs_physics::{
        colliders::Shape, ncollide::query::Ray, nphysics::object::BodyStatus,
        register_physics_systems, Physics, PhysicsBody, PhysicsBodyBuilder, PhysicsCollider,
        PhysicsColliderBuilder, SimplePosition,
    };
}

pub mod net {
    pub use comp::NetComponent;
    pub use msg::NetMessage;
    // UpdatePosition
    use super::na;
    use serde::{Deserialize, Serialize};
    use specs::{prelude::*, Component};

    #[derive(Clone, Debug, Component, Serialize, Deserialize)]
    /// These wrap around a SimplePosition.
    /// They're sent from the Server to the Client
    /// to update positions, no entity on the Server
    /// should have one of those, though they should
    /// be fairly common on the Client.
    pub struct UpdatePosition {
        pub iso: na::Isometry3<f32>,
        // duration since UNIX_EPOCH
        pub time_stamp: std::time::Duration,
    }

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
        // util includes
        use serde::{Deserialize, Serialize};
        use specs::{Entity, LazyUpdate};

        macro_rules! net_component_base {
            ( $( $x:tt : $y:ty $(: $extra:ident)? ),+ $(,)? ) => {
                #[derive(Deserialize, Serialize, Debug)]
                pub enum NetComponent {
                    $(
                        $x($y),
                    )+
                }

                $(
                    impl From<$y> for NetComponent {
                        fn from(c: $y) -> Self {
                            NetComponent::$x(c)
                        }
                    }
                )+

                impl NetComponent {
                    pub fn insert(self, ent: Entity, lu: &LazyUpdate) {
                        match self {
                            $(
                                NetComponent::$x(c) => lu.insert(ent, c),
                            )+
                        }
                    }
                }
            };
        }

        macro_rules! net_component {
            ( $( $name:ident $(: $inner:ty)? ),+ $(,)? ) => {
                net_component_base! {
                    $($name $(: $inner)? : $name),*
                }
            }
        }

        // Component includes
        use crate::art::Appearance;
        use specs_physics::SimplePosition;
        use super::UpdatePosition;
        use crate::controls::{Looking, Heading, Camera};

        net_component! {
            Appearance,
            SimplePosition: SimplePosition<f32>,
            UpdatePosition,
            Looking,
            Heading,
            Camera,
        }
    }
}
pub use net::{NetComponent, NetMessage};

pub mod art {
    use serde::{Deserialize, Serialize};
    use specs::{prelude::*, Component};

    #[derive(Clone, Debug, Component, Serialize, Deserialize)]
    /// Behavior can affect how something is rendered on the client, but
    /// the appearance should never affect the behavior.
    /// With that in mind:
    /// On the Server, this component isn't really used except for when a new entity is made
    /// and when the clients are being told about it. It sits in the ECS for when new connectees
    /// need it, but other than that it servers no practical purpose, after all, it only describes
    /// how the entity looks, which should never be tied into its behavior. On the client,
    /// this is passed over to three.js and a corresponding file is found for its appearance.
    pub struct Appearance {
        pub filepath: String,
    }

    impl Appearance {
        pub fn new<S: Into<String>>(filepath: S) -> Self {
            Self {
                filepath: filepath.into(),
            }
        }
    }
}

pub mod controls {
    use super::na;
    use serde::{Deserialize, Serialize};
    use specs::{prelude::*, Component};

    #[derive(Clone, Debug, Component, Serialize, Deserialize)]
    /// Nobody gets these on the Server, but the Server
    /// will tell the Client to put one on the entity the Client 
    /// is looking out of at the moment.
    pub struct Camera;

    #[derive(Clone, Debug, Component, Serialize, Deserialize)]
    /// This is sent from the Client to the Server
    /// so that the Server can know where they're looking,
    /// for movement and various other activities that require
    /// that kind of knowledge.
    pub struct Looking {
        pub dir: na::Unit<na::Vector3<f32>>,
    }

    #[derive(Clone, Debug, Component, Serialize, Deserialize)]
    /// Where, relative to where their looking, would the Client like to go?
    /// Note that the server isn't necessarily going to actually get them there.
    pub struct Heading {
        /// The direction that they supply is relative to where their camera is facing.
        /// (camera direction is interpreted as the last Looking the server's gotten from them)
        /// This means that negative X is to their left.
        pub dir: na::Unit<na::Vector3<f32>>,
    }
}

use super::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use stdweb::{
    traits::IKeyboardEvent,
    web::{
        document,
        event::{ConcreteEvent, KeyPressEvent, KeyUpEvent},
        IEventTarget,
    },
};

//(key direction, key down)
type KeyMap = Arc<Mutex<HashMap<char, bool>>>;

pub struct Controls {
    pub keys: KeyMap,
}
impl Controls {
    fn handle_key_event<K: IKeyboardEvent + ConcreteEvent>(keys: KeyMap, key_down: bool) {
        document().add_event_listener(move |e: K| {
            if !e.repeat() {
                let first_letter = e
                    .key()
                    .chars()
                    .next()
                    .expect("zero length key name")
                    .to_lowercase()
                    .next()
                    .expect("there is no lowercase");
                if "wsad,".contains(first_letter) {
                    keys.lock()
                        .expect("Can't lock keys")
                        .insert(first_letter, key_down);
                }
            }
        });
    }
}

impl Default for Controls {
    fn default() -> Self {
        let keys = Arc::new(Mutex::new(HashMap::new()));

        Self::handle_key_event::<KeyPressEvent>(keys.clone(), true);
        Self::handle_key_event::<KeyUpEvent>(keys.clone(), false);

        Controls { keys }
    }
}

impl Controls {
    // takes a mutable reference to the player's position
    // and updates it based on last frame's key input
    // returns a boolean indicating whether or not the player
    // shot a bullet this frame.
    pub fn update(&self, player_pos: &mut Isometry2<f32>) -> bool {
        if let Ok(keys) = self.keys.try_lock() {
            // these variables are needed to determine direction from key names.
            let y = Vector2::y();
            let x = Vector2::x();
            let zero = na::zero();

            if keys.len() > 0 {
                let move_vec = keys.iter().fold(zero, |vec: Vector2<f32>, key| match key {
                    ('w', true) => vec - y,
                    ('s', true) => vec + y,
                    ('a', true) => vec - x,
                    ('d', true) => vec + x,
                    _ => vec,
                });
                player_pos.translation.vector += move_vec * 0.2;
            }

            *keys.get(&',').unwrap_or(&false)
        } else {
            false
        }
    }
}

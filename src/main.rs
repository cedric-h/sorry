#![recursion_limit = "256"]
#[macro_use]
extern crate stdweb;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use stdweb::web::window;

mod bullet;
use bullet::{Bullet, BulletKills as Kills, BulletKind};

mod controls;
use controls::Controls;

mod enemy;
use enemy::Enemy;

mod renderer;

fn random_double() -> f64 {
    use stdweb::unstable::TryInto;
    js! ( return Math.random(); )
        .try_into()
        .expect("couldn't random")
}
fn random() -> f32 {
    random_double() as f32
}

pub mod prelude {
    pub use na::{Isometry2, Vector2};
    pub use nalgebra as na;
    pub use nc::shape::Cuboid;
    pub use ncollide2d as nc;
    pub use serde::{Deserialize, Serialize};
}
use prelude::*;

pub struct Level {
    pub setup: Box<dyn Fn(&mut Game)>,
    pub update: Box<dyn Fn(&mut Game)>,
}

impl Level {
    fn third() -> Self {
        Level {
            setup: Box::new(|game: &mut Game| {
                game.insert_enemy(
                    "Little Doll",
                    Isometry2::translation(17.5, 0.0),
                    Cuboid::new(Vector2::new(1.0, 2.5)),
                    Enemy::new(vec![
                        Vector2::new(11.0, 10.0),
                        Vector2::new(24.0, 15.0),
                        Vector2::new(11.0, 20.0),
                        Vector2::new(24.0, 25.0),
                        Vector2::new(11.0, 28.0),
                        Vector2::new(24.0, 28.0),
                    ], 6, 0.3),
                );
                game.insert_enemy(
                    "Little Doll",
                    Isometry2::translation(8.75, 0.0),
                    Cuboid::new(Vector2::new(1.0, 2.5)),
                    Enemy::new(vec![
                        Vector2::new(8.0, 8.0),
                        Vector2::new(15.0, 12.0),
                        Vector2::new(8.0, 16.0),
                        Vector2::new(15.0, 20.0),
                        Vector2::new(8.0, 24.0),
                        Vector2::new(15.0, 28.0),
                       
                    ], 6, 0.3),
                );
                game.insert_enemy(
                    "Little Doll",
                    Isometry2::translation(26.5, 0.0),
                    Cuboid::new(Vector2::new(1.0, 2.5)),
                    Enemy::new(vec![
                        Vector2::new(14.0, 8.0),
                        Vector2::new(19.0, 12.0),
                        Vector2::new(14.0, 16.0),
                        Vector2::new(19.0, 20.0),
                        Vector2::new(14.0, 24.0),
                        Vector2::new(19.0, 28.0),
                       
                    ], 6, 0.3),
                );
            }),
            update: Box::new({
                let cooldown = Arc::new(Mutex::new(10));

                move |game: &mut Game| {
                    let mut cooldown = cooldown.lock().unwrap();
                    if *cooldown == 0 {
                        for i_enemy in game.enemies.clone().keys() {
                            let enemy_pos = game.isos.get(i_enemy).expect("enemy with no pos");

                            game.insert_bullet(
                                "Flower3",
                                enemy_pos.clone(),
                                Cuboid::new(Vector2::new(1.0, 1.0)),
                                Bullet {
                                    kind: BulletKind::Straight(
                                        Vector2::y() * 0.4 * random() + Vector2::y() * 0.1,
                                    ),
                                    kills: Kills::Good,
                                },
                            );
                        }
                        *cooldown = 8;
                    } else {
                        *cooldown -= 1;
                    }
                    if game.enemies.len() == 0 {
                        js! { alert("You survived third round.") };
                        game.change_level(Level::fourth());
                    }
                }
            }),
        }
    }

    fn fourth() -> Self {
        Level {
            setup: Box::new(|game: &mut Game| {
                game.insert_enemy(
                    "Little Doll",
                    Isometry2::translation(0.0, 0.0),
                    Cuboid::new(Vector2::new(1.0, 2.5)),
                    Enemy::new(vec![
                        Vector2::new(0.0, 1.0),
                        Vector2::new(0.0, 2.0),
                        Vector2::new(2.0, 3.0),
                        Vector2::new(4.0, 4.0),
                        Vector2::new(6.0, 5.0),
                        Vector2::new(8.0, 6.0),
                        Vector2::new(10.0, 7.0),
                        Vector2::new(12.0, 8.0),
                        Vector2::new(14.0, 9.0),
                        Vector2::new(16.0, 10.0),
                        Vector2::new(18.0, 11.0),
                        Vector2::new(20.0, 12.0),
                        Vector2::new(22.0, 13.0),
                        Vector2::new(24.0, 14.0),
                        Vector2::new(26.0, 15.0),
                        Vector2::new(28.0, 16.0),
                        Vector2::new(30.0, 17.0),
                        Vector2::new(32.0, 18.0),
                        Vector2::new(30.0, 14.0),
                        Vector2::new(28.0, 15.0),
                        Vector2::new(26.0, 16.0),
                        Vector2::new(24.0, 17.0),
                        Vector2::new(22.0, 18.0),
                        Vector2::new(20.0, 19.0),
                        Vector2::new(18.0, 20.0),
                        Vector2::new(16.0, 21.0),
                        Vector2::new(14.0, 22.0),
                        Vector2::new(12.0, 23.0),
                        Vector2::new(10.0, 24.0),
                        Vector2::new(8.0, 25.0),
                        Vector2::new(6.0, 26.0),
                        Vector2::new(4.0, 27.0),
                        Vector2::new(6.0, 23.0),
                        Vector2::new(8.0, 24.0),
                        Vector2::new(10.0, 25.0),
                        Vector2::new(12.0, 26.0),
                        Vector2::new(14.0, 28.0),
                        Vector2::new(16.0, 29.0),
                        Vector2::new(18.0, 30.0),
                        Vector2::new(20.0, 31.0),
                        Vector2::new(22.0, 32.0),
                        Vector2::new(24.0, 33.0),
                        Vector2::new(26.0, 34.0),
                        Vector2::new(28.0, 35.0),
                    ], 20, 0.1),
                );
            }),
            update: Box::new({
                let cooldown = Arc::new(Mutex::new(10));

                move |game: &mut Game| {
                    let mut cooldown = cooldown.lock().unwrap();
                    if *cooldown == 0 {
                        for i_enemy in game.enemies.clone().keys() {
                            let enemy_pos = game.isos.get(i_enemy).expect("enemy with no pos").clone();

                            for i in 0..10 {
                                game.insert_bullet(
                                    "Flower3",
                                    enemy_pos.clone(),
                                    Cuboid::new(Vector2::new(1.0, 1.0)),
                                    Bullet {
                                        kind: BulletKind::Straight(
                                            Vector2::new(
                                                1.0 - random() * 2.0,
                                                1.0 - random() * 2.0,
                                                )
                                                .normalize()
                                                * 0.2,
                                        ),
                                        kills: Kills::Good,
                                    },
                                );
                            }
                        }
                        *cooldown = 20;
                    } else {
                        *cooldown -= 1;
                    }
                    if game.enemies.len() == 0 {
                        js! { alert("You survived the fourth round.") };
                        game.change_level(Level::fifth());
                    }
                }
            }),
        }
    }

    fn fifth() -> Self {
        Level {
            setup: Box::new(|game: &mut Game| {
                game.insert_enemy(
                    "Eye",
                    Isometry2::translation(17.5, 5.0),
                    Cuboid::new(Vector2::new(1.0, 2.5)),
                    Enemy::new(
                        vec![Vector2::new(17.5, 4.0), Vector2::new(17.5, 6.0)],
                        6,
                        0.3,
                    ),
                );
            }),
            update: Box::new({
                let cooldown = Arc::new(Mutex::new(10));

                move |game: &mut Game| {
                    let mut cooldown = cooldown.lock().unwrap();
                    if *cooldown == 0 {
                        for i_enemy in game.enemies.clone().keys() {
                            let enemy_pos = game.isos.get(i_enemy).expect("enemy with no pos").clone();

                            for i in 0..20 {
                                game.insert_bullet(
                                    "Flower3",
                                    enemy_pos.clone(),
                                    Cuboid::new(Vector2::new(1.0, 1.0)),
                                    Bullet {
                                        kind: BulletKind::Straight(
                                            Vector2::new(1.0 - random() * 2.0, random())
                                                .normalize()
                                                * 0.6,
                                        ),
                                        kills: Kills::Good,
                                    },
                                );
                            }
                        }
                        *cooldown = 8;
                    } else {
                        *cooldown -= 1;
                    }
                    if game.enemies.len() == 0 {
                        js! { alert("You survived final round.") };
                        game.change_level(Level::second());
                    }
                }
            }),
        }
    }

    fn second() -> Self {
        Level {
            setup: Box::new(|game: &mut Game| {
                for i in 0..40 {
                    game.insert_bullet(
                        "Flower3",
                        Isometry2::translation(0.0, (i as f32) * 1.0),
                        Cuboid::new(Vector2::new(1.0, 1.0)),
                        Bullet {
                            kind: BulletKind::Straight(
                                Vector2::x() * 0.4 * random() + Vector2::x() * 0.1,
                            ),
                            kills: Kills::Good,
                        },
                    );
                }
            }),
            update: Box::new(|game: &mut Game| {
                if game.bullets.len() == 0 {
                    js! { alert("You survived second round.") };
                    game.change_level(Level::third());
                }
            }),
        }
    }

    fn first() -> Self {
        Level {
            setup: Box::new(|game: &mut Game| {
                game.insert_enemy(
                    "Eye",
                    Isometry2::translation(17.5, 5.0),
                    Cuboid::new(Vector2::new(1.0, 2.5)),
                    Enemy::new(vec![
                        Vector2::new(17.5, 4.0),
                        Vector2::new(17.5, 6.0),
                    ], 6, 0.3),
                );
            }),
            update: Box::new({
                let cooldown = Arc::new(Mutex::new(10));

                move |game: &mut Game| {
                    let mut cooldown = cooldown.lock().unwrap();
                    if *cooldown == 0 {
                        for i_enemy in game.enemies.clone().keys() {
                            let enemy_pos = game.isos.get(i_enemy).expect("enemy with no pos");

                            game.insert_bullet(
                                "Flower3",
                                enemy_pos.clone(),
                                Cuboid::new(Vector2::new(1.0, 1.0)),
                                Bullet {
                                    kind: BulletKind::Straight(
                                        Vector2::new(
                                            1.0 - random() * 2.0,
                                            random(),
                                        ).normalize() * 0.8
                                    ),
                                    kills: Kills::Good,
                                },
                            );
                        }
                        *cooldown = 12;
                    } else {
                        *cooldown -= 1;
                    }
                    if game.enemies.len() == 0 {
                        js! { alert("You survived first round.") };
                        game.change_level(Level::second());
                    }
                }
            }),
        }
    }
}

pub struct Game {
    // components
    pub isos: HashMap<usize, Isometry2<f32>>,
    pub bullets: HashMap<usize, Bullet>,
    pub enemies: HashMap<usize, Enemy>,
    pub hitboxes: HashMap<usize, Cuboid<f32>>,
    pub appearances: HashMap<usize, String>,

    // controls
    pub controls: Controls,
    pub shooting_cooldown: usize,

    // screen size
    pub screen: Cuboid<f32>,

    //state
    pub level: Arc<Mutex<Level>>,

    //entities
    pub player: usize,
    ent_count: usize,
}

impl Game {
    pub fn new(level: Level) -> Self {
        let mut new_game = Self {
            // components
            isos: HashMap::new(),
            bullets: HashMap::new(),
            enemies: HashMap::new(),
            hitboxes: HashMap::new(),
            appearances: HashMap::new(),

            // controls
            controls: Controls::default(),
            shooting_cooldown: 0,

            // // state
            // dumb level to override
            level: Arc::new(Mutex::new(Level {
                setup: Box::new(|_: &mut Game| {}),
                update: Box::new(|_: &mut Game| {}),
            })),

            // state
            screen: Cuboid::new(Vector2::new(35.0, 35.0)),

            // entities
            player: 0,
            ent_count: 0,
        };
        new_game.change_level(level);
        new_game
    }

    pub fn delete_ent(&mut self, kill: &usize) {
        self.isos.remove(kill);
        self.bullets.remove(kill);
        self.enemies.remove(kill);
        self.hitboxes.remove(kill);
        self.appearances.remove(kill);
    }

    pub fn insert_bullet<S: Into<String>>(&mut self, appearance: S, iso: Isometry2<f32>, hb: Cuboid<f32>, bullet: Bullet) -> usize {
        let ent = self.entity();

        self.isos.insert(ent, iso);
        self.bullets.insert(ent, bullet);
        self.hitboxes.insert(ent, hb);
        self.appearances.insert(ent, appearance.into());

        ent
    }

    pub fn insert_enemy<S: Into<String>>(&mut self, appearance: S, iso: Isometry2<f32>, hb: Cuboid<f32>, enemy: Enemy) -> usize {
        let ent = self.entity();

        self.isos.insert(ent, iso);
        self.enemies.insert(ent, enemy);
        self.hitboxes.insert(ent, hb);
        self.appearances.insert(ent, appearance.into());

        ent
    }
    pub fn player(&mut self) -> usize {
        self.player = self.entity();
        self.player
    }

    pub fn change_level(&mut self, level: Level) {
        self.controls.keys.lock().unwrap().clear();
        (level.setup)(self);
        self.level = Arc::new(Mutex::new(level));
    }

    pub fn entity(&mut self) -> usize {
        let ent = self.ent_count;
        self.ent_count += 1;
        ent
    }

    pub fn max_entity(&self) -> usize {
        self.ent_count
    }
}

fn main() {
    stdweb::initialize();

    let mut game = Game::new(Level::first());

    // if you instantiate an entity before this,
    // I will byte you. (also the program won't work)
    let player = game.player();
    game.isos
        .insert(player, Isometry2::translation(35.0 / 2.0, 35.0 / 2.0));
    game.hitboxes.insert(player, Cuboid::new(Vector2::new(1.0, 1.0)));
    game.appearances
        .insert(player, "Heart".to_string());

    fn game_loop(mut game: Game) {
        use nc::query::PointQuery;

        let should_shoot = game
            .controls
            .update(&mut game.isos.get_mut(&game.player).unwrap());
        let player_pos = game.isos[&game.player];

        if should_shoot {
            if game.shooting_cooldown == 0 {
                game.insert_bullet(
                    "Flower3",
                    player_pos.clone(),
                    Cuboid::new(Vector2::new(1.0, 1.0)),
                    Bullet {
                        kind: BulletKind::Straight(Vector2::y() * -0.4),
                        kills: Kills::Bad,
                    },
                );
                game.shooting_cooldown = 5;
            } else {
                game.shooting_cooldown -= 1;
            }
        }

        let mut dead = Vec::new();

        // delete anything with a hitbox when it goes off the screen
        for i_hitbox in game.hitboxes.keys() {
            if !game.screen.contains_point(
                &Isometry2::identity(),
                &na::Point::from(
                    game.isos
                        .get(i_hitbox)
                        .expect("bullet with no pos")
                        .translation
                        .vector,
                ),
            ) {
                if *i_hitbox == game.player {
                    js! {
                        alert("You died!");
                        location.reload();
                    }
                } else {
                    dead.push(*i_hitbox);
                }
            }
        }

        // update enemy positions
        for (i_enemy, enemy) in game.enemies.iter_mut() {
            let mut enemy_pos = game.isos.get_mut(i_enemy).expect("enemy with no pos");

            enemy.update(&mut enemy_pos);
        }

        // (index of bullet, bullet)
        for (i_bullet, bullet) in game.bullets.iter_mut() {
            bullet.update(&mut game.isos.get_mut(i_bullet).expect("bullet with no pos"));

            let bullet_hitbox = game.hitboxes.get(i_bullet).expect("bullet with no hitbox");
            let bullet_pos = game.isos.get(i_bullet).expect("bullet with no pos");

            match bullet.kills {
                Kills::Good => {
                    if bullet_hitbox
                        .contains_point(bullet_pos, &na::Point::from(player_pos.translation.vector))
                    {
                        js! {
                            alert("you died!");
                            location.reload();
                        }
                    }
                }
                Kills::Bad => {
                    for (i_enemy, enemy) in game.enemies.iter_mut() {
                        let enemy_hitbox =
                            game.hitboxes.get(i_enemy).expect("enemy with no hitbox");
                        let enemy_pos = game.isos.get(i_enemy).expect("enemy with no pos");

                        use nc::query::{proximity, Proximity};

                        match proximity(bullet_pos, bullet_hitbox, enemy_pos, enemy_hitbox, 0.1) {
                            Proximity::Intersecting => {
                                dead.push(*i_bullet);
                                if enemy.damage() {
                                    dead.push(*i_enemy);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // actually kill the bullets
        for kill in dead.iter() {
            game.delete_ent(kill);
        }

        let level = game.level.clone();
        if let Ok(level) = level.lock() {
            (level.update)(&mut game);
        }

        let render_data = renderer::RenderData {
            ents: (0..game.max_entity())
                .filter_map(|i| {
                    Some(renderer::MeshBundle {
                        ent: i,
                        size: game.hitboxes
                            .get(&i)
                            .map(|hb| hb.half_extents())
                            .unwrap_or(&Vector2::repeat(1.0))
                            .clone(),
                        appearance: game.appearances.get(&i)?.clone(),
                        iso: game.isos.get(&i)?.clone(),
                    })
                })
                .collect::<Vec<_>>(),
        };
        js! {
            render(@{render_data})
        }

        // tell browser to repeat me the next time the monitor is going to refresh
        window().request_animation_frame(|_| game_loop(game));
    }

    game_loop(game);

    stdweb::event_loop();
}

use super::prelude::*;

pub struct Bullet {
    pub kind: BulletKind,
    pub kills: BulletKills,
}

pub enum BulletKind {
    Straight(Vector2<f32>),
    /*
    Wave {
        start: Vector2<f32>,
        trajectory: Vector2<f32>,
        accumulator: usize,
    },*/
}

pub enum BulletKills {
    Good,
    Bad,
}

impl Bullet {
    // updates a bullet's state and its position consequently
    pub fn update(&mut self, pos: &mut Isometry2<f32>) {
        use BulletKind::*;

        match self.kind {
            Straight(trajectory) => {
                pos.translation.vector += trajectory;
            }
        }
    }

    /*
    pub fn straight(trajectory: Vector2<f32>, kills: BulletKills) -> Self {
        Bullet {
            kind: BulletKind::Straight(trajectory),
            kills,
        }
    }*/
}

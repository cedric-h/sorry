use super::*;

#[derive(Clone)]
pub struct Enemy {
    pub route: Vec<Vector2<f32>>,
    pub health: usize,
    pub goal: usize,
}

impl Enemy {
    pub fn new(route: Vec<Vector2<f32>>, health: usize) -> Self {
        Enemy {
            route,
            health,
            goal: 0,
        }
    }

    pub fn update(&mut self, pos: &mut Isometry2<f32>) {
        let vec = pos.translation.vector;
        let to_go = self.route[self.goal];
        pos.translation.vector -= (vec - to_go).normalize() * 0.3;

        if (vec - to_go).magnitude().abs() < 0.2 {
            self.goal += 1;
            if self.goal >= self.route.len() {
                self.goal = 0;
            }
        }
    }

    // returns a boolean indicating whether or not the player died
    pub fn damage(&mut self) -> bool {
        self.health -= 1;
        self.health == 0
    }
}

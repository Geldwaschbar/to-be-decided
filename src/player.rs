use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 6.15;

#[derive(Debug)]
pub struct Player {
    position: Vec2,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Vec2::ZERO,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 5., RED);
    }

    pub fn movement(&mut self) {
        let mut diff = Vec2::ZERO;
        if is_key_down(KeyCode::W) {
            diff.y -= 1.;
        }
        if is_key_down(KeyCode::A) {
            diff.x -= 1.;
        }
        if is_key_down(KeyCode::S) {
            diff.y += 1.;
        }
        if is_key_down(KeyCode::D) {
            diff.x += 1.;
        }
        if diff != Vec2::ZERO {
            let next = self.position + diff * PLAYER_SPEED;
            self.set_position(next);
        }
    }

    pub fn set_position(&mut self, next: Vec2) {
        self.position = next;
    }

    pub fn get_position(&self) -> &Vec2 {
        &self.position
    }
}

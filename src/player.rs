use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 6.15;

#[derive(Debug)]
pub struct Player {
    money: f64,
}

impl Player {
    pub fn new() -> Player {
        Player { money: 0. }
    }
}

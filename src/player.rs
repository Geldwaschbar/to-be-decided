use macroquad::prelude::*;

#[derive(Debug)]
pub struct Player {
    money: f64,
}

impl Player {
    pub fn new() -> Player {
        Player { money: 0. }
    }

    pub fn income(&mut self, value: f64) {
        self.money += value;
    }

    pub fn can_buy(&self, value: f64) -> bool {
        self.money >= value
    }
}

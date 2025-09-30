use macroquad::prelude::*;
use serde::Deserialize;

#[derive(Debug)]
pub struct Party {
    pub approval: f32,
    pub popularity: f32,
    pub color: Color,
}

#[derive(Debug, Deserialize)]
pub struct Law {
    pub description: String,
}

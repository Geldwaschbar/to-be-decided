use macroquad::prelude::*;
use macroquad::ui::{Ui, hash, widgets::Group};
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

impl Law {
    pub fn draw_on(&self, ui: &mut Ui) {
        Group::new(hash!(&self.description), Vec2::new(390., 80.)).ui(ui, |ui| {
            for line in self.description.split('\n') {
                ui.label(None, line);
            }
        });
    }
}

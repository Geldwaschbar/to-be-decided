use crate::effect::Effect;
use macroquad::prelude::*;
use macroquad::ui::{Ui, hash, widgets::Group};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Event {
    /// e.g. BOTNET-Auftrag
    pub source: String,
    /// e.g. Smart Fridge nimmt Affäre von Politiker xyz auf.
    pub description: String,
    /// e.g. +x% money
    #[serde(default)]
    pub effects: Vec<Effect>,
}

impl Event {
    pub fn draw_on(&self, ui: &mut Ui) {
        Group::new(hash!(&self.source, &self.description), Vec2::new(290., 80.)).ui(ui, |ui| {
            for line in self.description.split('\n') {
                ui.label(None, line);
            }
            ui.label(None, &format!(" - {}", self.source));
        });
    }
}

use crate::component::Component;
use crate::effect::Effect;
use macroquad::prelude::*;
use macroquad::ui::{Ui, hash, widgets::Group};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

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
    pub fn new(source: String, description: String) -> Event {
        Event {
            source,
            description,
            effects: Default::default(),
        }
    }
}

impl Component for Event {
    fn draw_on(&self, ui: &mut Ui) {
        Group::new(hash!(&self.source, &self.description), Vec2::new(290., 80.)).ui(ui, |ui| {
            for line in self.description.split('\n') {
                ui.label(None, line);
            }
            ui.label(None, &format!(" - {}", self.source));
        });
    }
}

pub type News = VecDeque<Event>;

impl Component for News {
    fn draw_on(&self, ui: &mut Ui) {
        for event in self {
            event.draw_on(ui);
        }
    }
}

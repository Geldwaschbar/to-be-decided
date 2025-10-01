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

    pub chance: f32,
}

impl Event {
    pub fn new(source: String, description: String) -> Event {
        Event {
            source,
            description,
            effects: Default::default(),
            chance: 0.,
        }
    }
}

impl Component for Event {
    fn draw_on(&mut self, ui: &mut Ui) {
        Group::new(hash!(&self.source, &self.description), Vec2::new(290., 80.)).ui(ui, |ui| {
            for line in self.description.split('\n') {
                ui.label(None, line);
            }
            ui.label(None, &format!(" - {}", self.source));
        });
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct News {
    pub available: VecDeque<Event>,
    pub current: VecDeque<Event>,
}

impl News {
    pub fn update(&mut self) {
        for event in &self.available {}
    }
}

impl Component for News {
    fn draw_on(&mut self, ui: &mut Ui) {
        for event in &mut self.current {
            event.draw_on(ui);
        }
    }
}

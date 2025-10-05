use crate::{
    component::{Component, limit, wrap},
    effect::Effect,
};
use macroquad::prelude::*;
use macroquad::ui::{Ui, hash, widgets::Group};
use serde::Deserialize;
use std::{collections::VecDeque, rc::Rc};

#[derive(Clone, Default, Deserialize)]
pub struct Event {
    /// e.g. BOTNET-Auftrag
    pub source: String,
    /// e.g. Smart Fridge nimmt Affäre von Politiker xyz auf.
    pub description: String,
    /// e.g. +x% money
    #[serde(default)]
    pub effects: Vec<Rc<Effect>>,
    /// the chance that this event randomly occurs
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

#[derive(Default, Deserialize)]
pub struct News {
    pub available: VecDeque<Event>,
    pub current: VecDeque<Event>,
    real_time: f32,
}

impl News {
    pub async fn new() -> News {
        let serialized = load_string("assets/news.json").await.unwrap();
        serde_json::from_str::<News>(&serialized).unwrap()
    }
}

impl Component for News {
    fn draw_on(&mut self, ui: &mut Ui, font: &Font) {
        let mut counter = 0;
        for event in &self.current {
            let widget_size = Vec2::new(390., 80.);
            Group::new(hash!(counter, &event.description), widget_size).ui(ui, |ui| {
                for line in wrap(&event.description, widget_size.x, font) {
                    ui.label(None, &line);
                }
                ui.label(None, &format!(" - {}", event.source));
            });
            counter += 1;
        }
    }

    fn update(&mut self, effects: &mut Vec<Rc<Effect>>) {
        self.real_time += limit(get_frame_time(), 5.0);
        if self.real_time >= 1. {
            let mut triggered = Vec::new();
            let mut i = 0;
            for event in &self.available {
                if rand::gen_range(0.0, 1.0) < event.chance {
                    triggered.push(i);
                    for effect in &event.effects {
                        effects.push(effect.clone());
                    }
                    self.current.push_front(event.clone());
                }
                i += 1;
            }
            i = 0;
            for index in triggered {
                self.available.remove(index - i);
                i += 1;
            }
            self.real_time -= 1.
        }
        while self.current.len() > 10 {
            self.current.pop_back().expect("expected event exists");
        }
    }
}

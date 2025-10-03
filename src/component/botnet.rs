use crate::{
    component::{Component, wrap},
    effect::{Effect, MarketResolution, ModifierType, ParlamentResolution},
};
use macroquad::prelude::*;
use macroquad::ui::{Ui, hash};
use std::rc::Rc;

pub struct Botnet {
    pub capacity: f32,
    malware: f32,
    memes: f32,
    crypto_mining: f32,
    bribery: f32,
    pub show: bool,
    pub show_malware: bool,
    pub show_memes: bool,
}

impl Botnet {
    pub fn new() -> Botnet {
        Botnet {
            capacity: 1.0,
            malware: 0.0,
            memes: 0.0,
            crypto_mining: 0.0,
            bribery: 0.0,
            show: false,
            show_malware: false,
            show_memes: false,
        }
    }
}

impl Component for Botnet {
    fn draw_on(&mut self, ui: &mut Ui, font: &Font) {
        for line in wrap(
            &format!("Größe des Botnetzwerkes: {}", self.capacity as usize),
            250.,
            font,
        ) {
            ui.label(None, &line);
        }

        ui.slider(hash!(), "Crypto Mining", 0.0..1.0, &mut self.crypto_mining);
        ui.slider(hash!(), "Bestechung", 0.0..1.0, &mut self.bribery);

        if self.show_memes {
            ui.slider(hash!(), "Memes", 0.0..1.0, &mut self.memes);
        }
        if self.show_malware {
            ui.slider(hash!(), "Malware", 0.0..1.0, &mut self.malware);
        }
    }

    fn update(&mut self, effects: &mut Vec<Rc<Effect>>) {
        let total_usage = self.crypto_mining + self.malware + self.memes;

        if self.malware > 0. {
            self.capacity +=
                (self.malware / total_usage * get_frame_time()) / (self.capacity * self.capacity);
        }
        if self.memes > 0. {
            effects.push(Rc::new(Effect::ParlamentEffect {
                resolution: ParlamentResolution::Transfer,
                modifier: ModifierType::Constant,
                value: self.memes / total_usage * self.capacity * 0.00008 * get_frame_time(),
                party: 3,
            }));
        }
        if self.crypto_mining > 0. {
            effects.push(Rc::new(Effect::MarketEffect {
                resolution: MarketResolution::Money,
                modifier: ModifierType::Constant,
                value: self.crypto_mining / total_usage * self.capacity * get_frame_time(),
            }));
        }
        if self.bribery > 0. {
            effects.push(Rc::new(Effect::ParlamentEffect {
                resolution: ParlamentResolution::Approval,
                modifier: ModifierType::Constant,
                value: self.bribery / total_usage * self.capacity * 0.0001 * get_frame_time(),
                party: rand::gen_range(0, 2),
            }));
        }
    }
}

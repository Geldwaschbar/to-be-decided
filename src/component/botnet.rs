use crate::{
    component::{wrap, Component},
    effect::{Effect, MarketResolution, ModifierType, ParlamentResolution}, style::{COL_BAR_BG, FONT_SIZE, USAGE_COLS},
};
use macroquad::prelude::*;
use macroquad::ui::{Ui, hash};
use std::rc::Rc;

pub struct Botnet {
    pub capacity: f32,
    total_usage: f32,
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
            total_usage: 0.0,
            malware: 0.0,
            memes: 0.0,
            crypto_mining: 0.0,
            bribery: 0.0,
            //TODO: change these to false
            show: true,
            show_malware: true,
            show_memes: true,
        }
    }
}

impl Component for Botnet {
    fn draw_on(&mut self, ui: &mut Ui, font: &Font) {
        let mut canvas = ui.canvas();
        let cursor = Vec2::new(30., screen_height() * 0.5 - 125.);
        let window_size : Vec2 = Vec2::new(500., 500.);
        let bar_width: f32 = window_size.x - 100.;
        // Draw usage bar chart
        canvas.rect(
            Rect::new(
                cursor.x + 50.,
                cursor.y + 20.,
                bar_width,
                15.,
            ),
            COL_BAR_BG,
            COL_BAR_BG
        );
        let mut prev_pos: f32 = 0.0;
        let mut col: usize = 0;
        for usage in [self.crypto_mining, self.bribery, self.memes, self.malware] {
            let usage_width: f32;
            usage_width = usage/self.total_usage*bar_width;
            canvas.rect(
                Rect::new(
                    cursor.x + 50. + prev_pos,
                    cursor.y + 20.,
                    usage_width,
                    15.,
                ),
                BLACK,
                USAGE_COLS[col]
            );
            prev_pos += usage_width;
            col+=1
        }

        ui.label(None, "");
        ui.label(None, "");
        ui.label(None, "");
        for line in wrap(
            &format!(" < Bots: {} > ", self.capacity as usize),
            250.,
            font,
        ) {
            ui.label(Some(Vec2::new(250.,20.)-get_text_center(&line, Some(&font), FONT_SIZE, 1., 0.)), &line);
        }

        ui.slider(hash!(), "Crypto Mining", 0.0..1.0, &mut self.crypto_mining);
        ui.slider(hash!(), "Bestechung", 0.0..1.0, &mut self.bribery);

        if self.show_memes {
            ui.slider(hash!(), "Memes", 0.0..1.0, &mut self.memes);
        }
        if self.show_malware {
            ui.slider(hash!(), "Malware", 0.0..1.0, &mut self.malware);
        }

        if ui.button(None, "Sende Spammail") {
            self.capacity += 1.;
        }
    }

    fn update(&mut self, effects: &mut Vec<Rc<Effect>>) {
        self.total_usage = self.crypto_mining + self.malware + self.memes + self.bribery;

        if self.malware > 0. {
            self.capacity +=
                (self.malware / self.total_usage * get_frame_time()) / (self.capacity * self.capacity);
        }
        if self.memes > 0. {
            effects.push(Rc::new(Effect::ParlamentEffect {
                resolution: ParlamentResolution::Transfer,
                modifier: ModifierType::Constant,
                value: self.memes / self.total_usage * self.capacity * 0.00008 * get_frame_time(),
                party: 3,
            }));
        }
        if self.crypto_mining > 0. {
            effects.push(Rc::new(Effect::MarketEffect {
                resolution: MarketResolution::Money,
                modifier: ModifierType::Constant,
                value: self.crypto_mining / self.total_usage * self.capacity * get_frame_time(),
            }));
        }
        if self.bribery > 0. {
            effects.push(Rc::new(Effect::ParlamentEffect {
                resolution: ParlamentResolution::Approval,
                modifier: ModifierType::Constant,
                value: self.bribery / self.total_usage * self.capacity * 0.0001 * get_frame_time(),
                party: rand::gen_range(0, 2),
            }));
        }
    }
}

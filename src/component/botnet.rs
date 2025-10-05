use crate::{
    component::{Component, limit, wrap},
    effect::{Effect, MarketResolution, ModifierType, ParlamentResolution},
    style::{COL_BAR_BG, FONT_SIZE, USAGE_COLS},
};
use macroquad::prelude::*;
use macroquad::{
    audio::{Sound, load_sound_from_bytes, play_sound_once},
    ui::{Ui, hash},
};
use std::rc::Rc;

pub struct Botnet {
    sound: Sound,
    pub capacity: f32,
    malware: f32,
    memes: f32,
    crypto_mining: f32,
    bribery: f32,
    pub show: bool,
    pub show_malware: bool,
    pub show_memes: bool,
    pub show_bribery: bool,
}

impl Botnet {
    pub async fn new() -> Botnet {
        let sound = load_sound_from_bytes(include_bytes!("../../assets/audio/send.wav"))
            .await
            .ok()
            .unwrap();
        Botnet {
            sound,
            capacity: 1.0,
            malware: 0.0,
            memes: 0.0,
            crypto_mining: 0.0,
            bribery: 0.0,
            show: false,
            show_malware: false,
            show_memes: false,
            show_bribery: false,
        }
    }
}

impl Component for Botnet {
    fn draw_on(&mut self, ui: &mut Ui, font: &Font) {
        let mut canvas = ui.canvas();
        let cursor = Vec2::new(30., screen_height() * 0.5 - 125.);
        let window_size: Vec2 = Vec2::new(500., 500.);
        let bar_width: f32 = window_size.x - 100.;
        // Draw usage bar chart
        canvas.rect(
            Rect::new(cursor.x + 50., cursor.y + 20., bar_width, 15.),
            COL_BAR_BG,
            COL_BAR_BG,
        );
        let mut prev_pos: f32 = 0.0;
        let mut col: usize = 0;
        let total_usage = self.crypto_mining + self.bribery + self.memes + self.malware;
        for usage in [self.crypto_mining, self.bribery, self.memes, self.malware] {
            let usage_width: f32;
            usage_width = usage / total_usage * bar_width;
            canvas.rect(
                Rect::new(cursor.x + 50. + prev_pos, cursor.y + 20., usage_width, 15.),
                BLACK,
                USAGE_COLS[col],
            );
            prev_pos += usage_width;
            col += 1
        }

        ui.label(None, "");
        ui.label(None, "");
        ui.label(None, "");
        for line in wrap(
            &format!(" < Bots: {} > ", self.capacity as usize),
            250.,
            font,
        ) {
            ui.label(
                Some(Vec2::new(250., 20.) - get_text_center(&line, Some(&font), FONT_SIZE, 1., 0.)),
                &line,
            );
        }

        ui.slider(hash!(), "Crypto Mining", 0.0..1.0, &mut self.crypto_mining);
        if self.show_bribery {
            ui.slider(hash!(), "Bestechung", 0.0..1.0, &mut self.bribery);
        }
        if self.show_memes {
            ui.slider(hash!(), "Memes", 0.0..1.0, &mut self.memes);
        }
        if self.show_malware {
            ui.slider(hash!(), "Malware", 0.0..1.0, &mut self.malware);
        }

        if ui.button(None, "Sende Spammail") {
            self.capacity += 1.;
            play_sound_once(&self.sound);
        }
    }

    fn update(&mut self, effects: &mut Vec<Rc<Effect>>) {
        let total_usage =
            (self.crypto_mining + self.malware + self.memes + self.bribery) / self.capacity;

        let frame_time = limit(get_frame_time(), 5.0);
        if self.malware > 0. {
            self.capacity += self.malware / total_usage * 0.001 * frame_time;
        }
        if self.memes > 0. {
            effects.push(Rc::new(Effect::ParlamentEffect {
                resolution: ParlamentResolution::Transfer,
                modifier: ModifierType::Constant,
                value: self.memes / total_usage * 0.00008 * frame_time,
                party: 3,
            }));
        }
        if self.crypto_mining > 0. {
            effects.push(Rc::new(Effect::MarketEffect {
                resolution: MarketResolution::Money,
                modifier: ModifierType::Constant,
                value: self.crypto_mining / total_usage * 0.1 * frame_time,
            }));
        }
        if self.bribery > 0. {
            effects.push(Rc::new(Effect::ParlamentEffect {
                resolution: ParlamentResolution::Approval,
                modifier: ModifierType::Constant,
                value: self.bribery / total_usage * 0.0001 * frame_time,
                party: rand::gen_range(0, 2),
            }));
        }
    }
}

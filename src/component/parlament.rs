use crate::{component::Component, effect::Effect};
use macroquad::prelude::*;
use macroquad::ui::{Ui, hash, widgets::Group};
use serde::Deserialize;
use std::{collections::VecDeque, f64::consts::PI};

// TODO: increase voting time
const VOTING_TIME: f32 = 10.;

#[derive(Debug)]
pub struct Party {
    pub approval: f32,
    pub popularity: f32,
    pub color: Color,
}

#[derive(Debug)]
pub struct Parlament {
    pub parties: Vec<Party>,
    pub voting_progress: f32,
    pub available_laws: VecDeque<Law>,
    pub passed_laws: VecDeque<Law>,
}

impl Parlament {
    pub async fn new() -> Parlament {
        let parties = vec![
            Party {
                approval: 0.34,
                popularity: 0.45,
                color: RED,
            },
            Party {
                approval: 0.82,
                popularity: 0.35,
                color: GREEN,
            },
            Party {
                approval: 0.82,
                popularity: 0.2,
                color: BLUE,
            },
        ];

        let available_laws: VecDeque<Law> = {
            let serialized = load_string("assets/laws.json").await.unwrap();
            serde_json::from_str(&serialized).unwrap()
        };

        let passed_laws: VecDeque<Law> = VecDeque::new();

        Parlament {
            parties,
            available_laws,
            passed_laws,
            voting_progress: 0.,
        }
    }
}

impl Component for Parlament {
    fn draw_on(&mut self, ui: &mut Ui) {
        let mut canvas = ui.canvas();
        let cursor = canvas.cursor();

        const TOTAL_SEATS: f32 = (5 * 4 + 4 * 3) as f32;
        const WINDOW_CENTER: f32 = 190.;
        let mut placed = 0.;
        let mut party_num = 0;
        for arc in 0..9 {
            let base = if arc % 2 == 0 { 4 } else { 3 };
            for row in 0..base {
                let party = self.parties.get(party_num).expect("expect party exists");
                let angle = arc as f32 / 8. * PI as f32;
                // Draw a single parlament seat
                canvas.rect(
                    Rect::new(
                        WINDOW_CENTER + cursor.x - angle.cos() * 40. * (row + 5 - base) as f32,
                        WINDOW_CENTER + cursor.y - angle.sin() * 40. * (row + 5 - base) as f32,
                        15.0,
                        15.0,
                    ),
                    Color::new(0.2, 0.2, 0.2, 1.0),
                    party.color,
                );
                placed += (1.0 / party.popularity) / TOTAL_SEATS;
                // If we draw 100% of a party, go to the next party.
                if placed >= 1. {
                    placed = 0.;
                    party_num += 1;
                }
            }
        }

        canvas.rect(
            Rect::new(cursor.x + 8., cursor.y + 260., 380., 15.),
            Color::new(0.2, 0.2, 0.2, 1.0),
            WHITE,
        );
        let progress = self.voting_progress;
        canvas.rect(
            Rect::new(cursor.x + 8., cursor.y + 260., progress * 380., 15.),
            Color::new(0.2, 0.2, 0.2, 1.0),
            GRAY,
        );

        {
            let text = "Es wird über das nächste Gesetz abgestimmt.";
            let size = measure_text(text, None, 14, 1.);
            ui.label(Vec2::new(WINDOW_CENTER - size.width * 0.5, 230.), text);
        }
        let law = self.available_laws.front().expect("expected law exists");
        {
            let text = &format!("Es wird über \"{}\" abgestimmt.", law.title);
            let size = measure_text(text, None, 14, 1.);
            ui.label(Vec2::new(WINDOW_CENTER - size.width * 0.5, 290.), text);
        }
        {
            let mut approval = 0.;
            for party in &self.parties {
                if party.approval >= law.required_approval {
                    approval += party.popularity * 100.;
                }
            }
            let text = &format!("Die Zustimmung für dieses Gesetz beträgt {}%.", approval);
            let size = measure_text(text, None, 14, 1.);
            ui.label(Vec2::new(WINDOW_CENTER - size.width * 0.5, 310.), text);
        }
    }

    fn update(&mut self, effects: &mut Vec<Effect>) {
        self.voting_progress += get_frame_time() / VOTING_TIME;
        let progress = self.voting_progress;
        if progress >= 1. {
            let law = self.available_laws.front().expect("expected law exists");
            let mut votes = 0.;
            for party in &self.parties {
                if party.approval >= law.required_approval {
                    votes += party.popularity;
                }
            }
            if votes > 0.5 {
                if law.recurring {
                    for effect in &law.effects {
                        effects.push(effect.clone());
                    }
                    self.available_laws.push_back(law.clone());
                } else {
                    self.passed_laws.push_back(law.clone());
                }
                self.available_laws.pop_front();
            }
            self.voting_progress -= 1.0;

            for law in &self.passed_laws {
                for effect in &law.effects {
                    effects.push(effect.clone());
                }
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Law {
    pub title: String,
    /// The description of this law. Please insert '\n' in a long text yourself.
    pub description: String,
    /// How much approval do you need from a party to get there votes?
    pub required_approval: f32,
    /// The publicity describes how likly it is for the parlament to decide upon
    /// this law.
    #[serde(default)]
    pub publicity: f32,
    pub effects: Vec<Effect>,
    /// Whether or not this law can be passed multiple times.
    #[serde(default)]
    pub recurring: bool,
}

impl Law {
    pub fn draw_on(&self, ui: &mut Ui) {
        let screen_size: Vec2 = Vec2::new(390., 80.);

        Group::new(hash!(&self.description), screen_size).ui(ui, |ui| {
            ui.label(None, &format!(" # {}", &self.title));

            for line in Law::split(ui, &self.description, screen_size.x) {
                ui.label(None, &line);
            }
        });
    }

    pub fn split(ui: &mut Ui, text: &str, maxwidth: f32) -> Vec<String> {
        let mut strings = Vec::new();
        let mut currentString = String::new();

        for word in text.split_whitespace() {
            let newSize = measure_text(&format!("{currentString} {word}"), None, 14, 1.0).width;
            if (newSize >= maxwidth - 50.0) {
                strings.push(currentString);
                currentString = String::new();
            }
            if !currentString.is_empty() {
                currentString.push_str(" ");
            }
            currentString.push_str(word);
            
        }
        strings.push(currentString);
        strings
    }



    
}

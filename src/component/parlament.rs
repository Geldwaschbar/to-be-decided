use crate::{
    component::{Component, wrap},
    effect::Effect,
};
use macroquad::prelude::*;
use macroquad::ui::{Ui, hash, widgets::Group};
use serde::Deserialize;
use std::{cmp::Ordering, collections::VecDeque, f64::consts::PI, rc::Rc};

// TODO: increase voting time
const VOTING_TIME: f32 = 10.;

#[derive(Debug)]
pub struct Party {
    pub approval: f32,
    pub popularity: f32,
    pub color: Color,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Law {
    pub title: String,
    /// The description of this law.
    pub description: String,
    /// How much approval do you need from a party to get there votes?
    pub required_approval: f32,
    /// The publicity describes how likly it is for the parlament to decide upon
    /// this law.
    #[serde(default)]
    pub publicity: f32,
    /// All effects that are triggered when it gets passed.
    #[serde(default)]
    pub on_self_passed: Vec<Rc<Effect>>,
    /// All effects that are triggered whenever this or any other law gets passed.
    #[serde(default)]
    pub on_law_passed: Vec<Rc<Effect>>,
    /// Whether or not this law can be passed multiple times.
    #[serde(default)]
    pub recurring: bool,
}

impl Law {
    pub fn draw_on(&mut self, ui: &mut Ui) {
        let screen_size: Vec2 = Vec2::new(390., 180.);

        Group::new(hash!(&self.description), screen_size).ui(ui, |ui| {
            ui.label(None, &format!(" # {}", &self.title));

            for line in wrap(&self.description, screen_size.x) {
                ui.label(None, &line);
            }

            ui.label(
                None,
                &format!("Sichbarkeit in der Bevölkerung: {}", self.publicity),
            );
            ui.separator();
            ui.same_line(100.);
            if ui.button(None, "Lobbyieren") {
                self.publicity += 1.0;
            }
            ui.same_line(180.);
            if ui.button(None, "Verleumden") {
                self.publicity -= 1.0;
            }
        });
    }
}

pub struct Parlament {
    pub parties: Vec<Party>,
    pub voting_progress: f32,
    pub available_laws: VecDeque<Rc<Law>>,
    pub passed_laws: VecDeque<Rc<Law>>,
}

impl Parlament {
    pub async fn new() -> Parlament {
        let parties = vec![
            Party {
                approval: 0.34,
                popularity: 0.35,
                color: color_u8!(220, 20, 60, 255),
            },
            Party {
                approval: 0.22,
                popularity: 0.40,
                color: color_u8!(22, 163, 62, 255),
            },
            Party {
                approval: 0.19,
                popularity: 0.25,
                color: color_u8!(20, 54, 158, 255),
            },
            Party {
                approval: 1.0,
                popularity: 0.0,
                color: YELLOW,
            },
        ];

        let available_laws: VecDeque<Rc<Law>> = {
            let serialized = load_string("assets/laws.json").await.unwrap();
            serde_json::from_str(&serialized).unwrap()
        };

        let passed_laws: VecDeque<Rc<Law>> = VecDeque::new();

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
        let cursor = Vec2::new(screen_width()*0.5 - 190.,screen_height()*0.5 - 190.);

        const TOTAL_SEATS: f32 = (5 * 4 + 4 * 3) as f32;
        let window_center = Vec2::new(380., 380.) * 0.5;
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
                        window_center.x + cursor.x - angle.cos() * 40. * (row + 5 - base) as f32,
                        window_center.y + cursor.y - angle.sin() * 40. * (row + 5 - base) as f32,
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

        const BAR_WIDTH: f32 = 380.;
        canvas.rect(
            Rect::new(
                window_center.x - BAR_WIDTH * 0.5 + cursor.x + 8.,
                cursor.y + window_center.y + 60.,
                BAR_WIDTH,
                15.,
            ),
            Color::new(0.2, 0.2, 0.2, 1.0),
            color_u8!(50, 50, 50, 255),
        );
        let progress = self.voting_progress;
        canvas.rect(
            Rect::new(
                window_center.x - BAR_WIDTH * 0.5 + cursor.x + 9.,
                cursor.y + window_center.y + 61.,
                progress * BAR_WIDTH - 2.,
                15. - 2.,
            ),
            Color::new(0.2, 0.2, 0.2, 1.0),
            color_u8!(119, 0, 247, 255),
        );

        {
            let text = "Es wird über das nächste Gesetz abgestimmt.";
            let size = measure_text(text, None, 14, 1.);
            ui.label(
                Vec2::new(cursor.x + window_center.x - size.width * 0.5, cursor.y + window_center.y + 30.),
                text,
            );
        }
        let law = self.available_laws.front().expect("expected law exists");
        {
            let text = &format!("Es wird über \"{}\" abgestimmt.", law.title);
            let size = measure_text(text, None, 14, 1.);
            ui.label(
                Vec2::new(cursor.x + window_center.x - size.width * 0.5, cursor.y + window_center.x + 90.),
                text,
            );
        }
        {
            let mut approval = 0.;
            for party in &self.parties {
                if party.approval >= law.required_approval {
                    approval += party.popularity;
                }
            }
            let text = &format!(
                "Die Zustimmung für dieses Gesetz beträgt {}%.",
                (approval * 100.) as usize
            );
            let size = measure_text(text, None, 14, 1.);
            ui.label(
                Vec2::new(cursor.x + window_center.x - size.width * 0.5, cursor.y + window_center.y + 110.),
                text,
            );
        }

        Rc::make_mut(self.available_laws.front_mut().expect("law exists")).publicity =
            f32::INFINITY;
    }

    fn update(&mut self, effects: &mut Vec<Rc<Effect>>) {
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
                // The law was passed
                for effect in &law.on_self_passed {
                    effects.push(effect.clone());
                }
                let (available, passed) = (law.recurring, !law.on_law_passed.is_empty());
                if available {
                    // Reset publicity of the passed law back to 0.
                    let mut new = law.clone();
                    Rc::make_mut(&mut new).publicity = 0.;
                    self.available_laws.push_back(new);
                }
                if passed {
                    let law = self.available_laws.front().expect("expected law exists");
                    self.passed_laws.push_back(law.clone());
                }
            } else {
                // The law was not passed
                let mut new = law.clone();
                Rc::make_mut(&mut new).publicity = 0.;
                self.available_laws.push_back(new);
            }
            self.available_laws.pop_front();
            self.voting_progress -= 1.0;

            for law in &self.passed_laws {
                for effect in &law.on_law_passed {
                    effects.push(effect.clone());
                }
            }
            for law in &mut self.available_laws {
                Rc::make_mut(law).publicity += 1.;
            }
            self.available_laws.make_contiguous().sort_by(|a, b| {
                if a.publicity < b.publicity {
                    Ordering::Greater
                } else if a.publicity > b.publicity {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });
        }
    }
}

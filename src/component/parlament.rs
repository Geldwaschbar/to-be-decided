use crate::{
    component::{Component, news::Event},
    effect::Effect,
};
use macroquad::prelude::*;
use macroquad::ui::{Ui, hash, widgets::Group};
use serde::Deserialize;
use std::{collections::VecDeque, f64::consts::PI};

const VOTING_TIME: f32 = 20.;

#[derive(Debug)]
pub struct Party {
    pub approval: f32,
    pub popularity: f32,
    pub color: Color,
}

#[derive(Debug)]
pub struct Parlament {
    pub parties: Vec<Party>,
    pub voting_time: f32,
    pub available_laws: VecDeque<Law>,
    pub passed_laws: VecDeque<Law>,
}

impl Parlament {
    pub fn update(&mut self, news: &mut VecDeque<Event>) {
        let progress = self.voting_time / VOTING_TIME;
        if progress >= 1. {
            let law = self.available_laws.front().expect("expected law exists");
            let mut votes = 0.;
            for party in &self.parties {
                if party.approval >= law.required_approval {
                    votes += party.popularity;
                }
            }
            if votes > 0.5 {
                news.push_front(Event::new(
                    "Muchekipchen Watcher".into(),
                    format!("Das Gesetz \"{}\"\nwurde verabschiedet.", law.title),
                ));
                self.available_laws.push_back(law.clone());
                self.available_laws.pop_front();
            } else {
                news.push_front(Event::new(
                    "Muchekipchen Watcher".into(),
                    format!("Das Gesetz \"{}\"\nwurde abgelehnt.", law.title),
                ));
            }
            self.voting_time -= VOTING_TIME;
        }

        self.voting_time += get_frame_time();
    }
}

impl Component for Parlament {
    fn draw_on(&self, ui: &mut Ui) {
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
        let progress = self.voting_time / VOTING_TIME;
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
}

impl Law {
    pub fn draw_on(&self, ui: &mut Ui) {
        Group::new(hash!(&self.description), Vec2::new(390., 80.)).ui(ui, |ui| {
            ui.label(None, &format!(" # {}", &self.title));
            for line in self.description.split('\n') {
                ui.label(None, line);
            }
        });
    }
}

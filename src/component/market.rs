use macroquad::prelude::*;
use macroquad::ui::Ui;
use std::collections::VecDeque;

use crate::{component::Component, effect::Effect};

#[derive(Debug)]
pub struct Market {
    pub money: f32,
    pub price: f32,
    trading_time: f32,
    history: VecDeque<f32>,
}

impl Market {
    pub fn new() -> Market {
        Market {
            money: 100.,
            price: 10.,
            trading_time: 0.,
            history: vec![
                9., 9.2, 8.9, 8.8, 9.1, 9.2, 9.4, 9.6, 9.8, 10., 9., 9.2, 8.9, 8.8, 9.1, 9.2, 9.4,
                9.6, 9.8, 10., 9., 9.2, 8.9, 8.8, 9.1, 9.2, 9.4, 9.6, 9.8, 10.,
            ]
            .into(),
        }
    }
}

impl Component for Market {
    fn draw_on(&mut self, ui: &mut Ui) {
        let mut canvas = ui.canvas();
        let cursor = canvas.cursor();

        // Fetch the history of the stock market and
        // calculate the global minimum and maximum
        let (mut min, mut max) = (1000., 0.);
        for marker in &self.history {
            let value = *marker;
            if value < min {
                min = value
            } else if value > max {
                max = value
            }
        }

        // Draw all stock market lines
        let markers = self.history.len();
        for i in 0..markers - 1 {
            let first = self
                .history
                .get(i)
                .expect("expected first stock market item");
            let second = self
                .history
                .get(i + 1)
                .expect("expected second stock market item");
            canvas.line(
                Vec2::new(
                    cursor.x + (i as f32 / markers as f32) * 180.,
                    cursor.y + 130. - (first - min) / (max - min) * 130.,
                ),
                Vec2::new(
                    cursor.x + ((i + 1) as f32 / markers as f32) * 180.,
                    cursor.y + 130. - (second - min) / (max - min) * 130.,
                ),
                if first <= second { GREEN } else { RED },
            );
        }

        ui.label(Vec2::new(10., 140.), &format!("Money: {}", self.money));
        ui.label(Vec2::new(10., 160.), &format!("Price: {}", self.price));
    }

    fn update(&mut self, _effects: &mut Vec<Effect>) {
        self.trading_time += get_frame_time();
        if self.trading_time >= 1. {
            self.history.push_back(self.price);
            self.price += rand::gen_range(0.0, 2.2) - 1.;
            self.history
                .pop_front()
                .expect("expect history marker exists");
            self.trading_time -= 1.;
        }
    }
}

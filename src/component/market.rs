use crate::{
    component::{Component, limit},
    effect::Effect,
};
use macroquad::prelude::*;
use macroquad::ui::Ui;
use std::{collections::VecDeque, rc::Rc};

#[derive(Debug)]
pub struct Market {
    pub money: f32,
    pub price: f32,
    trading_time: f32,
    income_time: f32,
    history: VecDeque<f32>,
    pub show: bool,
}

impl Market {
    pub fn new() -> Market {
        Market {
            money: 250.,
            price: 10.,
            trading_time: 0.,
            income_time: 0.,
            history: VecDeque::from([
                9., 9.2, 8.9, 8.8, 9.1, 9.2, 9.4, 9.6, 9.8, 10., 9., 9.2, 8.9, 8.8, 9.1, 9.2, 9.4,
                9.6, 9.8, 10., 9., 9.2, 8.9, 8.8, 9.1, 9.2, 9.4, 9.6, 9.8, 10.,
            ]),
            show: false,
        }
    }
}

impl Component for Market {
    fn draw_on(&mut self, ui: &mut Ui, _: &Font) {
        let mut canvas = ui.canvas();
        let cursor = canvas.cursor();
        let window_size = Vec2::new(500., 200.);

        // Fetch the history of the stock market and
        // calculate the global minimum and maximum
        let (mut min, mut max) = (f32::INFINITY, 0.);
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

            let start_pos = Vec2::new(
                cursor.x
                    + 0.125 * window_size.x
                    + (i as f32 / markers as f32) * window_size.x * 0.75,
                cursor.y + 20. + window_size.y - (first - min) / (max - min) * window_size.y,
            );
            let end_pos = Vec2::new(
                cursor.x
                    + 0.125 * window_size.x
                    + ((i + 1) as f32 / markers as f32) * window_size.x * 0.75,
                cursor.y + 20. + window_size.y - (second - min) / (max - min) * window_size.y,
            );

            if first > second {
                canvas.rect(
                    Rect {
                        x: start_pos.x,
                        y: cursor.y + 20.,
                        w: end_pos.x - start_pos.x,
                        h: window_size.y,
                    },
                    Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                        a: 0.,
                    },
                    Color {
                        r: 1.,
                        g: 0.,
                        b: 0.,
                        a: 0.15,
                    },
                );
            };
            canvas.line(
                start_pos,
                end_pos,
                if first <= second { GREEN } else { RED },
            );
        }

        ui.label(
            Vec2::new(10., 240.),
            &format!("Geld: {}$", (self.money * 100.).floor() / 100.),
        );
        ui.label(
            Vec2::new(10., 260.),
            &format!("Preis: {}$", (self.price * 100.).floor() / 100.),
        );
    }

    fn update(&mut self, _effects: &mut Vec<Rc<Effect>>) {
        let frame_time = limit(get_frame_time(), 5.0);
        self.trading_time += frame_time;
        self.income_time += frame_time;
        if self.trading_time >= 1. {
            self.history.push_back(self.price);
            self.price *= rand::gen_range(0.96, 1.05);
            self.history
                .pop_front()
                .expect("expect history marker exists");
            self.trading_time -= 1.;
        }
        if self.income_time >= 24. {
            self.money += self.price * 0.1;
            self.income_time -= 50.;
        }
    }
}

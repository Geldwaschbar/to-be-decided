pub mod botnet;
pub mod market;
pub mod news;
pub mod parlament;

use crate::effect::Effect;
use crate::style::FONT_SIZE;
use macroquad::prelude::*;
use macroquad::ui::Ui;
use std::rc::Rc;

/// A component is a part of the game that can be drawn on the UI only depending on its own data.
pub trait Component {
    /// Draw this component on the screen
    fn draw_on(&mut self, ui: &mut Ui, font: &Font);

    /// Update this component. Effects should be put on the stack.
    fn update(&mut self, effects: &mut Vec<Rc<Effect>>);
}

pub fn wrap(text: &str, max_width: f32, font: &Font) -> Vec<String> {
    let mut lines = Vec::new();
    let mut builder = String::new();

    for word in text.split_whitespace() {
        let new_size = measure_text(&format!("{builder} {word}"), Some(font), FONT_SIZE, 1.0).width;
        if new_size >= max_width - 20.0 {
            lines.push(builder);
            builder = String::new();
        } else if !builder.is_empty() {
            builder.push_str(" ");
        }
        builder.push_str(word);
    }
    lines.push(builder);
    lines
}

pub fn limit(value: f32, max: f32) -> f32 {
    return if value > max { max } else { value };
}

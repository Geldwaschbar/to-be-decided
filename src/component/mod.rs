pub mod market;
pub mod news;
pub mod parlament;

use macroquad::ui::Ui;

use crate::effect::Effect;

/// A component is a part of the game that can be drawn on the UI only depending on its own data.
pub trait Component {
    /// Draw this component on the screen
    fn draw_on(&mut self, ui: &mut Ui);

    /// Update this component. Effects should be put on the stack.
    fn update(&mut self, effects: &mut Vec<Effect>);
}

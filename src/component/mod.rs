pub mod market;
pub mod news;
pub mod parlament;

use macroquad::ui::Ui;

/// A component is a part of the game that can be drawn on the UI only depending on its own data.
pub trait Component {
    fn draw_on(&self, ui: &mut Ui);
}

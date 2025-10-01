use macroquad::prelude::*;
use macroquad::ui::{Skin, Ui};

pub const CRT_FRAGMENT_SHADER: &'static str = include_str!("crt_shader.frag");

pub const CRT_VERTEX_SHADER: &'static str = include_str!("crt_shader.vert");

pub const COL_BACKGROUND: Color = Color {
    r: 0.165,
    g: 0.259,
    b: 0.231,
    a: 1.0,
};

pub fn terminal_skin(ui: &mut Ui) -> Skin {
    let label_style = ui
        .style_builder()
        .text_color(Color::from_rgba(180, 180, 120, 255))
        .font_size(30)
        .build();

    let window_style = ui
        .style_builder()
        .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
        .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
        .build();

    let button_style = ui
        .style_builder()
        .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
        .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
        .text_color(Color::from_rgba(180, 180, 100, 255))
        .font_size(40)
        .build();

    let editbox_style = ui
        .style_builder()
        .background_margin(RectOffset::new(0., 0., 0., 0.))
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .color_selected(Color::from_rgba(190, 190, 190, 255))
        .font_size(50)
        .build();

    Skin {
        editbox_style,
        window_style,
        button_style,
        label_style,
        ..ui.default_skin()
    }
}

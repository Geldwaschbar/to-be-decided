use macroquad::prelude::*;
use macroquad::ui::{Skin, Ui};

pub const CRT_FRAGMENT_SHADER: &'static str = include_str!("crt_shader.frag");

pub const CRT_VERTEX_SHADER: &'static str = include_str!("crt_shader.vert");

pub const COL_BACKGROUND: Color = Color {
    r: 0.15,
    g: 0.15,
    b: 0.15,
    a: 1.0,
};

pub fn terminal_skin(ui: &mut Ui) -> Skin {
    let label_style = ui
        .style_builder()
        .text_color(WHITE)
        .background(Image::gen_image_color(1, 1, COL_BACKGROUND))
        .font_size(14)
        .build();

    let window_style = ui
        .style_builder()
        .background(Image::gen_image_color(1, 1, COL_BACKGROUND))
        .build();

    let window_titlebar_style = ui.style_builder().text_color(WHITE).build();

    let button_style = ui
        .style_builder()
        .text_color(WHITE)
        .background(Image::gen_image_color(1, 1, COL_BACKGROUND))
        .font_size(14)
        .build();

    let editbox_style = ui
        .style_builder()
        .text_color(WHITE)
        .color_selected(GRAY)
        .background(Image::gen_image_color(1, 1, COL_BACKGROUND))
        .font_size(14)
        .build();

    let group_style = ui
        .style_builder()
        .background(Image::gen_image_color(
            1,
            1,
            Color::new(0.18, 0.18, 0.18, 1.0),
        ))
        .font_size(14)
        .build();

    Skin {
        editbox_style,
        window_style,
        window_titlebar_style,
        button_style,
        label_style,
        group_style,
        ..ui.default_skin()
    }
}

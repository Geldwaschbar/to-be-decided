use macroquad::prelude::*;
use macroquad::ui::{Skin, Ui};

//pub const CRT_FRAGMENT_SHADER: &'static str = include_str!("crt_shader.frag");
//
//pub const CRT_VERTEX_SHADER: &'static str = include_str!("crt_shader.vert");

pub const COL_BG: Color = Color { r: 0., g: 0., b: 0., a: 1.0, };
pub const COL_BG_ALT: Color = Color { r: 0.0, g: 0.3, b: 0.4, a: 1.0, };
pub const COL_MAIN: Color = Color { r: 0., g: 1., b: 0., a: 1.0, };
pub const COL_SECONDARY: Color = Color { r: 0., g: 0.3, b: 1., a: 1.0, };

pub const FONT_SIZE: u16 = 10;

pub fn terminal_skin(ui: &mut Ui, font: &Font) -> Skin {
    let label_style = ui
        .style_builder()
        .text_color(COL_MAIN)
        .background(Image::gen_image_color(1, 1, COL_BG))
        .font_size(FONT_SIZE)
        .with_font(font)
        .unwrap()
        .build();

    let window_style = ui
        .style_builder()
        .background(Image::gen_image_color(1, 1, COL_BG))
        .color(Color { r: 0., g: 0., b: 0., a: 0. })
        .color_inactive(RED)
        .build();

    let window_titlebar_style = ui
        .style_builder()
        .text_color(COL_MAIN)
        .color(COL_SECONDARY)
        .with_font(font).unwrap()
        .font_size(FONT_SIZE + 2)
        .build();

    let button_style = ui
        .style_builder()
        .margin(RectOffset::new(5.0, 5.0, 5.0, 5.0))
        .text_color(COL_SECONDARY)
        .text_color_hovered(SKYBLUE)
        .background(Image::from_file_with_format(include_bytes!("../../assets/sprites/button.png"), None).unwrap())
        .color(COL_BG_ALT)
        .with_font(font).unwrap()
        .font_size(FONT_SIZE)
        .build();

    let editbox_style = ui
        .style_builder()
        .text_color(WHITE)
        .color_selected(GRAY)
        .background(Image::gen_image_color(1, 1, COL_BG))
        .font_size(FONT_SIZE)
        .build();

    let group_style = ui
        .style_builder()
        .background(Image::gen_image_color(
            1,
            1,
            Color::new(0.18, 0.18, 0.18, 1.0),
        ))
        .font_size(FONT_SIZE)
        .color(COL_SECONDARY)
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

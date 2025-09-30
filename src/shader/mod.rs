use macroquad::prelude::Color;

pub const CRT_FRAGMENT_SHADER: &'static str = include_str!("crt_shader.frag");

pub const CRT_VERTEX_SHADER: &'static str = include_str!("crt_shader.vert");

pub const COL_BACKGROUND: Color = Color { r: 0.165, g: 0.259, b: 0.231, a: 1.0 };
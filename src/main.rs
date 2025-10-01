mod component;
mod effect;
mod shader;

use crate::{
    component::{Component, market::Market, news::News, parlament::Parlament},
    shader::{COL_BACKGROUND, CRT_FRAGMENT_SHADER, CRT_VERTEX_SHADER},
};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets::Window};

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad Template"),
        high_dpi: true,
        #[cfg(target_arch = "wasm32")]
        platform: miniquad::conf::Platform {
            webgl_version: miniquad::conf::WebGLVersion::WebGL2,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);

    let material = load_material(
        ShaderSource::Glsl {
            vertex: CRT_VERTEX_SHADER,
            fragment: CRT_FRAGMENT_SHADER,
        },
        Default::default(),
    )
    .unwrap();

    let mut market = Market::new();
    let mut parlament = Parlament::new().await;
    let mut news = News::new().await;

    let mut effects = Vec::new();

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        market.update(&mut effects);
        parlament.update(&mut effects);
        news.update(&mut effects);
        for effect in &mut effects {
            effect.resolve(&mut market, &mut parlament, &mut news);
        }
        effects.clear();

        set_camera(&Camera2D {
            render_target: Some(render_target.clone()),
            ..Default::default()
        });
        gl_use_material(&material);
        clear_background(COL_BACKGROUND);

        Window::new(hash!(), Vec2::new(30., 50.), Vec2::new(200., 220.))
            .label("Stock Market")
            .ui(&mut *root_ui(), |ui| {
                market.draw_on(ui);
            });

        Window::new(hash!(), Vec2::new(110., 80.), Vec2::new(400., 400.))
            .label("Parlament")
            .ui(&mut *root_ui(), |ui| {
                parlament.draw_on(ui);
            });

        Window::new(hash!(), Vec2::new(100., 400.), Vec2::new(400., 200.))
            .label("Laws")
            .ui(&mut *root_ui(), |ui| {
                for law in &parlament.available_laws {
                    law.draw_on(ui);
                }
            });

        Window::new(hash!(), Vec2::new(480., 50.), Vec2::new(300., 500.))
            .label("News")
            .ui(&mut *root_ui(), |ui| {
                news.draw_on(ui);
            });

        set_default_camera();
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        next_frame().await;
    }
}

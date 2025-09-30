mod event;
mod law;
mod market;
mod modifier;
mod shader;

use crate::{
    event::Event,
    law::{Law, Parlament, Party},
    market::Market,
    shader::{CRT_FRAGMENT_SHADER, CRT_VERTEX_SHADER},
};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets::Window};
use std::collections::VecDeque;

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

    let events: Vec<Event> = {
        let serialized = load_string("assets/events.json").await.unwrap();
        serde_json::from_str(&serialized).unwrap()
    };

    let mut parlament = {
        let parties = vec![
            Party {
                approval: 0.34,
                popularity: 0.45,
                color: RED,
            },
            Party {
                approval: 0.82,
                popularity: 0.35,
                color: GREEN,
            },
            Party {
                approval: 0.82,
                popularity: 0.2,
                color: BLUE,
            },
        ];

        let available_laws: VecDeque<Law> = {
            let serialized = load_string("assets/laws.json").await.unwrap();
            serde_json::from_str(&serialized).unwrap()
        };

        let passed_laws: VecDeque<Law> = VecDeque::new();

        Parlament {
            parties,
            available_laws,
            passed_laws,
            voting_time: 0.,
        }
    };

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        set_camera(&Camera2D {
            render_target: Some(render_target.clone()),
            ..Default::default()
        });
        gl_use_material(&material);
        clear_background(WHITE);

        Window::new(hash!(), Vec2::new(30., 50.), Vec2::new(200., 220.))
            .label("Stock Market")
            .ui(&mut *root_ui(), |ui| {
                market.update();
                market.draw_on(ui);
            });

        Window::new(hash!(), Vec2::new(110., 80.), Vec2::new(400., 400.))
            .label("Parlament")
            .ui(&mut *root_ui(), |ui| {
                parlament.update();
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
                for event in &events {
                    event.draw_on(ui);
                }
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

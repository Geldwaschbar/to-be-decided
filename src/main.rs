mod event;
mod law;
mod modifier;
mod player;
mod shader;

use crate::{
    event::Event,
    law::{Law, Party},
    modifier::{ModType, Modifier, Resource},
    player::Player,
    shader::{CRT_FRAGMENT_SHADER, CRT_VERTEX_SHADER},
};
use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{Group, Window},
};
use std::f64::consts::PI;

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

    let mut player = Player::new();

    let events: Vec<Event> = {
        let serialized = load_string("assets/events.json").await.unwrap();
        serde_json::from_str(&serialized).unwrap()
    };

    let laws: Vec<Law> = {
        let serialized = load_string("assets/laws.json").await.unwrap();
        serde_json::from_str(&serialized).unwrap()
    };

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

    let stock_market = vec![
        Vec2::new(0., 10.),
        Vec2::new(1., 30.),
        Vec2::new(2., 20.),
        Vec2::new(3., 50.),
        Vec2::new(4., 20.),
        Vec2::new(5., 60.),
        Vec2::new(5.5, 10.),
        Vec2::new(5.9, 30.),
        Vec2::new(6.4, 20.),
        Vec2::new(6.9, 50.),
        Vec2::new(7.5, 20.),
        Vec2::new(9., 60.),
    ];

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

        Window::new(hash!(), Vec2::new(30., 50.), Vec2::new(200., 180.))
            .label("Stock Market")
            .ui(&mut *root_ui(), |ui| {
                let mut canvas = ui.canvas();
                let cursor = canvas.cursor();

                let (mut min_x, mut max_x, mut min_y, mut max_y) = (0., 0., 0., 0.);
                for marker in &stock_market {
                    if marker.x < min_x {
                        min_x = marker.x
                    } else if marker.x > max_x {
                        max_x = marker.x
                    }
                    if marker.y < min_y {
                        min_y = marker.y
                    } else if marker.y > max_y {
                        max_y = marker.y
                    }
                }
                let (min, max) = (
                    Vec2::new(min_x, min_y),
                    Vec2::new(max_x / 200., max_y / 130.),
                );
                for i in 0..stock_market.len() - 1 {
                    let first = stock_market
                        .get(i)
                        .expect("expected first stock market item");
                    let second = stock_market
                        .get(i + 1)
                        .expect("expected second stock market item");
                    canvas.line(
                        Vec2::new(
                            cursor.x + (first.x - min.x) / max.x,
                            cursor.y + 130. - (first.y - min.y) / max.y,
                        ),
                        Vec2::new(
                            cursor.x + (second.x - min.x) / max.x,
                            cursor.y + 130. - (second.y - min.y) / max.y,
                        ),
                        if first.y <= second.y { GREEN } else { RED },
                    );
                }

                if ui.button(Vec2::new(10., 140.), "Buy") {}
                if ui.button(Vec2::new(50., 140.), "Sell") {}
            });

        Window::new(hash!(), Vec2::new(110., 80.), Vec2::new(400., 400.))
            .label("Parlament")
            .ui(&mut *root_ui(), |ui| {
                let mut canvas = ui.canvas();
                let cursor = canvas.cursor();

                const TOTAL_SEATS: f32 = (9 * 4) as f32;
                // dbg!(&TOTAL_SEATS);
                let mut placed = 0.;
                let mut party_num = 0;
                for arc in 0..9 {
                    let base = if arc % 2 == 0 { 4 } else { 3 };
                    for row in 0..base {
                        let party = parties.get(party_num).expect("expect party exists");
                        let angle = arc as f32 / 8. * PI as f32;
                        canvas.rect(
                            Rect::new(
                                200. + cursor.x - angle.cos() * 40. * (row + 5 - base) as f32,
                                200. + cursor.y - angle.sin() * 40. * (row + 5 - base) as f32,
                                15.0,
                                15.0,
                            ),
                            Color::new(0.2, 0.2, 0.2, 1.0),
                            party.color,
                        );
                        placed += (1.0 / party.popularity) / TOTAL_SEATS;
                        if placed >= 1. {
                            placed = 0.;
                            party_num += 1;
                        }
                    }
                }
            });

        Window::new(hash!(), Vec2::new(100., 400.), Vec2::new(400., 200.))
            .label("Laws")
            .ui(&mut *root_ui(), |ui| {
                for law in &laws {
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

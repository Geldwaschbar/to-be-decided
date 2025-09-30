mod event;
mod law;
mod modifier;
mod player;
mod shader;

use crate::{
    event::Event,
    law::Party,
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
    let serialized = load_string("assets/events.json").await.unwrap();
    let events: Vec<Event> = serde_json::from_str(&serialized).unwrap();

    let mut parties = vec![
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

        Window::new(hash!(), Vec2::new(30., 80.), Vec2::new(400., 400.))
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

        Window::new(hash!(), Vec2::new(50., 250.), Vec2::new(320., 200.))
            .label("Shop")
            .ui(&mut *root_ui(), |ui| {
                for i in 0..10 {
                    Group::new(hash!("shop", i), Vec2::new(300., 80.)).ui(ui, |ui| {
                        ui.label(Vec2::new(10., 10.), &format!("Item N {}", i));
                        ui.label(Vec2::new(260., 40.), "10/10");
                        ui.label(Vec2::new(200., 58.), &format!("{} kr", 800));
                        if ui.button(Vec2::new(260., 55.), "buy") && player.can_buy(800.) {
                            dbg!("buyed item");
                        }
                    });
                }
            });

        Window::new(hash!(), Vec2::new(400., 50.), Vec2::new(300., 500.))
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

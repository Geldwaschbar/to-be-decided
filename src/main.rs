mod component;
mod effect;
mod state;
mod style;

use crate::{
    component::{Component, botnet::Botnet, market::Market, news::News, parlament::Parlament},
    state::GameState,
    style::{COL_BG, terminal_skin},
};
use macroquad::prelude::*;
use macroquad::{
    audio::{PlaySoundParams, load_sound_from_bytes, play_sound, stop_sound},
    ui::{hash, root_ui, widgets::Window},
};
use std::rc::Rc;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad Template"),
        window_width: 1920,
        window_height: 1080,
        high_dpi: false,
        fullscreen: true,
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
    // Font and Skin
    let font: Font = load_ttf_font("./assets/fonts/Mx437_HP_100LX_16x12.ttf")
        .await
        .unwrap();
    let skin = terminal_skin(&mut *root_ui(), &font);
    root_ui().push_skin(&skin);

    // Sound
    let startup_sound = load_sound_from_bytes(include_bytes!("../assets/audio/startup.wav"))
        .await
        .ok()
        .unwrap();
    let loop_sound = load_sound_from_bytes(include_bytes!("../assets/audio/loop.wav"))
        .await
        .ok()
        .unwrap();
    let action_ok = load_sound_from_bytes(include_bytes!("../assets/audio/action_ok.wav"))
        .await
        .ok()
        .unwrap();
    let action_err = load_sound_from_bytes(include_bytes!("../assets/audio/action_err.wav"))
        .await
        .ok()
        .unwrap();

    // Screens
    let start_screen = load_texture("assets/screens/start_scr.png").await.unwrap();
    start_screen.set_filter(FilterMode::Nearest);
    let win_screen = load_texture("assets/screens/win_scr.png").await.unwrap();
    start_screen.set_filter(FilterMode::Nearest);
    let lost_screen = load_texture("assets/screens/lost_scr.png").await.unwrap();
    start_screen.set_filter(FilterMode::Nearest);
    play_sound(
        &startup_sound,
        PlaySoundParams {
            looped: true,
            volume: 1.0,
        },
    );

    let mut botnet = Botnet::new().await;
    let mut market = Market::new();
    let mut parlament = Parlament::new().await;
    let mut news = News::new().await;

    let mut state = GameState::Starting;
    let mut effects = Vec::new();

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if (is_key_down(KeyCode::LeftControl) && is_key_down(KeyCode::Q))
            || is_key_down(KeyCode::Escape)
        {
            break;
        }

        if state == GameState::Starting {
            draw_texture_ex(
                &start_screen,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
            let mouse_keys_pressd: bool = [MouseButton::Right, MouseButton::Left]
                .map(is_mouse_button_down)
                .contains(&true);

            if !get_keys_pressed().is_empty() | mouse_keys_pressd {
                state = GameState::Running;
                stop_sound(&startup_sound);
                play_sound(
                    &loop_sound,
                    PlaySoundParams {
                        looped: true,
                        volume: 1.0,
                    },
                );
            }
        } else if state == GameState::Running {
            if botnet.show {
                botnet.update(&mut effects);
            }
            if market.show {
                market.update(&mut effects);
            }
            parlament.update(&mut effects);
            news.update(&mut effects);
            for effect in &mut effects {
                effect.resolve(
                    &mut botnet,
                    &mut market,
                    &mut parlament,
                    &mut news,
                    &mut state,
                );
            }
            effects.clear();

            clear_background(COL_BG);

            if market.show {
                let market_id = hash!();
                Window::new(market_id, Vec2::new(30., 50.), Vec2::new(500., 400.))
                    .label("Evil Inc. Stocks")
                    .movable(false)
                    .ui(&mut *root_ui(), |ui| {
                        ui.move_window(market_id, Vec2::new(30., 50.));
                        market.draw_on(ui, &font);
                    });
            }

            if botnet.show {
                let botnet_id = hash!();
                Window::new(
                    botnet_id,
                    Vec2::new(30., screen_height() * 0.5 - 125.),
                    Vec2::new(550., 500.),
                )
                .movable(false)
                .label("Botnet")
                .ui(&mut *root_ui(), |ui| {
                    ui.move_window(botnet_id, Vec2::new(30., screen_height() * 0.5 - 125.));
                    botnet.draw_on(ui, &font);
                });
            }

            parlament.draw_on(&mut *root_ui(), &font);

            let laws_id = hash!();
            Window::new(
                laws_id,
                Vec2::new(screen_width() * 0.5 - 300., screen_height() * 0.5 + 200.),
                Vec2::new(600., 1080. * 0.5 - 200.),
            )
            .movable(false)
            .label("Gesetze")
            .ui(&mut *root_ui(), |ui| {
                ui.move_window(
                    laws_id,
                    Vec2::new(screen_width() * 0.5 - 300., screen_height() * 0.5 + 200.),
                );
                let mut law_pos = Vec2::new(5.0, 5.0);
                for law in &mut parlament.available_laws {
                    if law.publicity > 0.0 {
                        Rc::make_mut(law).draw_on(
                            ui,
                            &font,
                            &mut law_pos,
                            &mut market,
                            &action_ok,
                            &action_err,
                        );
                    }
                }
            });

            let news_id = hash!();
            Window::new(
                news_id,
                Vec2::new(screen_width() * 0.85 - 200., 50.),
                Vec2::new(400., screen_height() - 50.),
            )
            .movable(false)
            .label("Neuigkeiten")
            .ui(&mut *root_ui(), |ui| {
                ui.move_window(news_id, Vec2::new(screen_width() * 0.85 - 200., 50.));
                news.draw_on(ui, &font);
            });
        } else if state == GameState::Won {
            draw_texture_ex(
                &win_screen,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
            if !get_keys_pressed().is_empty() {
                state = GameState::Running;

                // Reset all entities
                botnet = Botnet::new().await;
                market = Market::new();
                parlament = Parlament::new().await;
                news = News::new().await;
            }
        } else {
            draw_texture_ex(
                &lost_screen,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
            if !get_keys_pressed().is_empty() {
                state = GameState::Running;

                // Reset all entities
                botnet = Botnet::new().await;
                market = Market::new();
                parlament = Parlament::new().await;
                news = News::new().await;
            }
        }

        next_frame().await;
    }
}

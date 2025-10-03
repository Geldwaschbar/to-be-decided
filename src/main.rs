mod component;
mod effect;
mod shader;

use crate::{
    component::{Component, botnet::Botnet, market::Market, news::News, parlament::Parlament},
    shader::{COL_BG, terminal_skin},
};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets::Window};
use std::rc::Rc;

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
    let font: Font = load_ttf_font("./assets/fonts/Mx437_HP_100LX_16x12.ttf")
        .await
        .unwrap();
    let skin = terminal_skin(&mut *root_ui(), &font);
    root_ui().push_skin(&skin);

    let mut botnet = Botnet::new();
    let mut market = Market::new();
    let mut parlament = Parlament::new().await;
    let mut news = News::new().await;

    let mut effects = Vec::new();
    set_fullscreen(true);

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        botnet.update(&mut effects);
        market.update(&mut effects);
        parlament.update(&mut effects);
        news.update(&mut effects);
        for effect in &mut effects {
            effect.resolve(&mut market, &mut parlament, &mut news);
        }
        effects.clear();

        clear_background(COL_BG);
        Window::new(hash!(), Vec2::new(30., 50.), Vec2::new(250., 220.))
            .label("Evil Inc. Stocks")
            .ui(&mut *root_ui(), |ui| {
                market.draw_on(ui, &font);
            });

        Window::new(
            hash!(),
            Vec2::new(30., screen_height() - 125.),
            Vec2::new(250., 300.),
        )
        .label("Botnet")
        .ui(&mut *root_ui(), |ui| {
            botnet.draw_on(ui, &font);
        });

        parlament.draw_on(&mut *root_ui(), &font);

        Window::new(
            hash!(),
            Vec2::new(screen_width() * 0.5 - 300., screen_height() * 0.5 + 200.),
            Vec2::new(600., screen_height() * 0.5 - 200.),
        )
        .movable(false)
        .label("Gesetze")
        .ui(&mut *root_ui(), |ui| {
            let mut law_pos = Vec2::new(5.0, 5.0);
            for law in &mut parlament.available_laws {
                Rc::make_mut(law).draw_on(ui, &font, &mut law_pos);
            }
        });

        Window::new(
            hash!(),
            Vec2::new(screen_width(), 50.),
            Vec2::new(400., 500.),
        )
        .label("Neuigkeiten")
        .ui(&mut *root_ui(), |ui| {
            news.draw_on(ui, &font);
        });

        next_frame().await;
    }
}

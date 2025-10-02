mod component;
mod effect;
mod shader;

use crate::{
    component::{Component, botnet::Botnet, market::Market, news::News, parlament::Parlament},
    shader::{COL_BG, CRT_FRAGMENT_SHADER, CRT_VERTEX_SHADER, terminal_skin},
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
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);

    //let material = load_material(
    //    ShaderSource::Glsl {
    //        vertex: CRT_VERTEX_SHADER,
    //        fragment: CRT_FRAGMENT_SHADER,
    //    },
    //    Default::default(),
    //)
    //.unwrap();
    let font : Font = load_ttf_font("./assets/fonts/Mx437_HP_100LX_16x12.ttf").await.unwrap();
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

        set_camera(&Camera2D {
            render_target: Some(render_target.clone()),
            ..Default::default()
        });
        //gl_use_material(&material);
        clear_background(COL_BG);

        Window::new(hash!(), Vec2::new(30., 50.), Vec2::new(200., 220.))
            .label("Evil Inc. Stocks")
            .ui(&mut *root_ui(), |ui| {
                market.draw_on(ui, &font);
            });

        Window::new(hash!(), Vec2::new(30., 200.), Vec2::new(200., 250.))
            .label("Botnet")
            .ui(&mut *root_ui(), |ui| {
                botnet.draw_on(ui, &font);
            });

        parlament.draw_on(&mut *root_ui(), &font);

        Window::new(hash!(), Vec2::new(screen_width()*0.5 - 300., 
            screen_height()*0.5 + 200.), 
            Vec2::new(600., screen_height()*0.5 - 200.))
            .movable(false)
            .label("Gesetze")
            .ui(&mut *root_ui(), |ui| {
                for law in &mut parlament.available_laws {
                    Rc::make_mut(law).draw_on(ui, &font);
                }
            });

        Window::new(hash!(), Vec2::new(480., 50.), Vec2::new(300., 500.))
            .label("Neuigkeiten")
            .ui(&mut *root_ui(), |ui| {
                news.draw_on(ui, &font);
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

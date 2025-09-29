mod player;
mod shader;

use crate::player::{Mail, Player};
use crate::shader::{CRT_FRAGMENT_SHADER, CRT_VERTEX_SHADER};
use macroquad::prelude::*;
use macroquad::ui::{
    Skin, hash, root_ui,
    widgets::{Group, Window},
};

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

    let skin = {
        let label_style = root_ui()
            .style_builder()
            .font_size(18)
            .margin(RectOffset::new(2., 2., 2., 2.))
            .build();

        Skin {
            label_style,
            ..root_ui().default_skin()
        }
    };

    let mut player = Player::new();
    let serialized = load_string("assets/mails.json").await.unwrap();
    let mails: Vec<Mail> = serde_json::from_str(&serialized).unwrap();
    for mail in mails {
        player.add_mail(mail);
    }

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

        Window::new(hash!(), Vec2::new(400., 50.), Vec2::new(320., 400.))
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

        Window::new(hash!(), Vec2::new(50., 50.), Vec2::new(300., 500.))
            .label("E-Mails")
            .ui(&mut *root_ui(), |ui| {
                for mail in player.fetch_mails() {
                    Group::new(hash!(mail.get_message()), Vec2::new(292., 80.)).ui(ui, |ui| {
                        ui.push_skin(&skin);
                        ui.label(None, &format!("from: {}", mail.get_author()));
                        ui.pop_skin();
                        for line in mail.get_message().split('\n') {
                            ui.label(None, line);
                        }
                    });
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

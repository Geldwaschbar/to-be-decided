mod player;

use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled as tiled;

use crate::player::Player;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad Template"),
        high_dpi: true,
        sample_count: 2,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let tileset = load_texture("assets/tileset.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("assets/map.json").await.unwrap();
    let tiled_map = tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]).unwrap();

    let mut static_colliders = Vec::new();
    for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
        static_colliders.push(if tile.is_some() {
            Tile::Solid
        } else {
            Tile::Empty
        });
    }

    let mut world = World::new();
    world.add_static_tiled_layer(static_colliders, 8., 8., 40, 1);

    let mut player = Player::new();
    player.set_position(Vec2::new(64.0, 64.0));

    let mut camera = Camera2D {
        zoom: Vec2::ONE * 0.005,
        ..Default::default()
    };

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        // Update the player movement based on user input
        player.movement();

        // Clip the camera target to the new player position,
        // so that the camera always focus on the player
        camera.target = *player.get_position();

        // Lets draw the whole map full screen
        let dest_rect = Rect::new(0., 0., screen_width(), screen_height());

        // We used only part of tiled canvas to create our first level
        // So lets draw only that part of the canvas
        // Area is hardcoded for now, but we will use the technique of drawing parts of tiled canvas
        // to jump through level sections in the future
        let source_rect = Rect::new(0., 0., 40., 19.);

        clear_background(BLACK);
        set_camera(&camera);
        tiled_map.draw_tiles("main layer", dest_rect, source_rect);
        player.draw();

        next_frame().await
    }
}

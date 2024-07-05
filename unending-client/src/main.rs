mod connection;
mod draw;

use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Unending Adventure"),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let content = connection::get_random_content();
    let area_name = content.area.name;

    loop {
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(0., 7.5, 25.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw::ground();

        draw::quest_giver(BLUE);

        set_default_camera();
        draw_text(&area_name, 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
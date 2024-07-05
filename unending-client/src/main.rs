mod connection;
mod draw;
mod game;

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
    let mut state = game::State::Normal;
    let mut state_data = game::StateData{ quest: None };

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

        // TODO: need some kind of ray-tracing to be able to go from
        //  a mouse_position() check to which person is aimed at, plus
        //  some kind of connection between person and quest index
        if is_mouse_button_pressed(MouseButton::Right) {
            if state == game::State::Normal {
                state = game::State::Dialogue;
                state_data.quest = Some(content.quests[0].clone());
            } else {
                state = game::State::Normal;
                state_data.quest = None;
            }
        }

        set_default_camera();
        draw_text(&content.area.name, 10.0, 20.0, 30.0, BLACK);
        if state == game::State::Dialogue {
            if let Some(quest) = &state_data.quest {
                draw_text(&quest.giver, 10.0, 45.0, 30.0, BLACK);
                draw_text(&quest.description, 10.0, 65.0, 30.0, BLACK);
                // TODO: need system to wrap text in some reasonable way
            }
        }

        next_frame().await
    }
}
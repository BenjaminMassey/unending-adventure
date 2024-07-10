mod character;
mod connection;
mod events;
mod ui;
mod world;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_event::<events::Popup>()
        .add_systems(Startup, setup)
        .add_systems(Update, events::handle_popup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let content = connection::get_random_content();
    ui::top_left_text(&mut commands, &content.area.name);
    world::create_circular_base(&mut commands, &mut meshes, &mut materials);
    for (i, quest) in content.quests.iter().enumerate() {
        character::create_quest_giver(
            &mut commands,
            &mut meshes,
            &mut materials,
            &Vec3::new(-1.5 + ((i as f32) * 3.0), 0.0, 0.0),
            &character::CharacterDetails::new(
                &quest.giver,
                &quest.description,
            ),
        );
    }
    world::create_main_light(&mut commands, &mut meshes, &mut materials, &Vec3::new(4.0, 8.0, 4.0));
    world::create_main_camera(&mut commands, &mut meshes, &mut materials, &Vec3::new(-2.5, 4.5, 9.0));
}
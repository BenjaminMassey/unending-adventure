mod character;
mod connection;
mod ui;
mod world;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
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
    character::create_quest_giver(&mut commands, &mut meshes, &mut materials, &Vec3::new(0.0, 0.0, 0.0));
    world::create_main_light(&mut commands, &mut meshes, &mut materials, &Vec3::new(4.0, 8.0, 4.0));
    world::create_main_camera(&mut commands, &mut meshes, &mut materials, &Vec3::new(-2.5, 4.5, 9.0));
}
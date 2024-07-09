use crate::events;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Component)]
struct QuestGiver {
    name: String,
    dialogue: String,
}
// TODO: use above struct to attach to character created with below function
// and tie to clicking them with events::Popup
pub fn create_quest_giver(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    base_position: &Vec3,
) {
    create_character(commands, meshes, materials, base_position);
    create_question_mark(commands, meshes, materials, &(*base_position + Vec3::new(0., 1.75, 0.)));
}

fn create_character(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    base_position: &Vec3,
) {
    let cube_pos_sizes: Vec<((f32, f32, f32), (f32, f32, f32))> = vec![
        ((-0.25, 0., 0.), (0.25, 0.5, 0.25)), // left leg
        ((0.25, 0., 0.), (0.25, 0.5, 0.25)), // right leg
        ((0., 0.5, 0.), (0.25, 0.75, 0.25)), // body
        ((-0.25, 0.625, 0.), (0.5, 0.25, 0.25)), // left arm
        ((0.25, 0.625, 0.), (0.5, 0.25, 0.25)), // right arm
        ((0., 1.125, 0.), (0.5, 0.5, 0.375)), // head
    ];
    for (pos, size) in cube_pos_sizes {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(size.0, size.1, size.2)),
            material: materials.add(Color::srgb_u8(0, 255, 255)),
            transform: Transform::from_xyz(
                base_position.x + pos.0,
                base_position.y + pos.1,
                base_position.z + pos.2,
            ),
            ..default()
        });
    }
}

fn test_click(text: &str) {
    info!("{}", text);
}

fn create_question_mark(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    base_position: &Vec3,
) {
    commands.spawn(
        (
            PbrBundle {
                mesh: meshes.add(Sphere::new(0.075)),
                material: materials.add(Color::srgb_u8(255, 255, 0)),
                transform: Transform::from_xyz(base_position.x, base_position.y, base_position.z),
                ..default()
            },
            On::<Pointer<Click>>::send_event::<events::Popup>(),
        )
    );
    let cube_positions: Vec<(f32, f32, f32)> = vec![
        (0., 0.25, 0.), (0., 0.375, 0.), (0.125, 0.5, 0.),
        (0.25, 0.625, 0.), (0.25, 0.75, 0.), (0.125, 0.875, 0.),
        (0., 1., 0.), (-0.125, 0.875, 0.), (-0.25, 0.75, 0.),
    ];
    for pos in cube_positions {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(0.125, 0.125, 0.125)),
            material: materials.add(Color::srgb_u8(255, 255, 0)),
            transform: Transform::from_xyz(
                base_position.x + pos.0,
                base_position.y + pos.1,
                base_position.z + pos.2,
            ),
            ..default()
        });
    }
}
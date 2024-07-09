use bevy::prelude::*;

// TODO: want only one instance of a particular text,
// but this right now will overlap many instances

pub fn top_left_text(commands: &mut Commands, text: &str) {
    generate_text(
        commands, 
        text,
        JustifyText::Left,
        Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }
    );
}

pub fn top_right_text(commands: &mut Commands, text: &str) {
    generate_text(
        commands, 
        text,
        JustifyText::Right,
        Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }
    );
}

fn generate_text(
    commands: &mut Commands,
    text: &str,
    justify: JustifyText,
    style: Style,
) {
    commands.spawn((
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: 24.0,
                ..default()
            },
        )
        .with_text_justify(justify)
        .with_style(style),
    ));
}
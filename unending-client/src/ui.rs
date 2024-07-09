use bevy::prelude::*;

pub fn top_left_text(
    commands: &mut Commands,
    text: &str,
) {
    commands.spawn((
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: 24.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    ));
}
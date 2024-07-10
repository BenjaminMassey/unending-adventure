use bevy::prelude::*;

#[derive(Component)]
pub struct OverlayText;

#[derive(Copy, Clone, PartialEq)]
pub enum TextPositionType {
    TopLeft,
    TopRight,
}
#[derive(Component)]
pub struct TextPosition {
    pub pos_type: TextPositionType,
}
impl TextPosition {
    pub fn new(pos_type: TextPositionType) -> Self {
        Self { pos_type }
    }
}

pub fn top_left_text(commands: &mut Commands, text: &str) {
    commands.spawn(
        (
            TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 24.0,
                    ..default()
                },
            )
            .with_text_justify(JustifyText::Left)
            .with_style(
                Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    max_width: Val::Percent(50.),
                    ..default()
                }
            ),
            OverlayText,
            TextPosition::new(TextPositionType::TopLeft),
        )
    );
}

pub fn top_right_text(commands: &mut Commands, text: &str) {
    commands.spawn(
        (
            TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 24.0,
                    ..default()
                },
            )
            .with_text_justify(JustifyText::Right)
            .with_style(
                Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    max_width: Val::Percent(50.),
                    ..default()
                }
            ),
            OverlayText,
            TextPosition::new(TextPositionType::TopRight),
        )
    );
}
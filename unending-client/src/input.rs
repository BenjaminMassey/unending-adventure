use crate::events;
use crate::ui;

use bevy::prelude::*;

pub fn keyboard_events(
    keys: Res<ButtonInput<KeyCode>>,
    mut clear_text_writer: EventWriter<events::ClearText>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        clear_text_writer.send(
            events::ClearText::new(
                &vec![ui::TextPositionType::TopRight],
            )
        );
    }
}
use crate::character;
use crate::ui;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct Popup {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Click>>> for Popup {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Popup{ entity: event.target }
    }
}

pub fn handle_popup(
    mut commands: Commands,
    mut popups: EventReader<Popup>,
    mut query_quest_giver: Query<(Entity, &character::CharacterDetails), With<character::QuestGiver>>,
    mut query_text: Query<(Entity, &ui::TextPosition), With<ui::OverlayText>>
) {
    for popup in popups.read() {
        for (entity, details) in &query_quest_giver {
            if entity == popup.entity {
                for (entity, text_position) in &query_text {
                    if &text_position.pos_type == &ui::TextPositionType::TopRight {
                        commands.entity(entity).despawn();
                    }
                } // TODO: would greatly prefer to use ClearText event, but the queueing
                  // up and ordering nature of things means that it is hard to control
                ui::top_right_text(
                    &mut commands,
                    &details.dialogue,
                );
            }
        }
    }
}

#[derive(Event)]
pub struct ClearText {
    pub pos_types: Vec<ui::TextPositionType>,
}
impl ClearText {
    pub fn new(types: &[ui::TextPositionType]) -> Self {
        Self { pos_types: types.to_vec() }
    }
}

pub fn handle_clear_text(
    mut commands: Commands,
    mut clear_texts: EventReader<ClearText>,
    mut query_text: Query<(Entity, &ui::TextPosition), With<ui::OverlayText>>,
) {
    for clear_text in clear_texts.read() {
        for (entity, text_position) in &query_text {
            if clear_text.pos_types.contains(&text_position.pos_type) {
                commands.entity(entity).despawn();
            }
        }
    }
}
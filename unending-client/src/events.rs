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
) {
    for popup in popups.read() {
        for (entity, details) in &query_quest_giver {
            if entity == popup.entity {
                ui::top_right_text(
                    &mut commands,
                    &details.dialogue,
                );
            }
        }
    }
}
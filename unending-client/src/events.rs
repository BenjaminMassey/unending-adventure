use crate::ui;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct Popup {
    pub text: String,
}

impl From<ListenerInput<Pointer<Click>>> for Popup {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Popup{ text: "test".to_owned() } // TODO: retrieve text from event.target
    }
}

pub fn handle_popup(mut commands: Commands, mut popups: EventReader<Popup>) {
    for popup in popups.read() {
        ui::top_right_text(
            &mut commands,
            &format!("Popup: {}", &popup.text)
        );
    }
}
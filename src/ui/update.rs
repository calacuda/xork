use super::MainTextBody;
use bevy::prelude::*;
use bevy_simple_text_input::TextInputSubmitEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Event, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpdateMainSectionText(pub String);

pub fn update_main_section(
    mut events: EventReader<UpdateMainSectionText>,
    mut text: Query<&mut Text, With<MainTextBody>>,
) {
    for event in events.read() {
        let new_text = event.0.clone();

        if let Err(e) = text.get_single_mut().map(|mut text| text.0 = new_text) {
            error!("setting main body text resulted in: {e}");
        }
    }
}

pub fn update_tester(
    mut events: EventReader<TextInputSubmitEvent>,
    mut update_event: EventWriter<UpdateMainSectionText>,
) {
    for event in events.read() {
        let cmd = event.value.clone();
        update_event.send(UpdateMainSectionText(cmd));
    }
}

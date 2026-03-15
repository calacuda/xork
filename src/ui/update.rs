use super::{LookTextBody, MainTextBody};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Message, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpdateMainSectionText(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, Message, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpdateLookSectionText(pub String);

pub fn update_main_section(
    mut events: MessageReader<UpdateMainSectionText>,
    mut text: Query<&mut Text, With<MainTextBody>>,
) {
    for event in events.read() {
        let new_text = event.0.clone();

        if let Err(e) = text.single_mut().map(|mut text| text.0 = new_text) {
            error!("setting main body text resulted in: {e}");
        }
    }
}

// pub fn update_tester(
//     mut events: MessageReader<bevy_simple_text_input::TextInputSubmitEvent>,
//     mut update_event: MessageWriter<UpdateMainSectionText>,
// ) {
//     for event in events.read() {
//         let cmd = event.value.clone();
//         update_event.write(UpdateMainSectionText(cmd));
//     }
// }

pub fn update_look_section(
    mut events: MessageReader<UpdateLookSectionText>,
    mut text: Query<&mut Text, With<LookTextBody>>,
) {
    for event in events.read() {
        // warn!("{event:?}");
        let new_text = event.0.clone();

        if let Err(e) = text.single_mut().map(|mut text| text.0 = new_text) {
            error!("setting look text body text resulted in: {e}");
        }
    }
}

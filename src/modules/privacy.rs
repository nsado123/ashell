use super::{Module, OnModulePress};
use crate::{
    app::Message,
    components::icons::{icon, Icons},
    services::{
        privacy::{PrivacyData, PrivacyService},
        ServiceEvent,
    },
};
use iced::{widget::Row, Alignment, Element};

#[derive(Debug, Clone)]
pub enum PrivacyMessage {
    Event(ServiceEvent<PrivacyService>),
}

impl Module for PrivacyData {
    type Data<'a> = ();

    fn view<'a>(&self, _: Self::Data<'a>) -> Option<(Element<Message>, Option<OnModulePress>)> {
        if !self.no_access() {
            Some((
                Row::new()
                    .push_maybe(self.screenshare_access().then(|| icon(Icons::ScreenShare)))
                    .push_maybe(self.webcam_access().then(|| icon(Icons::Webcam)))
                    .push_maybe(self.microphone_access().then(|| icon(Icons::Mic1)))
                    .align_y(Alignment::Center)
                    .spacing(8)
                    .into(),
                None,
            ))
        } else {
            None
        }
    }
}

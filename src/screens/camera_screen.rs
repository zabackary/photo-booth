use crate::camera_feed::{CameraFeed, CameraMessage};
use iced::{
    widget::{container, row},
    Alignment, Element, Length,
};
use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{RequestedFormat, RequestedFormatType},
    Camera,
};

#[derive(Clone)]
pub(crate) struct CameraScreen {
    feed: CameraFeed,
}

impl std::fmt::Debug for CameraScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish()
    }
}

#[derive(Debug, Clone)]
pub enum CameraScreenMessage {
    CameraFeedMessage(CameraMessage),
}

impl Into<super::ScreenMessage> for CameraScreenMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::CameraScreenMessage(self)
    }
}

impl super::Screenish for CameraScreen {
    type Message = CameraScreenMessage;
    fn new() -> (Self, Option<CameraScreenMessage>) {
        let index = nokhwa::utils::CameraIndex::Index(0);
        let requested =
            RequestedFormat::new::<RgbAFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        let mut camera = Camera::new(index, requested).unwrap();
        camera.open_stream().unwrap();
        let (feed, feed_command) = CameraFeed::new(camera, 48.into());
        (
            CameraScreen { feed },
            feed_command.map(CameraScreenMessage::CameraFeedMessage),
        )
    }
    fn update(&mut self, message: CameraScreenMessage) -> iced::Command<CameraScreenMessage> {
        match message {
            CameraScreenMessage::CameraFeedMessage(msg) => self
                .feed
                .update(msg)
                .map(CameraScreenMessage::CameraFeedMessage),
        }
    }
    fn view(&self) -> Element<CameraScreenMessage> {
        let content = row![self.feed.view().width(Length::Fill).height(Length::Fill)]
            .spacing(20)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
    fn subscription(self) -> iced::Subscription<CameraScreenMessage> {
        self.feed
            .subscription()
            .map(CameraScreenMessage::CameraFeedMessage)
    }
}

impl Into<super::Screen> for CameraScreen {
    fn into(self) -> super::Screen {
        super::Screen::CameraScreen(self)
    }
}

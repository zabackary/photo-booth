use crate::camera_feed::{CameraFeed, CameraMessage};
use iced::{
    alignment,
    widget::{button, column, container, row, text},
    window, Alignment, Color, Element, Length,
};
use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{RequestedFormat, RequestedFormatType},
    Camera,
};

#[derive(Clone)]
pub(crate) struct App {
    feed: CameraFeed,
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish()
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    CameraFeedMessage(CameraMessage),
    ExitPressed,
}

impl Into<super::ScreenMessage> for AppMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::AppMessage(self)
    }
}

impl<'a> super::Screenish for App {
    type Message = AppMessage;
    fn new() -> (Self, Option<AppMessage>) {
        let index = nokhwa::utils::CameraIndex::Index(0);
        let requested =
            RequestedFormat::new::<RgbAFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        let mut camera = Camera::new(index, requested).unwrap();
        camera.open_stream().unwrap();
        let (feed, feed_command) = CameraFeed::new(camera, 48.into());
        (
            App { feed },
            feed_command.map(AppMessage::CameraFeedMessage),
        )
    }
    fn update(&mut self, message: AppMessage) -> iced::Command<AppMessage> {
        match message {
            AppMessage::CameraFeedMessage(msg) => {
                self.feed.update(msg).map(AppMessage::CameraFeedMessage)
            }
            AppMessage::ExitPressed => window::close(),
        }
    }
    fn view(&self) -> Element<AppMessage> {
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
}

impl Into<super::Screen> for App {
    fn into(self) -> super::Screen {
        super::Screen::App(self)
    }
}

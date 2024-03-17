use crate::camera_feed::{CameraFeed, CameraMessage};
use iced::{
    widget::{button, container, text, Row},
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

#[derive(Clone)]
pub(crate) struct CameraScreenFlags {
    pub index: nokhwa::utils::CameraIndex,
}

impl std::fmt::Debug for CameraScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish()
    }
}

#[derive(Debug, Clone)]
pub enum CameraScreenMessage {
    CameraFeedMessage(CameraMessage),
    CaptureButtonPressed,
}

impl Into<super::ScreenMessage> for CameraScreenMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::CameraScreenMessage(self)
    }
}

impl super::Screenish for CameraScreen {
    type Message = CameraScreenMessage;
    type Flags = CameraScreenFlags;
    fn new(flags: CameraScreenFlags) -> (Self, Option<CameraScreenMessage>) {
        let requested =
            RequestedFormat::new::<RgbAFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        let mut camera = Camera::new(flags.index, requested).unwrap();
        camera.open_stream().unwrap();
        let (feed, feed_command) = CameraFeed::new(camera, 48.into());
        (
            CameraScreen { feed },
            feed_command.map(CameraScreenMessage::CameraFeedMessage),
        )
    }
    fn update(&mut self, message: CameraScreenMessage) -> iced::Command<super::ScreenMessage> {
        match message {
            CameraScreenMessage::CameraFeedMessage(msg) => self
                .feed
                .update(msg)
                .map(CameraScreenMessage::CameraFeedMessage)
                .map(super::ScreenMessage::CameraScreenMessage),
            CameraScreenMessage::CaptureButtonPressed => iced::Command::perform(async {}, |_| {
                super::transition_to_screen(super::printing_screen::PrintingScreen::new(()))
            }),
        }
    }
    fn view(&self) -> Element<CameraScreenMessage> {
        container(
            Row::new()
                .push(self.feed.view().width(Length::Fill).height(Length::Fill))
                .push(
                    button(text("Take picture"))
                        .on_press(CameraScreenMessage::CaptureButtonPressed),
                )
                .spacing(20)
                .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
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

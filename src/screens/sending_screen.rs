use std::time::Duration;

use iced::{
    widget::{container, text, Column, Space},
    Element, Length,
};
use image::RgbaImage;

use crate::{config::Config, utils::loading_spinners};

#[derive(Debug)]
pub(crate) struct SendingScreen {
    config: Config,
    index: nokhwa::utils::CameraIndex,

    image: RgbaImage,
}

#[derive(Debug, Clone)]
pub enum SendingScreenMessage {
    Tick,
}

#[derive(Debug, Clone)]
pub(crate) struct SendingScreenFlags {
    pub config: Config,
    pub index: nokhwa::utils::CameraIndex,

    pub image: RgbaImage,
}

impl Into<super::ScreenMessage> for SendingScreenMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::SendingScreenMessage(self)
    }
}

impl super::Screenish for SendingScreen {
    type Message = SendingScreenMessage;
    type Flags = SendingScreenFlags;
    fn new(flags: SendingScreenFlags) -> (Self, Option<SendingScreenMessage>) {
        (
            SendingScreen {
                config: flags.config,
                index: flags.index,

                image: flags.image,
            },
            None,
        )
    }

    fn update(&mut self, message: SendingScreenMessage) -> iced::Command<super::ScreenMessage> {
        iced::Command::none()
    }

    fn view(&self) -> Element<SendingScreenMessage> {
        container(
            Column::new()
                .push(text("Emailing your photos to you...").size(46))
                .push(Space::with_height(24))
                .push(
                    loading_spinners::circular::Circular::new()
                        .easing(&loading_spinners::easing::STANDARD)
                        .cycle_duration(Duration::from_millis(2000))
                        .size(86.0)
                        .bar_height(8.0),
                )
                .align_items(iced::Alignment::Center)
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(iced::alignment::Vertical::Center)
        .padding(20)
        .into()
    }

    fn subscription(&self) -> iced::Subscription<SendingScreenMessage> {
        iced::Subscription::none()
    }
}

impl Into<super::Screen> for SendingScreen {
    fn into(self) -> super::Screen {
        super::Screen::SendingScreen(self)
    }
}

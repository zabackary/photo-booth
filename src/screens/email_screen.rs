use iced::{
    widget::{container, image::Handle, text, Column, Image, Row},
    Element, Length,
};
use image::RgbaImage;

use crate::config::Config;

#[derive(Debug)]
pub(crate) struct EmailScreen {
    config: Config,
    index: nokhwa::utils::CameraIndex,

    preview_handle: Handle,
    printable_image: RgbaImage,
}

#[derive(Debug, Clone)]
pub enum EmailScreenMessage {
    Tick,
}

#[derive(Debug, Clone)]
pub(crate) struct EmailScreenFlags {
    pub config: Config,
    pub index: nokhwa::utils::CameraIndex,

    pub preview_handle: Handle,
    pub printable_image: RgbaImage,
}

impl Into<super::ScreenMessage> for EmailScreenMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::EmailScreenMessage(self)
    }
}

impl super::Screenish for EmailScreen {
    type Message = EmailScreenMessage;
    type Flags = EmailScreenFlags;
    fn new(flags: EmailScreenFlags) -> (Self, Option<EmailScreenMessage>) {
        (
            EmailScreen {
                config: flags.config,
                index: flags.index,

                preview_handle: flags.preview_handle,
                printable_image: flags.printable_image,
            },
            None,
        )
    }
    fn update(&mut self, message: EmailScreenMessage) -> iced::Command<EmailScreenMessage> {
        match message {
            EmailScreenMessage::Tick => iced::Command::none(),
        }
    }
    fn view(&self) -> Element<EmailScreenMessage> {
        container(
            Row::new()
                .push(
                    Column::new()
                        .push(text("email screen!").size(46))
                        .align_items(iced::Alignment::Center)
                        .width(Length::Fill),
                )
                .push(Image::new(self.preview_handle.clone()))
                .align_items(iced::Alignment::Center)
                .spacing(24),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
    }

    fn subscription(&self) -> iced::Subscription<EmailScreenMessage> {
        iced::Subscription::none()
    }
}

impl Into<super::Screen> for EmailScreen {
    fn into(self) -> super::Screen {
        super::Screen::EmailScreen(self)
    }
}

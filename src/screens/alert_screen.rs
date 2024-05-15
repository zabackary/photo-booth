use std::time::Duration;

use iced::{
    theme,
    widget::{container, text, Column},
    Element, Length,
};

use crate::{config::Config, utils::container_styles::RoundedBoxContainerStyle};

#[derive(Debug)]
pub(crate) struct AlertScreen {
    config: Config,
    index: nokhwa::utils::CameraIndex,

    alert_title: String,
    alert_content: String,
    timeout: Duration,
}

#[derive(Debug, Clone)]
pub enum AlertScreenMessage {
    TimeoutFinished,
}

#[derive(Debug, Clone)]
pub(crate) struct AlertScreenFlags {
    pub config: Config,
    pub index: nokhwa::utils::CameraIndex,

    pub alert_title: String,
    pub alert_content: String,
    pub timeout: Duration,
}

impl Into<super::ScreenMessage> for AlertScreenMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::AlertScreenMessage(self)
    }
}

impl super::Screenish for AlertScreen {
    type Message = AlertScreenMessage;
    type Flags = AlertScreenFlags;
    fn new(flags: AlertScreenFlags) -> (Self, Option<AlertScreenMessage>) {
        (
            AlertScreen {
                config: flags.config,
                index: flags.index,

                alert_title: flags.alert_title,
                alert_content: flags.alert_content,
                timeout: flags.timeout,
            },
            None,
        )
    }

    fn update(&mut self, message: AlertScreenMessage) -> iced::Command<super::ScreenMessage> {
        match message {
            AlertScreenMessage::TimeoutFinished => {
                let flags = super::camera_screen::CameraScreenFlags {
                    config: self.config.clone(),
                    index: self.index.clone(),
                };
                iced::Command::perform(async {}, |_| {
                    super::ScreenMessage::TransitionToScreen(super::ScreenFlags::CameraScreenFlags(
                        flags,
                    ))
                })
            }
        }
    }

    fn view(&self) -> Element<AlertScreenMessage> {
        container(
            container(
                Column::new()
                    .push(text(&self.alert_title).size(36))
                    .push(text(&self.alert_content).size(26))
                    .align_items(iced::Alignment::Center)
                    .width(Length::Fill),
            )
            .style(theme::Container::Custom(Box::new(
                RoundedBoxContainerStyle {},
            )))
            .max_width(680)
            .padding(24),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .padding(20)
        .into()
    }

    fn subscription(&self) -> iced::Subscription<AlertScreenMessage> {
        // this is a bit of a hack, but to send a message after the delay,
        // we just use a time-based subscription (setIntervalみたい) since the
        // message causes the destruction of this screen anyway.
        iced::time::every(self.timeout).map(|_| AlertScreenMessage::TimeoutFinished)
    }
}

impl Into<super::Screen> for AlertScreen {
    fn into(self) -> super::Screen {
        super::Screen::AlertScreen(self)
    }
}

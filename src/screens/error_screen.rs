use iced::{
    theme,
    widget::{button, container, text, Column, Space},
    Element, Length,
};

use crate::{config::Config, utils::container_styles::RoundedErrorBoxContainerStyle};

#[derive(Debug)]
pub(crate) struct ErrorScreen {
    config: Config,
    index: nokhwa::utils::CameraIndex,

    error_title: String,
    error_content: String,
}

#[derive(Debug, Clone)]
pub enum ErrorScreenMessage {
    OkPressed,
}

#[derive(Debug, Clone)]
pub(crate) struct ErrorScreenFlags {
    pub config: Config,
    pub index: nokhwa::utils::CameraIndex,

    pub error_title: String,
    pub error_content: String,
}

impl Into<super::ScreenMessage> for ErrorScreenMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::ErrorScreenMessage(self)
    }
}

impl super::Screenish for ErrorScreen {
    type Message = ErrorScreenMessage;
    type Flags = ErrorScreenFlags;
    fn new(flags: ErrorScreenFlags) -> (Self, Option<ErrorScreenMessage>) {
        (
            ErrorScreen {
                config: flags.config,
                index: flags.index,

                error_title: flags.error_title,
                error_content: flags.error_content,
            },
            None,
        )
    }

    fn update(&mut self, message: ErrorScreenMessage) -> iced::Command<super::ScreenMessage> {
        match message {
            ErrorScreenMessage::OkPressed => {
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

    fn view(&self) -> Element<ErrorScreenMessage> {
        container(
            container(
                Column::new()
                    .push(text(&self.error_title).size(36))
                    .push(text(&self.error_content).size(26))
                    .push(Space::with_height(18))
                    .push(
                        button(text("Press [Space] to close error"))
                            .style(theme::Button::Destructive)
                            .on_press(ErrorScreenMessage::OkPressed),
                    )
                    .align_items(iced::Alignment::Center)
                    .width(Length::Fill),
            )
            .style(theme::Container::Custom(Box::new(
                RoundedErrorBoxContainerStyle {},
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

    fn subscription(&self) -> iced::Subscription<ErrorScreenMessage> {
        iced::keyboard::on_key_press(|key, _modifiers| match key {
            iced::keyboard::Key::Named(iced::keyboard::key::Named::Space)
            | iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => {
                Some(ErrorScreenMessage::OkPressed)
            }
            _ => None,
        })
    }
}

impl Into<super::Screen> for ErrorScreen {
    fn into(self) -> super::Screen {
        super::Screen::ErrorScreen(self)
    }
}

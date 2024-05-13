use email_address::EmailAddress;
use iced::{
    theme,
    widget::{
        self, button, column, container, image::Handle, scrollable, text, Column, Image, Row,
        TextInput,
    },
    Element, Length,
};
use image::RgbaImage;

use crate::{
    config::Config,
    utils::container_styles::{OutlinedContainerStyle, RoundedBoxContainerStyle},
};

const MAX_RECIPIENT_NUMBER: usize = 4;

#[derive(Debug)]
pub(crate) struct EmailScreen {
    config: Config,
    index: nokhwa::utils::CameraIndex,

    preview_handle: Handle,
    printable_image: RgbaImage,

    email_addresses: Vec<String>,
    current_email_address_valid: bool,
    current_email_address: String,

    has_focused_email_field: bool,
}

#[derive(Debug, Clone)]
pub enum EmailScreenMessage {
    Tick,
    CurrentEmailAddressChanged(String),
    CurrentEmailAddressSubmitted,
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

                email_addresses: Vec::new(),
                current_email_address_valid: false,
                current_email_address: String::new(),
                has_focused_email_field: false,
            },
            Some(EmailScreenMessage::Tick),
        )
    }
    fn update(&mut self, message: EmailScreenMessage) -> iced::Command<super::ScreenMessage> {
        iced::Command::batch([
            match message {
                EmailScreenMessage::Tick => iced::Command::none(),
                EmailScreenMessage::CurrentEmailAddressChanged(new_address) => {
                    if self.email_addresses.len() < MAX_RECIPIENT_NUMBER {
                        self.current_email_address_valid = EmailAddress::is_valid(&new_address);
                        self.current_email_address = new_address;
                    };
                    iced::Command::none()
                }
                EmailScreenMessage::CurrentEmailAddressSubmitted => {
                    if self.current_email_address.len() > 0 {
                        if self.current_email_address_valid {
                            self.email_addresses
                                .push(self.current_email_address.clone());
                            self.current_email_address = String::new();
                        }
                    } else if self.email_addresses.len() > 0 {
                        // TODO: move on to next screen
                    } else {
                        let flags = super::camera_screen::CameraScreenFlags {
                            config: self.config.clone(),
                            index: self.index.clone(),
                        };
                        return iced::Command::perform(async {}, |_| {
                            super::ScreenMessage::TransitionToScreen(
                                super::ScreenFlags::CameraScreenFlags(flags),
                            )
                        });
                    }
                    iced::Command::none()
                }
            },
            if !self.has_focused_email_field {
                self.has_focused_email_field = true;
                widget::focus_next()
            } else {
                iced::Command::none()
            },
        ])
    }
    fn view(&self) -> Element<EmailScreenMessage> {
        let email_list: Element<_> = scrollable(
            column(
                self.email_addresses
                    .iter()
                    .map(|address| {
                        container(
                            text(address).horizontal_alignment(iced::alignment::Horizontal::Center),
                        )
                        .style(theme::Container::Custom(Box::new(
                            OutlinedContainerStyle {},
                        )))
                        .padding(10)
                        .width(Length::Fill)
                    })
                    .map(Element::from),
            )
            .spacing(10),
        )
        .into();
        container(
            Row::new()
                .push(
                    container(
                        Column::new()
                            .push(text("Enter your emails").size(36))
                            .push(email_list)
                            .push(
                                Row::new()
                                    .push(
                                        TextInput::new(
                                            &if self.email_addresses.len() < MAX_RECIPIENT_NUMBER {
                                                format!("my_email@{}", &self.config.email_example_domain)
                                            } else {
                                                "Maximum number of recipients reached".into()
                                            },
                                            &self.current_email_address
                                        )
                                        .on_input(EmailScreenMessage::CurrentEmailAddressChanged)
                                        .on_submit(EmailScreenMessage::CurrentEmailAddressSubmitted)
                                        .width(Length::Fill)
                                    )
                                    .push(
                                        button(
                                            text(
                                                if self.current_email_address.len() > 0 {
                                                    "Press [Enter] to add email address"
                                                } else if self.email_addresses.len() > 0 {
                                                    "Press [Enter] to finish"
                                                } else {
                                                    "Press [Enter] to cancel and delete your photos"
                                                }
                                            )
                                        )
                                        .style(
                                            if self.current_email_address.len() > 0 {
                                                iced::theme::Button::Secondary
                                            } else if self.email_addresses.len() > 0 {
                                                iced::theme::Button::Primary
                                            } else {
                                                iced::theme::Button::Destructive
                                            }
                                        )
                                        .on_press_maybe(
                                            if self.current_email_address.len() == 0 || self.current_email_address_valid {
                                                Some(EmailScreenMessage::CurrentEmailAddressSubmitted)
                                            } else {
                                                None
                                            }
                                        )
                                    )
                                    .spacing(8)
                            )
                            .push(
                                text(
                                    if self.email_addresses.len() >= MAX_RECIPIENT_NUMBER {
                                        "You have reached the maximum number of recipients. Press [Enter] to have the photo emailed to the above accounts."
                                    } else if self.current_email_address.len() > 0 && !self.current_email_address_valid {
                                        "By entering your email address(es), you consent to having your photos processed by the system and saved on our servers. Please enter a valid email address."
                                    } else if self.current_email_address.len() > 0 {
                                        "By entering your email address(es), you consent to having your photos processed by the system and saved on our servers."
                                    } else if self.email_addresses.len() > 0 {
                                        "You may add more addresses to send the photo to. Press [Enter] to have the photo emailed to the above accounts, or type another one."
                                    } else {
                                        "Enter your email address so we can send you the photos you just took. By entering your email address(es), you consent to having your photos processed by the system and saved on our servers. Press [Enter] now to cancel."
                                    }
                                )
                            )
                            .width(Length::Fill)
                            .spacing(10),
                        )
                        .style(theme::Container::Custom(Box::new(RoundedBoxContainerStyle {})))
                        .padding(16)
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

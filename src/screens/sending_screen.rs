use std::time::Duration;

use base64::Engine;
use iced::{
    widget::{container, text, Column, Space},
    Element, Length,
};
use image::{codecs::png::PngEncoder, ImageEncoder, RgbaImage};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{config::Config, utils::loading_spinners};

#[derive(Serialize, Deserialize)]
#[serde(tag = "status")]
enum ServerResponse {
    #[serde(rename = "error")]
    Error { message: String },
    #[serde(rename = "partial")]
    PartialSuccess { failed_addresses: Vec<String> },
    #[serde(rename = "success")]
    Success,
}

#[derive(Debug)]
pub(crate) struct SendingScreen {
    config: Config,
    index: nokhwa::utils::CameraIndex,

    image: Option<RgbaImage>,
    addresses: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum SendResult {
    DecodeFailure,
    TransferFailure,
    Failure(String),
    PartialSuccess { failed_addresses: Vec<String> },
    Success,
}

#[derive(Debug, Clone)]
pub enum SendingScreenMessage {
    StartSend,
    SendFinished(SendResult),
}

#[derive(Debug, Clone)]
pub(crate) struct SendingScreenFlags {
    pub config: Config,
    pub index: nokhwa::utils::CameraIndex,

    pub image: RgbaImage,
    pub addresses: Vec<String>,
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

                image: Some(flags.image),
                addresses: flags.addresses,
            },
            Some(SendingScreenMessage::StartSend),
        )
    }

    fn update(&mut self, message: SendingScreenMessage) -> iced::Command<super::ScreenMessage> {
        match message {
            SendingScreenMessage::StartSend => {
                if let Some(image) = self.image.take() {
                    let endpoint = self.config.email_server_endpoint.clone();
                    let recipients = self.addresses.clone();
                    iced::Command::perform(
                        async move {
                            // from one test, 2^18 seems to be the amount of memory needed
                            let mut encoded: Vec<u8> = Vec::with_capacity(1 << 18);
                            // if the encoder is switched, make sure to switch the mime
                            let encoder = PngEncoder::new(&mut encoded);
                            let encode_result = encoder.write_image(
                                &image,
                                image.width(),
                                image.height(),
                                image::ColorType::Rgba8,
                            );
                            if matches!(encode_result, Err(..)) {
                                return SendResult::Failure(
                                    "failed to encode image for transport".to_string(),
                                );
                            }
                            let base64_encoded =
                                base64::engine::general_purpose::STANDARD.encode(encoded);
                            let response = reqwest::Client::new()
                                .post(endpoint)
                                .body(
                                    json!({
                                        "recipients": recipients,
                                        "image": base64_encoded,
                                        "imageMime": "image/png" // make sure this matches up with the encoder used above
                                    })
                                    .to_string(),
                                )
                                .send()
                                .await;
                            match response {
                                Ok(response) => match response.json::<ServerResponse>().await {
                                    Ok(parsed) => match parsed {
                                        ServerResponse::Error { message } => {
                                            SendResult::Failure(message)
                                        }
                                        ServerResponse::PartialSuccess { failed_addresses } => {
                                            eprintln!(
                                                "failed to send emails to: {:?}",
                                                failed_addresses
                                            );
                                            SendResult::PartialSuccess { failed_addresses }
                                        }
                                        ServerResponse::Success => SendResult::Success,
                                    },
                                    Err(e) => {
                                        eprintln!("failed to decode server response: {:?}", e);
                                        SendResult::DecodeFailure
                                    }
                                },
                                Err(..) => {
                                    eprintln!("failed to send the request to the server");
                                    SendResult::TransferFailure
                                }
                            }
                        },
                        |result: SendResult| SendingScreenMessage::SendFinished(result),
                    )
                    .map(super::ScreenMessage::SendingScreenMessage)
                } else {
                    eprintln!("warning: SendingScreenMessage::StartSend called while there was no captured frame data");
                    iced::Command::none()
                }
            }
            SendingScreenMessage::SendFinished(result) => {
                let flags = match result {
                    SendResult::Failure(reason) => super::ScreenFlags::ErrorScreenFlags(
                        super::error_screen::ErrorScreenFlags {
                            config: self.config.clone(),
                            index: self.index.clone(),

                            error_title: "Something went wrong".to_string(),
                            error_content: format!("Error message: {}", reason),
                        },
                    ),
                    SendResult::PartialSuccess { failed_addresses } => {
                        super::ScreenFlags::ErrorScreenFlags(
                            super::error_screen::ErrorScreenFlags {
                                config: self.config.clone(),
                                index: self.index.clone(),

                                error_title: "Failed to send to some addresses".to_string(),
                                error_content: format!(
                                    "This could be because of a typo. The failed addresses are: {}",
                                    failed_addresses.join(", ")
                                ),
                            },
                        )
                    }
                    SendResult::DecodeFailure => super::ScreenFlags::ErrorScreenFlags(
                        super::error_screen::ErrorScreenFlags {
                            config: self.config.clone(),
                            index: self.index.clone(),

                            error_title: "Something went wrong".to_string(),
                            error_content:
                                "We couldn't parse the response from the server. Try again later."
                                    .to_string(),
                        },
                    ),
                    SendResult::TransferFailure => super::ScreenFlags::ErrorScreenFlags(
                        super::error_screen::ErrorScreenFlags {
                            config: self.config.clone(),
                            index: self.index.clone(),

                            error_title: "Something went wrong".to_string(),
                            error_content: "The request didn't go through. Try again later."
                                .to_string(),
                        },
                    ),
                    SendResult::Success => super::ScreenFlags::AlertScreenFlags(
                        super::alert_screen::AlertScreenFlags {
                            config: self.config.clone(),
                            index: self.index.clone(),

                            alert_title: "All done!".to_string(),
                            alert_content:
                                "You should have received an email with your photos attached."
                                    .to_string(),
                            timeout: Duration::from_millis(4000),
                        },
                    ),
                };
                iced::Command::perform(async {}, |_| {
                    super::ScreenMessage::TransitionToScreen(flags)
                })
            }
        }
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

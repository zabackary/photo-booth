use std::fmt::Display;

use iced::{
    alignment,
    widget::{button, combo_box, container, text, Column, Space},
    Command, Element, Length,
};
use nokhwa::utils::CameraInfo;

use crate::config::Config;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct CameraWrapper(CameraInfo);

impl Display for CameraWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.human_name())
    }
}

impl From<CameraInfo> for CameraWrapper {
    fn from(value: CameraInfo) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ConfigScreen {
    cameras: combo_box::State<CameraWrapper>,
    selected_camera: Option<CameraWrapper>,
    text: String,
    config: Config,
}

#[derive(Debug, Clone)]
pub enum ConfigScreenMessage {
    CameraSelected(CameraWrapper),
    OptionHovered(CameraWrapper),
    Closed,
    Next,
}

#[derive(Debug, Clone)]
pub(crate) struct ConfigScreenFlags {
    pub config: Config,
}

impl Into<super::ScreenMessage> for ConfigScreenMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::ConfigScreenMessage(self)
    }
}

impl super::Screenish for ConfigScreen {
    type Message = ConfigScreenMessage;
    type Flags = ConfigScreenFlags;
    fn new(flags: ConfigScreenFlags) -> (Self, Option<ConfigScreenMessage>) {
        let cameras = nokhwa::query(nokhwa::utils::ApiBackend::Auto)
            .unwrap()
            .into_iter()
            .map(|info| CameraWrapper::from(info))
            .collect::<Vec<CameraWrapper>>();
        (
            ConfigScreen {
                selected_camera: None,
                text: String::new(),
                cameras: combo_box::State::new(cameras),
                config: flags.config,
            },
            None,
        )
    }
    fn update(&mut self, message: ConfigScreenMessage) -> Command<super::ScreenMessage> {
        match message {
            ConfigScreenMessage::CameraSelected(info) => {
                self.text = info.to_string();
                self.selected_camera = Some(info);
                Command::none()
            }
            ConfigScreenMessage::OptionHovered(info) => {
                self.text = info.to_string();
                Command::none()
            }
            ConfigScreenMessage::Closed => {
                self.text = self
                    .selected_camera
                    .clone()
                    .map(|language| language.to_string())
                    .unwrap_or_default();
                Command::none()
            }
            ConfigScreenMessage::Next => {
                let flags = super::camera_screen::CameraScreenFlags {
                    index: self
                        .selected_camera
                        .as_ref()
                        .expect("selected camera is None")
                        .0
                        .index()
                        .clone(),
                    config: self.config.clone(),
                };
                Command::perform(async {}, |_| {
                    super::ScreenMessage::TransitionToScreen(super::ScreenFlags::CameraScreenFlags(
                        flags,
                    ))
                })
            }
        }
    }
    fn view(&self) -> Element<ConfigScreenMessage> {
        container(
            Column::new()
                .push(text("Configure the photo booth").size(24))
                .push(Space::with_height(16))
                .push(text("Camera").size(18))
                .push(
                    combo_box(
                        &self.cameras,
                        "Search cameras...",
                        self.selected_camera.as_ref(),
                        ConfigScreenMessage::CameraSelected,
                    )
                    .on_option_hovered(ConfigScreenMessage::OptionHovered)
                    .on_close(ConfigScreenMessage::Closed)
                    .width(250),
                )
                .push(Space::with_height(12))
                .push(text("Printer").size(18))
                .push(
                    combo_box(
                        &self.cameras,
                        "Search printers...",
                        self.selected_camera.as_ref(),
                        ConfigScreenMessage::CameraSelected,
                    )
                    .on_option_hovered(ConfigScreenMessage::OptionHovered)
                    .on_close(ConfigScreenMessage::Closed)
                    .width(250),
                )
                .push(Space::with_height(12))
                .push(
                    button(
                        text("Start photo booth")
                            .size(18)
                            .horizontal_alignment(alignment::Horizontal::Center),
                    )
                    .on_press_maybe(
                        self.selected_camera
                            .as_ref()
                            .and(Some(ConfigScreenMessage::Next)),
                    )
                    .padding(6),
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .center_x()
        .center_y()
        .into()
    }
}

impl Into<super::Screen> for ConfigScreen {
    fn into(self) -> super::Screen {
        super::Screen::ConfigScreen(self)
    }
}

use std::fmt::Display;

use iced::{
    alignment,
    widget::{button, column, container, text},
    Color, Command, Element, Length,
};
use iced_aw::SelectionList;
use nokhwa::utils::CameraInfo;

use super::transition_to_screen;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
struct CameraWrapper(CameraInfo);

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
pub(crate) struct CameraSelect {
    cameras: Vec<CameraWrapper>,
    selected_camera: CameraWrapper,
    selected_index: usize,
}

#[derive(Debug, Clone)]
pub enum CameraSelectMessage {
    CameraSelected(usize, CameraWrapper),
    Next,
}

impl Into<super::ScreenMessage> for CameraSelectMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::CameraSelectMessage(self)
    }
}

impl super::Screenish for CameraSelect {
    type Message = CameraSelectMessage;
    fn new() -> (Self, Option<CameraSelectMessage>) {
        let cameras = nokhwa::query(nokhwa::utils::ApiBackend::Auto)
            .unwrap()
            .into_iter()
            .map(|info| CameraWrapper::from(info))
            .collect::<Vec<CameraWrapper>>();
        (
            CameraSelect {
                selected_camera: cameras[0].clone(),
                selected_index: 0,
                cameras,
            },
            None,
        )
    }
    fn update(&mut self, message: CameraSelectMessage) -> Command<super::ScreenMessage> {
        match message {
            CameraSelectMessage::CameraSelected(index, info) => {
                self.selected_camera = info;
                self.selected_index = index;
                Command::none()
            }
            CameraSelectMessage::Next => {
                Command::perform(async {}, |_| transition_to_screen(super::app::App::new()))
            }
        }
    }
    fn view(&self) -> Element<CameraSelectMessage> {
        let selection_list = SelectionList::new(&self.cameras, CameraSelectMessage::CameraSelected);
        let content = column![
            text("Select camera")
                .size(18)
                .style(Color::from([0.5, 0.5, 0.5])),
            button(text("Start").horizontal_alignment(alignment::Horizontal::Center))
                .on_press(CameraSelectMessage::Next)
                .padding(10)
                .width(80),
            selection_list,
        ];
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}

impl Into<super::Screen> for CameraSelect {
    fn into(self) -> super::Screen {
        super::Screen::CameraSelect(self)
    }
}

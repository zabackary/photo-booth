use camerafeed::{CameraFeed, CameraMessage};
use iced::widget::{column, container, text};
use iced::{executor, theme, window, Application, Subscription, Theme};
use iced::{Alignment, Color, Element, Length, Settings};
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::utils::{RequestedFormat, RequestedFormatType};
use nokhwa::Camera;

mod camerafeed;

pub fn main() -> iced::Result {
    let icon = image::load_from_memory(include_bytes!("../assets/icon.png"))
        .expect("failed to decode application icon");
    PhotoBooth::run(Settings {
        window: window::Settings {
            icon: Some(
                window::icon::from_rgba(icon.to_rgba8().to_vec(), icon.width(), icon.height())
                    .expect("failed to construct application icon"),
            ),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct PhotoBooth {
    feed: CameraFeed,
}

#[derive(Debug, Clone)]
enum Message {
    CameraFeedMessage(CameraMessage),
}

impl Application for PhotoBooth {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        // first camera in system
        let index = nokhwa::utils::CameraIndex::Index(0);
        // request the absolute highest resolution CameraFormat that can be decoded to RGB.
        let requested =
            RequestedFormat::new::<RgbAFormat>(RequestedFormatType::HighestFrameRate(30));
        // make the camera
        let mut camera = Camera::new(index, requested).unwrap();
        camera.open_stream().unwrap();
        let (feed, feed_command) = CameraFeed::new(camera);
        (
            PhotoBooth { feed },
            feed_command.map(Message::CameraFeedMessage),
        )
    }

    fn title(&self) -> String {
        String::from("Photo Booth")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::CameraFeedMessage(msg) => {
                self.feed.update(msg).map(Message::CameraFeedMessage)
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        self.feed.subscription().map(Message::CameraFeedMessage)
        // Subscription::none()
    }

    fn theme(&self) -> Theme {
        Theme::Custom(
            theme::Custom::new(theme::Palette {
                background: Color::from_rgb8(22, 33, 106),
                text: Color::from_rgb8(111, 125, 224),
                primary: Color::from_rgb8(0, 0, 255),
                success: Color::from_rgb8(136, 240, 122),
                danger: Color::from_rgb8(224, 111, 111),
            })
            .into(),
        )
    }

    fn view(&self) -> Element<Message> {
        let content = column![text("Photo Booth")
            .size(24)
            .style(Color::from([0.5, 0.5, 0.5])),]
        .width(700)
        .spacing(20)
        .align_items(Alignment::Center)
        .push(container(
            self.feed.view().width(Length::Fill).height(Length::Fill),
        ));

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}

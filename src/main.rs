mod camera_feed;
mod config;
mod screens;

use camera_feed::{CameraFeed, CameraMessage};
use config::Config;
use iced::widget::{button, column, container, row, text};
use iced::window::Mode;
use iced::{alignment, executor, theme, window, Application, Font, Subscription, Theme};
use iced::{Alignment, Color, Element, Length, Settings};
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::utils::{RequestedFormat, RequestedFormatType};
use nokhwa::Camera;

pub fn main() -> iced::Result {
    let icon = image::load_from_memory(include_bytes!("../assets/icon.png"))
        .expect("failed to decode application icon");
    let config =
        config::Config::new(include_str!("../assets/config.json")).expect("failed to read config");
    PhotoBooth::run(Settings {
        window: window::Settings {
            icon: Some(
                window::icon::from_rgba(icon.to_rgba8().to_vec(), icon.width(), icon.height())
                    .expect("failed to construct application icon"),
            ),
            decorations: !config.fullscreen,
            ..window::Settings::default()
        },
        flags: config,
        antialiasing: false,
        exit_on_close_request: true,
        default_font: Font::DEFAULT,
        id: None,
        default_text_size: 16.0,
    })
}

struct PhotoBooth {
    screen: screens::Screen,
}

#[derive(Debug, Clone)]
enum Message {
    ScreenMessage(screens::ScreenMessage),
    ExitPressed,
}

impl Application for PhotoBooth {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = Config;
    type Theme = Theme;

    fn new(flags: Config) -> (Self, iced::Command<Message>) {
        let (screen, command) = screens::initial_screen();
        let command = match command {
            Some(inner) => iced::Command::perform(async {}, |_| inner),
            None => iced::Command::none(),
        };
        (
            PhotoBooth { screen },
            iced::Command::batch([
                command.map(Message::ScreenMessage),
                if flags.fullscreen {
                    window::change_mode(Mode::Fullscreen)
                } else {
                    iced::Command::none()
                },
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Photo Booth")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::ScreenMessage(msg) => match self.screen.update(msg) {
                screens::ScreenUpdateOutcome::Command(cmd) => cmd.map(Message::ScreenMessage),
                screens::ScreenUpdateOutcome::NewScreen(screen, cmd) => {
                    self.screen = screen;
                    cmd.map(Message::ScreenMessage)
                }
            },
            Message::ExitPressed => window::close(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        self.screen.subscription().map(Message::ScreenMessage)
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
        let content = column![
            row![
                text("Photo Booth")
                    .size(24)
                    .style(Color::from([0.8, 0.8, 0.8]))
                    .width(Length::Fill),
                text(format!("v{}", env!("CARGO_PKG_VERSION")))
                    .size(18)
                    .style(Color::from([0.3, 0.3, 0.3]))
                    .vertical_alignment(alignment::Vertical::Center),
                button(text("Exit")).on_press(Message::ExitPressed)
            ],
            self.screen.view().map(Message::ScreenMessage)
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()
    }
}

mod camera_feed;
mod config;
mod screens;
mod utils;

use config::Config;
use iced::widget::{button, container, text, Column, Row, Space};
use iced::window::Mode;
use iced::{alignment, executor, theme, window, Application, Font, Subscription, Theme};
use iced::{Alignment, Color, Element, Length, Settings};

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
        default_font: Font::DEFAULT,
        fonts: vec![],
        id: None,
        default_text_size: iced::Pixels(16.0),
    })
}

struct PhotoBooth {
    screen: screens::Screen,
    config: Config,
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
        let (screen, command) = screens::initial_screen().into();
        let command = match command {
            Some(inner) => iced::Command::perform(async {}, |_| inner),
            None => iced::Command::none(),
        };
        let fullscreen = flags.fullscreen;
        (
            PhotoBooth {
                screen,
                config: flags,
            },
            iced::Command::batch([
                command.map(Message::ScreenMessage),
                if fullscreen {
                    window::change_mode(window::Id::MAIN, Mode::Fullscreen)
                } else {
                    iced::Command::none()
                },
            ]),
        )
    }

    fn title(&self) -> String {
        self.config.name.clone()
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
            Message::ExitPressed => window::close(window::Id::MAIN),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        self.screen.subscription().map(Message::ScreenMessage)
    }

    fn theme(&self) -> Theme {
        theme::Theme::TokyoNight
    }

    fn view(&self) -> Element<Message> {
        let content = Column::new()
            .push(
                Row::new()
                    .push(
                        text(&self.config.name)
                            .size(24)
                            .style(Color::from([0.8, 0.8, 0.8]))
                            .width(Length::Fill)
                            .vertical_alignment(alignment::Vertical::Center),
                    )
                    .push(container(
                        text(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .size(18)
                            .style(Color::from([0.3, 0.3, 0.3]))
                            .vertical_alignment(alignment::Vertical::Center),
                    ))
                    .push(Space::with_width(12))
                    .push(button(text("Exit")).on_press(Message::ExitPressed))
                    .align_items(Alignment::Center)
                    .padding(20),
            )
            .push(self.screen.view().map(Message::ScreenMessage))
            .spacing(20)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

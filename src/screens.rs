use iced::{Element, Subscription};

use crate::config::Config;

mod camera_screen;
mod config_screen;
mod printing_screen;

#[derive(Debug, Clone)]
pub enum ScreenMessage {
    TransitionToScreen(ScreenFlags),
    CameraScreenMessage(camera_screen::CameraScreenMessage),
    ConfigScreenMessage(config_screen::ConfigScreenMessage),
    PrintingScreenMessage(printing_screen::PrintingScreenMessage),
}

#[derive(Debug, Clone)]
pub enum ScreenFlags {
    CameraScreenFlags(camera_screen::CameraScreenFlags),
    ConfigScreenFlags(config_screen::ConfigScreenFlags),
    PrintingScreenFlags(printing_screen::PrintingScreenFlags),
}

impl Into<(Screen, Option<ScreenMessage>)> for ScreenFlags {
    fn into(self) -> (Screen, Option<ScreenMessage>) {
        match self {
            ScreenFlags::CameraScreenFlags(flags) => {
                let (screen, message) = camera_screen::CameraScreen::new(flags);
                (
                    Screen::CameraScreen(screen),
                    message.map(ScreenMessage::CameraScreenMessage),
                )
            }
            ScreenFlags::ConfigScreenFlags(flags) => {
                let (screen, message) = config_screen::ConfigScreen::new(flags);
                (
                    Screen::ConfigScreen(screen),
                    message.map(ScreenMessage::ConfigScreenMessage),
                )
            }
            ScreenFlags::PrintingScreenFlags(flags) => {
                let (screen, message) = printing_screen::PrintingScreen::new(flags);
                (
                    Screen::PrintingScreen(screen),
                    message.map(ScreenMessage::PrintingScreenMessage),
                )
            }
        }
    }
}

pub fn initial_screen(config: Config) -> ScreenFlags {
    ScreenFlags::ConfigScreenFlags(config_screen::ConfigScreenFlags { config })
}

#[derive(Debug)]
pub enum Screen {
    CameraScreen(camera_screen::CameraScreen),
    ConfigScreen(config_screen::ConfigScreen),
    PrintingScreen(printing_screen::PrintingScreen),
}

#[derive(Debug)]
pub enum ScreenUpdateOutcome<T> {
    Command(iced::Command<T>),
    NewScreen(Screen, iced::Command<ScreenMessage>),
}

impl Screen {
    pub fn update(&mut self, message: ScreenMessage) -> ScreenUpdateOutcome<ScreenMessage> {
        match (self, message) {
            (_, ScreenMessage::TransitionToScreen(flags)) => {
                let (screen, message) = flags.into();
                ScreenUpdateOutcome::NewScreen(
                    screen,
                    match message {
                        Some(inner) => iced::Command::perform(async {}, |_| inner),
                        None => iced::Command::none(),
                    },
                )
            }
            (Screen::CameraScreen(screen), ScreenMessage::CameraScreenMessage(msg)) => {
                ScreenUpdateOutcome::Command(screen.update(msg).map(|x| x.into()))
            }
            (Screen::ConfigScreen(screen), ScreenMessage::ConfigScreenMessage(msg)) => {
                ScreenUpdateOutcome::Command(screen.update(msg).map(|x| x.into()))
            }
            (Screen::PrintingScreen(screen), ScreenMessage::PrintingScreenMessage(msg)) => {
                ScreenUpdateOutcome::Command(screen.update(msg).map(|x| x.into()))
            }
            _ => {
                // don't do anything
                ScreenUpdateOutcome::Command(iced::Command::none())
            }
        }
    }

    pub fn subscription(&self) -> Subscription<ScreenMessage> {
        match self {
            Screen::CameraScreen(screen) => screen.subscription().map(|x| x.into()),
            Screen::ConfigScreen(screen) => screen.subscription().map(|x| x.into()),
            Screen::PrintingScreen(screen) => screen.subscription().map(|x| x.into()),
        }
    }

    pub fn view(&self) -> Element<ScreenMessage> {
        match self {
            Screen::CameraScreen(screen) => screen.view().map(|x| x.into()),
            Screen::ConfigScreen(screen) => screen.view().map(|x| x.into()),
            Screen::PrintingScreen(screen) => screen.view().map(|x| x.into()),
        }
    }
}

/// A screen. This doesn't have any practical use other than organizing
/// code.
trait Screenish: Sized {
    type Message;
    type Flags;
    fn new(flags: Self::Flags) -> (Self, Option<impl Into<ScreenMessage>>);
    fn update(
        &mut self,
        message: Self::Message,
    ) -> iced::Command<impl Into<ScreenMessage> + 'static>;
    fn subscription(&self) -> Subscription<impl Into<ScreenMessage> + 'static> {
        Subscription::<ScreenMessage>::none()
    }
    fn view(&self) -> Element<impl Into<ScreenMessage>>;
}

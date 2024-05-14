use iced::{Element, Subscription};

use crate::config::Config;

mod alert_screen;
mod camera_screen;
mod config_screen;
mod email_screen;
mod error_screen;
mod generation_screen;
mod sending_screen;

#[derive(Debug, Clone)]
pub enum ScreenMessage {
    TransitionToScreen(ScreenFlags),
    AlertScreenMessage(alert_screen::AlertScreenMessage),
    CameraScreenMessage(camera_screen::CameraScreenMessage),
    ConfigScreenMessage(config_screen::ConfigScreenMessage),
    EmailScreenMessage(email_screen::EmailScreenMessage),
    ErrorScreenMessage(error_screen::ErrorScreenMessage),
    GenerationScreenMessage(generation_screen::GenerationScreenMessage),
    SendingScreenMessage(sending_screen::SendingScreenMessage),
}

#[derive(Debug, Clone)]
pub enum ScreenFlags {
    AlertScreenFlags(alert_screen::AlertScreenFlags),
    CameraScreenFlags(camera_screen::CameraScreenFlags),
    ConfigScreenFlags(config_screen::ConfigScreenFlags),
    EmailScreenFlags(email_screen::EmailScreenFlags),
    ErrorScreenFlags(error_screen::ErrorScreenFlags),
    GenerationScreenFlags(generation_screen::GenerationScreenFlags),
    SendingScreenFlags(sending_screen::SendingScreenFlags),
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
            ScreenFlags::GenerationScreenFlags(flags) => {
                let (screen, message) = generation_screen::GenerationScreen::new(flags);
                (
                    Screen::GenerationScreen(screen),
                    message.map(ScreenMessage::GenerationScreenMessage),
                )
            }
            ScreenFlags::EmailScreenFlags(flags) => {
                let (screen, message) = email_screen::EmailScreen::new(flags);
                (
                    Screen::EmailScreen(screen),
                    message.map(ScreenMessage::EmailScreenMessage),
                )
            }
            ScreenFlags::AlertScreenFlags(flags) => {
                let (screen, message) = alert_screen::AlertScreen::new(flags);
                (
                    Screen::AlertScreen(screen),
                    message.map(ScreenMessage::AlertScreenMessage),
                )
            }
            ScreenFlags::ErrorScreenFlags(flags) => {
                let (screen, message) = error_screen::ErrorScreen::new(flags);
                (
                    Screen::ErrorScreen(screen),
                    message.map(ScreenMessage::ErrorScreenMessage),
                )
            }
            ScreenFlags::SendingScreenFlags(flags) => {
                let (screen, message) = sending_screen::SendingScreen::new(flags);
                (
                    Screen::SendingScreen(screen),
                    message.map(ScreenMessage::SendingScreenMessage),
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
    AlertScreen(alert_screen::AlertScreen),
    CameraScreen(camera_screen::CameraScreen),
    ConfigScreen(config_screen::ConfigScreen),
    EmailScreen(email_screen::EmailScreen),
    ErrorScreen(error_screen::ErrorScreen),
    GenerationScreen(generation_screen::GenerationScreen),
    SendingScreen(sending_screen::SendingScreen),
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
            (Screen::GenerationScreen(screen), ScreenMessage::GenerationScreenMessage(msg)) => {
                ScreenUpdateOutcome::Command(screen.update(msg).map(|x| x.into()))
            }
            (Screen::EmailScreen(screen), ScreenMessage::EmailScreenMessage(msg)) => {
                ScreenUpdateOutcome::Command(screen.update(msg).map(|x| x.into()))
            }
            (Screen::AlertScreen(screen), ScreenMessage::AlertScreenMessage(msg)) => {
                ScreenUpdateOutcome::Command(screen.update(msg).map(|x| x.into()))
            }
            (Screen::ErrorScreen(screen), ScreenMessage::ErrorScreenMessage(msg)) => {
                ScreenUpdateOutcome::Command(screen.update(msg).map(|x| x.into()))
            }
            (Screen::SendingScreen(screen), ScreenMessage::SendingScreenMessage(msg)) => {
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
            Screen::AlertScreen(screen) => screen.subscription().map(|x| x.into()),
            Screen::CameraScreen(screen) => screen.subscription().map(|x| x.into()),
            Screen::ConfigScreen(screen) => screen.subscription().map(|x| x.into()),
            Screen::EmailScreen(screen) => screen.subscription().map(|x| x.into()),
            Screen::ErrorScreen(screen) => screen.subscription().map(|x| x.into()),
            Screen::GenerationScreen(screen) => screen.subscription().map(|x| x.into()),
            Screen::SendingScreen(screen) => screen.subscription().map(|x| x.into()),
        }
    }

    pub fn view(&self) -> Element<ScreenMessage> {
        match self {
            Screen::AlertScreen(screen) => screen.view().map(|x| x.into()),
            Screen::CameraScreen(screen) => screen.view().map(|x| x.into()),
            Screen::ConfigScreen(screen) => screen.view().map(|x| x.into()),
            Screen::EmailScreen(screen) => screen.view().map(|x| x.into()),
            Screen::ErrorScreen(screen) => screen.view().map(|x| x.into()),
            Screen::GenerationScreen(screen) => screen.view().map(|x| x.into()),
            Screen::SendingScreen(screen) => screen.view().map(|x| x.into()),
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

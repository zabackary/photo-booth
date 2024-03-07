use iced::{Element, Subscription};

mod app;
mod camera_select;

#[derive(Debug, Clone)]
pub enum ScreenMessage {
    TransitionToScreen(Screen, Box<Option<ScreenMessage>>),
    AppMessage(app::AppMessage),
    CameraSelectMessage(camera_select::CameraSelectMessage),
}

pub fn initial_screen() -> (Screen, Option<ScreenMessage>) {
    let (screen, command) = camera_select::CameraSelect::new();
    (Screen::CameraSelect(screen), command.map(|x| x.into()))
}

fn transition_to_screen(
    init: (
        impl Into<Screen>,
        Option<impl Into<ScreenMessage> + 'static>,
    ),
) -> ScreenMessage {
    ScreenMessage::TransitionToScreen(init.0.into(), Box::new(init.1.map(|x| x.into())))
}

#[derive(Clone, Debug)]
pub enum Screen {
    App(app::App),
    CameraSelect(camera_select::CameraSelect),
}

#[derive(Debug)]
pub enum ScreenUpdateOutcome<T> {
    Command(iced::Command<T>),
    NewScreen(Screen, iced::Command<ScreenMessage>),
}

impl Screen {
    pub fn update(&mut self, message: ScreenMessage) -> ScreenUpdateOutcome<ScreenMessage> {
        match (self, message) {
            (_, ScreenMessage::TransitionToScreen(new_screen, command)) => {
                ScreenUpdateOutcome::NewScreen(
                    new_screen,
                    match *command {
                        Some(inner) => iced::Command::perform(async {}, |_| inner),
                        None => iced::Command::none(),
                    },
                )
            }
            (Screen::App(screen), ScreenMessage::AppMessage(msg)) => {
                ScreenUpdateOutcome::Command(screen.update(msg).map(|x| x.into()))
            }
            (Screen::CameraSelect(screen), ScreenMessage::CameraSelectMessage(msg)) => {
                ScreenUpdateOutcome::Command(screen.update(msg).map(|x| x.into()))
            }
            _ => {
                // don't do anything
                ScreenUpdateOutcome::Command(iced::Command::none())
            }
        }
    }

    pub fn subscription<'a, 'b>(&self) -> Subscription<ScreenMessage> {
        match self {
            Screen::App(screen) => screen.clone().subscription().map(|x| x.into()),
            Screen::CameraSelect(screen) => screen.clone().subscription().map(|x| x.into()),
        }
    }

    pub fn view(&self) -> Element<ScreenMessage> {
        match self {
            Screen::App(screen) => screen.view().map(|x| x.into()),
            Screen::CameraSelect(screen) => screen.view().map(|x| x.into()),
        }
    }
}

/// A screen. This doesn't have any practical use other than organizing
/// code.
trait Screenish: Sized {
    type Message;
    fn new() -> (Self, Option<impl Into<ScreenMessage>>);
    fn update(
        &mut self,
        message: Self::Message,
    ) -> iced::Command<impl Into<ScreenMessage> + 'static>;
    fn subscription(self) -> Subscription<impl Into<ScreenMessage>> {
        Subscription::<ScreenMessage>::none()
    }
    fn view(&self) -> Element<impl Into<ScreenMessage>>;
}

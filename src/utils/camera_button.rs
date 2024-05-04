use iced::widget::{button, component, space, Component};
use iced::{Background, Border, Color, Element, Length, Size};

const CAMERA_BUTTON_SIZE: f32 = 64.0;

#[derive(Debug, Clone, Copy, Default)]
struct CameraStyle {}
impl button::StyleSheet for CameraStyle {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border: Border {
                color: Color::from_rgb8(200, 200, 200),
                radius: i32::MAX.into(),
                width: 10.0,
            },
            background: Some(Background::Color(Color::from_rgb8(255, 255, 255))),
            ..Default::default()
        }
    }
    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border: Border {
                color: Color::from_rgb8(128, 128, 128),
                radius: i32::MAX.into(),
                width: 10.0,
            },
            background: Some(Background::Color(Color::from_rgb8(200, 200, 200))),
            ..Default::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border: Border {
                color: Color::from_rgb8(180, 180, 180),
                radius: i32::MAX.into(),
                width: 8.0,
            },
            background: Some(Background::Color(Color::from_rgb8(230, 230, 230))),
            ..Default::default()
        }
    }
    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border: Border {
                color: Color::from_rgb8(180, 180, 180),
                radius: i32::MAX.into(),
                width: 12.0,
            },
            background: Some(Background::Color(Color::from_rgb8(220, 220, 220))),
            ..Default::default()
        }
    }
}

pub struct CameraButton<Message: Clone> {
    on_press: Option<Message>,
}

pub fn camera_button<Message: Clone>() -> CameraButton<Message> {
    CameraButton::new()
}

#[derive(Debug, Clone)]
pub enum Event {
    ButtonPressed,
}

impl<Message: Clone> CameraButton<Message> {
    pub fn new() -> Self {
        Self { on_press: None }
    }

    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(on_press);
        self
    }
}

impl<Message: Clone> Component<Message> for CameraButton<Message> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
        match event {
            Event::ButtonPressed => self.on_press.clone(),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Event> {
        button(space::Space::new(0.0, 0.0))
            .style(iced::theme::Button::custom(CameraStyle {}))
            .on_press_maybe(match self.on_press {
                Some(_) => Some(Event::ButtonPressed),
                None => None,
            })
            .width(CAMERA_BUTTON_SIZE)
            .height(CAMERA_BUTTON_SIZE)
            .into()
    }

    fn size_hint(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(CAMERA_BUTTON_SIZE),
            height: Length::Fixed(CAMERA_BUTTON_SIZE),
        }
    }
}

impl<'a, Message> From<CameraButton<Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(camera_button: CameraButton<Message>) -> Self {
        component(camera_button)
    }
}

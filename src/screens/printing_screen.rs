use iced::{
    widget::{container, text, Column, Image, ProgressBar, Row, Space},
    Element, Length,
};

#[derive(Clone)]
pub(crate) struct PrintingScreen {}

impl std::fmt::Debug for PrintingScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish()
    }
}

#[derive(Debug, Clone)]
pub enum PrintingScreenMessage {}

#[derive(Debug, Clone)]
pub(crate) struct PrintingScreenFlags {}

impl Into<super::ScreenMessage> for PrintingScreenMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::PrintingScreenMessage(self)
    }
}

impl super::Screenish for PrintingScreen {
    type Message = PrintingScreenMessage;
    type Flags = PrintingScreenFlags;
    fn new(_flags: PrintingScreenFlags) -> (Self, Option<PrintingScreenMessage>) {
        (PrintingScreen {}, None)
    }
    fn update(&mut self, message: PrintingScreenMessage) -> iced::Command<PrintingScreenMessage> {
        match message {}
    }
    fn view(&self) -> Element<PrintingScreenMessage> {
        container(
            Row::new()
                .push(
                    Column::new()
                        .push(text("Printing...").size(46))
                        .push(Space::with_height(24))
                        .push(ProgressBar::new(0.0..=1.0, 0.5).height(16).width(460))
                        .align_items(iced::Alignment::Center)
                        .width(Length::Fill),
                )
                .push(Image::new("assets/template.png"))
                .align_items(iced::Alignment::Center)
                .spacing(24),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
    }
    fn subscription(&self) -> iced::Subscription<PrintingScreenMessage> {
        iced::Subscription::none()
    }
}

impl Into<super::Screen> for PrintingScreen {
    fn into(self) -> super::Screen {
        super::Screen::PrintingScreen(self)
    }
}

mod image_strip_renderer;

use iced::{
    widget::{container, image::Handle, text, Column, Image, ProgressBar, Row, Space},
    Element, Length,
};
use image::RgbaImage;

use crate::config::Config;

use self::image_strip_renderer::image_strip_renderer;

#[derive(Debug)]
enum ProcessingState {
    GeneratingImage,
    GenerateImageFailed,
    Printing,
}

#[derive(Debug)]
pub(crate) struct PrintingScreen {
    config: Config,

    captured_frames: Option<Vec<image::RgbaImage>>,

    processing_state: ProcessingState,
    preview_handle: Option<Handle>,
    printable_image: Option<RgbaImage>,
}

#[derive(Debug, Clone)]
pub enum PrintingScreenMessage {
    GenerateImage,
    FinishProcessImage(Option<(image::RgbaImage, Handle)>),
}

#[derive(Debug, Clone)]
pub(crate) struct PrintingScreenFlags {
    pub config: Config,
    pub captured_frames: Vec<image::RgbaImage>,
}

impl Into<super::ScreenMessage> for PrintingScreenMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::PrintingScreenMessage(self)
    }
}

impl super::Screenish for PrintingScreen {
    type Message = PrintingScreenMessage;
    type Flags = PrintingScreenFlags;
    fn new(flags: PrintingScreenFlags) -> (Self, Option<PrintingScreenMessage>) {
        (
            PrintingScreen {
                processing_state: ProcessingState::GeneratingImage,
                captured_frames: Some(flags.captured_frames),
                config: flags.config,

                preview_handle: None,
                printable_image: None,
            },
            Some(PrintingScreenMessage::GenerateImage),
        )
    }
    fn update(&mut self, message: PrintingScreenMessage) -> iced::Command<PrintingScreenMessage> {
        match message {
            PrintingScreenMessage::GenerateImage => {
                let template = self.config.template.clone();
                let maybe_captured_frames = self.captured_frames.take();
                if let Some(captured_frames) = maybe_captured_frames {
                    self.processing_state = ProcessingState::GeneratingImage;
                    iced::Command::perform(
                        async move {
                            tokio::task::spawn_blocking(move || {
                                let rendered = image_strip_renderer(
                                    image::io::Reader::open("assets/template.png")
                                        .expect("failed to read template file")
                                        .decode()
                                        .expect("failed to decode template image")
                                        .into_rgba8(),
                                    &captured_frames,
                                    &template,
                                );
                                (
                                    rendered.clone(),
                                    Handle::from_pixels(
                                        rendered.width(),
                                        rendered.height(),
                                        rendered.into_raw(),
                                    ),
                                )
                            })
                            .await
                            .ok()
                        },
                        PrintingScreenMessage::FinishProcessImage,
                    )
                } else {
                    eprintln!("warning: PrintingScreenMessage::GenerateImage called while there was no captured frame data");
                    iced::Command::none()
                }
            }
            PrintingScreenMessage::FinishProcessImage(Some((rendered, handle))) => {
                self.printable_image = Some(rendered);
                self.preview_handle = Some(handle);
                self.processing_state = ProcessingState::Printing;
                iced::Command::none()
            }
            PrintingScreenMessage::FinishProcessImage(None) => {
                self.processing_state = ProcessingState::GenerateImageFailed;
                iced::Command::none()
            }
        }
    }
    fn view(&self) -> Element<PrintingScreenMessage> {
        container(
            Row::new()
                .push(
                    Column::new()
                        .push(
                            text(match self.processing_state {
                                ProcessingState::GeneratingImage => "Processing your photos...",
                                ProcessingState::Printing => "Printing...",
                                ProcessingState::GenerateImageFailed => {
                                    "Failed to generate your image."
                                }
                            })
                            .size(46),
                        )
                        .push(Space::with_height(24))
                        .push(ProgressBar::new(0.0..=1.0, 0.5).height(16).width(460))
                        .align_items(iced::Alignment::Center)
                        .width(Length::Fill),
                )
                .push(Image::new(
                    self.preview_handle
                        .clone()
                        .unwrap_or_else(|| Handle::from("assets/template.png")),
                ))
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

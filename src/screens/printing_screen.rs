mod image_strip_renderer;

use std::time::Duration;

use anim::{Animation, Timeline};
use iced::{
    widget::{container, image::Handle, text, Column, Image, ProgressBar, Row, Space},
    Element, Length,
};
use image::RgbaImage;

use crate::config::Config;

use self::image_strip_renderer::image_strip_renderer;

fn progress_bar_animation(
    old_value: f32,
    new_value: f32,
    duration_ms: u64,
) -> impl Animation<Item = f32> {
    anim::Options::new(old_value, new_value)
        .easing(anim::easing::quad_ease().mode(anim::easing::EasingMode::InOut))
        .duration(Duration::from_millis(duration_ms))
        .build()
}

#[derive(Debug)]
enum ProcessingState {
    GeneratingImage,
    GenerateImageFailed,
    Printing,
}

#[derive(Debug)]
pub(crate) struct PrintingScreen {
    config: Config,

    progress_bar_timeline: Timeline<f32>,

    captured_frames: Option<Vec<image::RgbaImage>>,

    processing_state: ProcessingState,
    preview_handle: Option<Handle>,
    printable_image: Option<RgbaImage>,
}

#[derive(Debug, Clone)]
pub enum PrintingScreenMessage {
    GenerateImage,
    FinishProcessImage(Option<(image::RgbaImage, Handle)>),
    Tick,
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

                progress_bar_timeline: progress_bar_animation(0.0, 0.8, 3000).to_timeline(),

                preview_handle: None,
                printable_image: None,
            },
            Some(PrintingScreenMessage::GenerateImage),
        )
    }
    fn update(&mut self, message: PrintingScreenMessage) -> iced::Command<PrintingScreenMessage> {
        match message {
            PrintingScreenMessage::GenerateImage => {
                self.progress_bar_timeline.begin();
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
                self.progress_bar_timeline =
                    progress_bar_animation(self.progress_bar_timeline.value(), 1.0, 500)
                        .to_timeline();
                self.progress_bar_timeline.begin();
                iced::Command::none()
            }
            PrintingScreenMessage::FinishProcessImage(None) => {
                self.processing_state = ProcessingState::GenerateImageFailed;
                iced::Command::none()
            }
            PrintingScreenMessage::Tick => {
                self.progress_bar_timeline.update();
                if self.progress_bar_timeline.status().is_completed()
                    && self.progress_bar_timeline.value() == 1.0
                {
                    self.progress_bar_timeline.pause();
                }
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
                                ProcessingState::Printing => "Email addresses",
                                ProcessingState::GenerateImageFailed => {
                                    "Failed to generate your image."
                                }
                            })
                            .size(46),
                        )
                        .push(Space::with_height(24))
                        .push(
                            ProgressBar::new(0.0..=1.0, self.progress_bar_timeline.value())
                                .height(16)
                                .width(460),
                        )
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
        if self.progress_bar_timeline.status().is_animating() {
            const FPS: f32 = 60.0;
            iced::time::every(Duration::from_secs_f32(1.0 / FPS))
                .map(|_tick| PrintingScreenMessage::Tick)
        } else {
            iced::Subscription::none()
        }
    }
}

impl Into<super::Screen> for PrintingScreen {
    fn into(self) -> super::Screen {
        super::Screen::PrintingScreen(self)
    }
}

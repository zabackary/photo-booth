mod image_strip_renderer;

use std::time::Duration;

use anim::{Animation, Timeline};
use iced::{
    widget::{container, image::Handle, text, Column, ProgressBar, Space},
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
    GenerateImageFinished,
}

#[derive(Debug)]
pub(crate) struct GenerationScreen {
    config: Config,
    index: nokhwa::utils::CameraIndex,

    progress_bar_timeline: Timeline<f32>,

    captured_frames: Option<Vec<image::RgbaImage>>,

    processing_state: ProcessingState,
    preview_handle: Option<Handle>,
    printable_image: Option<RgbaImage>,
}

#[derive(Debug, Clone)]
pub enum GenerationScreenMessage {
    GenerateImage,
    FinishProcessImage(Option<(image::RgbaImage, Handle)>),
    Tick,
}

#[derive(Debug, Clone)]
pub(crate) struct GenerationScreenFlags {
    pub config: Config,
    pub index: nokhwa::utils::CameraIndex,

    pub captured_frames: Vec<image::RgbaImage>,
}

impl Into<super::ScreenMessage> for GenerationScreenMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::GenerationScreenMessage(self)
    }
}

impl super::Screenish for GenerationScreen {
    type Message = GenerationScreenMessage;
    type Flags = GenerationScreenFlags;
    fn new(flags: GenerationScreenFlags) -> (Self, Option<GenerationScreenMessage>) {
        (
            GenerationScreen {
                processing_state: ProcessingState::GeneratingImage,
                captured_frames: Some(flags.captured_frames),
                config: flags.config,
                index: flags.index,

                progress_bar_timeline: progress_bar_animation(0.0, 0.8, 3000).to_timeline(),

                preview_handle: None,
                printable_image: None,
            },
            Some(GenerationScreenMessage::GenerateImage),
        )
    }
    fn update(&mut self, message: GenerationScreenMessage) -> iced::Command<super::ScreenMessage> {
        match message {
            GenerationScreenMessage::GenerateImage => {
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
                        GenerationScreenMessage::FinishProcessImage,
                    )
                    .map(super::ScreenMessage::GenerationScreenMessage)
                } else {
                    eprintln!("warning: GenerationScreenMessage::GenerateImage called while there was no captured frame data");
                    iced::Command::none()
                }
            }
            GenerationScreenMessage::FinishProcessImage(Some((rendered, handle))) => {
                self.printable_image = Some(rendered);
                self.preview_handle = Some(handle);
                self.processing_state = ProcessingState::GenerateImageFinished;
                self.progress_bar_timeline =
                    progress_bar_animation(self.progress_bar_timeline.value(), 1.0, 500)
                        .to_timeline();
                self.progress_bar_timeline.begin();
                iced::Command::none()
            }
            GenerationScreenMessage::FinishProcessImage(None) => {
                self.processing_state = ProcessingState::GenerateImageFailed;
                iced::Command::none()
            }
            GenerationScreenMessage::Tick => {
                self.progress_bar_timeline.update();
                if self.progress_bar_timeline.status().is_completed()
                    && self.progress_bar_timeline.value() == 1.0
                {
                    let config = self.config.clone();
                    let index = self.index.clone();
                    let preview_handle = self
                        .preview_handle
                        .clone()
                        .expect("preview handle is None when progress bar is finished");
                    let printable_image = self
                        .printable_image
                        .clone()
                        .expect("printable image is None when progress bar is finished");
                    return iced::Command::perform(
                        async {
                            super::ScreenFlags::EmailScreenFlags(
                                super::email_screen::EmailScreenFlags {
                                    config,
                                    index,

                                    preview_handle,
                                    printable_image,
                                },
                            )
                        },
                        super::ScreenMessage::TransitionToScreen,
                    );
                }
                iced::Command::none()
            }
        }
    }
    fn view(&self) -> Element<GenerationScreenMessage> {
        container(
            Column::new()
                .push(
                    text(match self.processing_state {
                        ProcessingState::GeneratingImage => "Processing your photos...",
                        ProcessingState::GenerateImageFinished => "Processing your photos...",
                        ProcessingState::GenerateImageFailed => "Failed to generate your image.",
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
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(iced::alignment::Vertical::Center)
        .padding(20)
        .into()
    }

    fn subscription(&self) -> iced::Subscription<GenerationScreenMessage> {
        if self.progress_bar_timeline.status().is_animating() {
            const FPS: f32 = 60.0;
            iced::time::every(Duration::from_secs_f32(1.0 / FPS))
                .map(|_tick| GenerationScreenMessage::Tick)
        } else {
            iced::Subscription::none()
        }
    }
}

impl Into<super::Screen> for GenerationScreen {
    fn into(self) -> super::Screen {
        super::Screen::GenerationScreen(self)
    }
}

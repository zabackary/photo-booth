use std::time::Duration;

use crate::{
    camera_feed::{CameraFeed, CameraMessage},
    utils::circle::circle,
};
use anim::{Animation, Timeline};
use iced::{
    widget::{button, container, space, text, Container, Row},
    Alignment, Color, Element, Length,
};
use iced_aw::floating_element;
use image::Rgba;
use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{RequestedFormat, RequestedFormatType},
    Camera,
};

#[derive(Clone)]
struct CounterAnimationState {
    radius: f32,
    alpha: f32,
}

fn counter_animation() -> impl Animation<Item = CounterAnimationState> {
    const COUNTER_RADIUS: f32 = 80.0;
    const ANIMATION_CYCLE_DURATION: u64 = 1000;
    let radius = anim::builder::key_frames(vec![
        anim::KeyFrame::new(0.0).by_percent(0.0),
        anim::KeyFrame::new(COUNTER_RADIUS)
            .by_percent(0.4)
            .easing(anim::easing::quad_ease()),
        anim::KeyFrame::new(COUNTER_RADIUS).by_percent(0.8),
        anim::KeyFrame::new(0.0)
            .by_duration(Duration::from_millis(ANIMATION_CYCLE_DURATION))
            .easing(anim::easing::quad_ease()),
    ]);
    let alpha = anim::builder::key_frames(vec![
        anim::KeyFrame::new(0.0).by_percent(0.0),
        anim::KeyFrame::new(1.0)
            .by_percent(0.4)
            .easing(anim::easing::quad_ease()),
        anim::KeyFrame::new(1.0).by_duration(Duration::from_millis(ANIMATION_CYCLE_DURATION)),
    ]);
    radius
        .zip(alpha)
        .map(|(radius, alpha)| CounterAnimationState { radius, alpha })
}

pub(crate) struct CameraScreen {
    feed: CameraFeed,
    captured_frames: Vec<image::ImageBuffer<Rgba<u8>, Vec<u8>>>,
    counter_animation_value: u16,
    counter_animation_running: bool,
    counter_timeline: Timeline<CounterAnimationState>,
}

#[derive(Clone, Debug)]
pub(crate) struct CameraScreenFlags {
    pub index: nokhwa::utils::CameraIndex,
}

impl std::fmt::Debug for CameraScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App").finish()
    }
}

#[derive(Debug, Clone)]
pub enum CameraScreenMessage {
    CameraFeedMessage(CameraMessage),
    CaptureButtonPressed,
    StartCaptureFrameAnimation,
    CaptureFrameAnimationEnded,
    AllFramesCaptured,
    Tick,
}

impl Into<super::ScreenMessage> for CameraScreenMessage {
    fn into(self) -> super::ScreenMessage {
        super::ScreenMessage::CameraScreenMessage(self)
    }
}

impl super::Screenish for CameraScreen {
    type Message = CameraScreenMessage;
    type Flags = CameraScreenFlags;
    fn new(flags: CameraScreenFlags) -> (Self, Option<CameraScreenMessage>) {
        let requested =
            RequestedFormat::new::<RgbAFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        let mut camera = Camera::new(flags.index, requested).unwrap();
        camera.open_stream().unwrap();
        let (feed, feed_command) = CameraFeed::new(camera, 48.into());
        (
            CameraScreen {
                feed,
                counter_animation_value: 0,
                counter_animation_running: false,
                captured_frames: vec![],
                counter_timeline: counter_animation().to_timeline(),
            },
            feed_command.map(CameraScreenMessage::CameraFeedMessage),
        )
    }
    fn update(&mut self, message: CameraScreenMessage) -> iced::Command<super::ScreenMessage> {
        match message {
            CameraScreenMessage::CameraFeedMessage(msg) => self
                .feed
                .update(msg)
                .map(CameraScreenMessage::CameraFeedMessage)
                .map(super::ScreenMessage::CameraScreenMessage),
            CameraScreenMessage::StartCaptureFrameAnimation => {
                self.counter_animation_running = true;
                self.counter_timeline.begin();
                iced::Command::none()
            }
            CameraScreenMessage::Tick => {
                self.counter_timeline.update();
                if self.counter_animation_running && self.counter_timeline.status().is_completed() {
                    self.counter_animation_running = false;
                    iced::Command::perform(async {}, |_| {
                        CameraScreenMessage::CaptureFrameAnimationEnded
                    })
                    .map(super::ScreenMessage::CameraScreenMessage)
                } else {
                    iced::Command::none()
                }
            }
            CameraScreenMessage::CaptureFrameAnimationEnded => {
                self.counter_animation_value -= 1;
                if self.counter_animation_value == 0 {
                    // TODO: capture frame
                    iced::Command::none()
                } else {
                    iced::Command::perform(async {}, |_| {
                        CameraScreenMessage::StartCaptureFrameAnimation
                    })
                    .map(super::ScreenMessage::CameraScreenMessage)
                }
            }
            CameraScreenMessage::CaptureButtonPressed => {
                self.counter_animation_value = 3;
                iced::Command::perform(async {}, |_| {
                    CameraScreenMessage::StartCaptureFrameAnimation
                })
                .map(super::ScreenMessage::CameraScreenMessage)
            }
            CameraScreenMessage::AllFramesCaptured => iced::Command::perform(async {}, |_| {
                super::ScreenMessage::TransitionToScreen(super::ScreenFlags::PrintingScreenFlags(
                    super::printing_screen::PrintingScreenFlags {},
                ))
            }),
        }
    }
    fn view(&self) -> Element<CameraScreenMessage> {
        let counter_animation_value = self.counter_timeline.value();
        container(
            Row::new()
                .push(
                    floating_element(
                        self.feed.view().width(Length::Fill).height(Length::Fill),
                        if self.counter_animation_running {
                            container(
                                floating_element(
                                    circle(
                                        counter_animation_value.radius,
                                        Color::from_rgba8(
                                            255,
                                            255,
                                            255,
                                            counter_animation_value.alpha,
                                        ),
                                    ),
                                    container(
                                        text(format!("{}", self.counter_animation_value)).size(80),
                                    )
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .align_x(iced::alignment::Horizontal::Center)
                                    .align_y(iced::alignment::Vertical::Center)
                                    .clip(true),
                                )
                                .offset(0.0),
                            )
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(iced::alignment::Horizontal::Center)
                            .align_y(iced::alignment::Vertical::Center)
                        } else {
                            container(space::Space::new(0, 0))
                        },
                    )
                    .offset(0.0),
                )
                .push(
                    button(text("Take picture"))
                        .on_press(CameraScreenMessage::CaptureButtonPressed),
                )
                .spacing(20)
                .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
    }
    fn subscription(&self) -> iced::Subscription<CameraScreenMessage> {
        iced::Subscription::batch([
            self.feed
                .subscription()
                .map(CameraScreenMessage::CameraFeedMessage),
            if self.counter_timeline.status().is_animating() {
                const FPS: f32 = 60.0;
                iced::time::every(Duration::from_secs_f32(1.0 / FPS))
                    .map(|_tick| CameraScreenMessage::Tick)
            } else {
                iced::Subscription::none()
            },
        ])
    }
}

impl Into<super::Screen> for CameraScreen {
    fn into(self) -> super::Screen {
        super::Screen::CameraScreen(self)
    }
}

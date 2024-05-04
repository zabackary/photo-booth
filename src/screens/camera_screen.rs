use std::time::Duration;

use crate::{
    camera_feed::{CameraFeed, CameraMessage},
    utils::{camera_button::camera_button, circle::circle},
};
use anim::{Animation, Timeline};
use iced::{
    widget::{container, space, text, Row},
    Alignment, Color, Element, Length,
};
use iced_aw::floating_element;
use image::Rgba;
use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{RequestedFormat, RequestedFormatType},
    Camera,
};

const COUNTER_RADIUS: f32 = 80.0;

#[derive(Clone)]
struct CounterAnimationState {
    radius: f32,
    alpha: f32,
    text_size: f32,
}

fn counter_animation() -> impl Animation<Item = CounterAnimationState> {
    const COUNTER_ANIMATION_DURATION: u64 = 1000;
    let radius = anim::builder::key_frames(vec![
        anim::KeyFrame::new(0.0).by_percent(0.0),
        anim::KeyFrame::new(COUNTER_RADIUS)
            .by_percent(0.4)
            .easing(anim::easing::quad_ease()),
        anim::KeyFrame::new(COUNTER_RADIUS).by_percent(0.8),
        anim::KeyFrame::new(0.0)
            .by_duration(Duration::from_millis(COUNTER_ANIMATION_DURATION))
            .easing(anim::easing::quad_ease()),
    ]);
    let alpha = anim::builder::key_frames(vec![
        anim::KeyFrame::new(0.0).by_percent(0.0),
        anim::KeyFrame::new(1.0)
            .by_percent(0.4)
            .easing(anim::easing::quad_ease()),
        anim::KeyFrame::new(1.0).by_duration(Duration::from_millis(COUNTER_ANIMATION_DURATION)),
    ]);
    let text_size = anim::builder::key_frames(vec![
        anim::KeyFrame::new(0.0).by_percent(0.0),
        anim::KeyFrame::new(COUNTER_RADIUS)
            .by_percent(0.4)
            .easing(anim::easing::quad_ease()),
        anim::KeyFrame::new(COUNTER_RADIUS).by_percent(0.8),
        anim::KeyFrame::new(0.0)
            .by_duration(Duration::from_millis(COUNTER_ANIMATION_DURATION))
            .easing(anim::easing::quad_ease()),
    ]);
    radius
        .zip(alpha)
        .zip(text_size)
        .map(|((radius, alpha), text_size)| CounterAnimationState {
            radius,
            alpha,
            text_size,
        })
}

fn snap_animation() -> impl Animation<Item = f32> {
    const SNAP_ANIMATION_DURATION: u64 = 500;
    anim::builder::key_frames(vec![
        anim::KeyFrame::new(0.0).by_percent(0.0),
        anim::KeyFrame::new(1.0)
            .by_percent(0.2)
            .easing(anim::easing::quad_ease()),
        anim::KeyFrame::new(0.0)
            .by_duration(Duration::from_millis(SNAP_ANIMATION_DURATION))
            .easing(anim::easing::quad_ease()),
    ])
}

fn frame_size_animation() -> impl Animation<Item = f32> {
    const FRAME_SIZE_ANIMATION_DURATION: u64 = 1000;
    anim::builder::key_frames(vec![
        anim::KeyFrame::new(0.0).by_percent(0.0),
        anim::KeyFrame::new(1.2)
            .by_percent(0.8)
            .easing(anim::easing::quad_ease()),
        anim::KeyFrame::new(1.0)
            .by_duration(Duration::from_millis(FRAME_SIZE_ANIMATION_DURATION))
            .easing(anim::easing::quad_ease()),
    ])
}

enum CaptureSequenceState {
    None,
    Counter(u16),
    Snap,
    FrameSize,
}

pub(crate) struct CameraScreen {
    feed: CameraFeed,
    captured_frames: Vec<image::ImageBuffer<Rgba<u8>, Vec<u8>>>,

    capture_sequence_state: CaptureSequenceState,
    counter_timeline: Timeline<CounterAnimationState>,
    snap_timeline: Timeline<f32>,
    frame_size_timeline: Timeline<f32>,
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
    ImageCaptured(image::ImageBuffer<Rgba<u8>, Vec<u8>>),
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
                capture_sequence_state: CaptureSequenceState::None,
                captured_frames: vec![],
                counter_timeline: counter_animation().to_timeline(),
                frame_size_timeline: frame_size_animation().to_timeline(),
                snap_timeline: snap_animation().to_timeline(),
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
            CameraScreenMessage::Tick => {
                match self.capture_sequence_state {
                    CaptureSequenceState::None => {}
                    CaptureSequenceState::Counter(value) => {
                        self.counter_timeline.update();
                        if self.counter_timeline.status().is_completed() {
                            if value > 1 {
                                self.capture_sequence_state =
                                    CaptureSequenceState::Counter(value - 1);
                                self.counter_timeline.begin();
                            } else {
                                // start the snap animation
                                self.capture_sequence_state = CaptureSequenceState::Snap;
                                self.snap_timeline.begin();

                                // capture the frame
                                // everything important inside feed is Mutex'd
                                // so perf is fine
                                let mut feed = self.feed.clone();
                                return iced::Command::perform(
                                    async move {
                                        tokio::task::spawn_blocking(move || feed.frame())
                                            .await
                                            .unwrap()
                                    },
                                    CameraScreenMessage::ImageCaptured,
                                )
                                .map(super::ScreenMessage::CameraScreenMessage);
                            }
                        }
                    }
                    CaptureSequenceState::Snap => {
                        self.snap_timeline.update();
                        if self.snap_timeline.status().is_completed() {
                            // start the frame size animation
                            self.capture_sequence_state = CaptureSequenceState::FrameSize;
                            self.frame_size_timeline.begin();
                        }
                    }
                    CaptureSequenceState::FrameSize => {
                        self.frame_size_timeline.update();
                        if self.frame_size_timeline.status().is_completed() {
                            // finish the sequence
                            self.capture_sequence_state = CaptureSequenceState::None;

                            // TODO: transition to the next screen if we're done
                            if false {
                                return iced::Command::perform(async {}, |_| {
                                    super::ScreenMessage::TransitionToScreen(
                                        super::ScreenFlags::PrintingScreenFlags(
                                            super::printing_screen::PrintingScreenFlags {},
                                        ),
                                    )
                                });
                            }
                        }
                    }
                }
                iced::Command::none()
            }
            CameraScreenMessage::ImageCaptured(image) => {
                self.captured_frames.push(image);
                iced::Command::none()
            }
            CameraScreenMessage::CaptureButtonPressed => {
                self.capture_sequence_state = CaptureSequenceState::Counter(3);
                self.counter_timeline.begin();
                iced::Command::none()
            }
        }
    }
    fn view(&self) -> Element<CameraScreenMessage> {
        let counter_animation_value = self.counter_timeline.value();
        let snap_animation_value = self.snap_timeline.value();
        floating_element(
            container(
                Row::new()
                    .push(
                        floating_element(
                            self.feed.view().width(Length::Fill).height(Length::Fill),
                            match self.capture_sequence_state {
                                CaptureSequenceState::Counter(counter) => container(
                                    floating_element(
                                        container(circle(
                                            counter_animation_value.radius,
                                            Color::from_rgba8(
                                                255,
                                                255,
                                                255,
                                                counter_animation_value.alpha,
                                            ),
                                        ))
                                        .width(COUNTER_RADIUS * 2.0)
                                        .height(COUNTER_RADIUS * 2.0)
                                        .align_x(iced::alignment::Horizontal::Center)
                                        .align_y(iced::alignment::Vertical::Center),
                                        container(text(format!("{}", counter)).size(
                                            if counter_animation_value.text_size > 0.0 {
                                                counter_animation_value.text_size
                                            } else {
                                                f32::MIN_POSITIVE
                                            },
                                        ))
                                        .width(Length::Fill)
                                        .height(Length::Fill)
                                        .align_x(iced::alignment::Horizontal::Center)
                                        .align_y(iced::alignment::Vertical::Center)
                                        .clip(true),
                                    )
                                    .offset(0.0),
                                ),
                                _ => container(space::Space::new(0, 0)),
                            }
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(iced::alignment::Horizontal::Center)
                            .align_y(iced::alignment::Vertical::Center),
                        )
                        .offset(0.0),
                    )
                    .push(camera_button().on_press(CameraScreenMessage::CaptureButtonPressed))
                    .spacing(20)
                    .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20),
            if matches!(self.capture_sequence_state, CaptureSequenceState::Snap) {
                container(space::Space::new(0, 0))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(
                        container::Appearance::default().with_background(Color::from_rgba8(
                            255,
                            255,
                            255,
                            snap_animation_value,
                        )),
                    )
            } else {
                container(space::Space::new(0, 0)).width(0).height(0)
            },
        )
        .into()
    }

    fn subscription(&self) -> iced::Subscription<CameraScreenMessage> {
        iced::Subscription::batch([
            self.feed
                .subscription()
                .map(CameraScreenMessage::CameraFeedMessage),
            if self.counter_timeline.status().is_animating()
                || self.snap_timeline.status().is_animating()
                || self.frame_size_timeline.status().is_animating()
            {
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

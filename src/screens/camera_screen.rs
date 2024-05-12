mod element_strip_renderer;

use std::time::Duration;

use crate::{
    camera_feed::{CameraFeed, CameraMessage},
    config::Config,
    utils::{camera_button::camera_button, circle::circle},
};
use anim::{Animation, Timeline};
use iced::{
    widget::{container, image::Handle, space, text, Row},
    Alignment, Color, Element, Length,
};
use iced_aw::floating_element;
use image::Rgba;
use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{RequestedFormat, RequestedFormatType},
    Camera,
};

use self::element_strip_renderer::element_strip_renderer;

const COUNTER_RADIUS: f32 = 80.0;
const GET_READY_FONT_SIZE: f32 = 60.0;

fn get_ready_animation() -> impl Animation<Item = f32> {
    anim::builder::key_frames([
        anim::KeyFrame::new(0.0).by_percent(0.0),
        anim::KeyFrame::new(GET_READY_FONT_SIZE)
            .by_percent(0.2)
            .easing(anim::easing::quad_ease().mode(anim::easing::EasingMode::Out)),
        anim::KeyFrame::new(GET_READY_FONT_SIZE).by_percent(0.9),
        anim::KeyFrame::new(0.0)
            .by_duration(Duration::from_millis(3000))
            .easing(anim::easing::quad_ease().mode(anim::easing::EasingMode::In)),
    ])
}

#[derive(Clone)]
struct CounterAnimationState {
    radius: f32,
    alpha: f32,
    text_size: f32,
}

fn counter_animation() -> impl Animation<Item = CounterAnimationState> {
    const COUNTER_ANIMATION_DURATION: u64 = 1000;
    let radius = anim::builder::key_frames([
        anim::KeyFrame::new(0.0).by_percent(0.0),
        anim::KeyFrame::new(COUNTER_RADIUS)
            .by_percent(0.4)
            .easing(anim::easing::quad_ease().mode(anim::easing::EasingMode::Out)),
        anim::KeyFrame::new(COUNTER_RADIUS).by_percent(0.8),
        anim::KeyFrame::new(0.0)
            .by_duration(Duration::from_millis(COUNTER_ANIMATION_DURATION))
            .easing(anim::easing::quad_ease().mode(anim::easing::EasingMode::In)),
    ]);
    let alpha = anim::builder::key_frames([
        anim::KeyFrame::new(0.0).by_percent(0.0),
        anim::KeyFrame::new(1.0)
            .by_percent(0.4)
            .easing(anim::easing::quad_ease()),
        anim::KeyFrame::new(1.0).by_duration(Duration::from_millis(COUNTER_ANIMATION_DURATION)),
    ]);
    let text_size = anim::builder::key_frames([
        anim::KeyFrame::new(0.0).by_percent(0.0),
        anim::KeyFrame::new(COUNTER_RADIUS)
            .by_percent(0.4)
            .easing(anim::easing::quad_ease().mode(anim::easing::EasingMode::Out)),
        anim::KeyFrame::new(COUNTER_RADIUS).by_percent(0.8),
        anim::KeyFrame::new(0.0)
            .by_duration(Duration::from_millis(COUNTER_ANIMATION_DURATION))
            .easing(anim::easing::quad_ease().mode(anim::easing::EasingMode::In)),
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
    anim::builder::key_frames([
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
    anim::builder::key_frames([
        anim::KeyFrame::new(1.0).by_percent(0.0),
        anim::KeyFrame::new(0.0)
            .by_percent(1.0)
            .easing(anim::easing::bounce_ease().mode(anim::easing::EasingMode::In)),
        anim::KeyFrame::new(1.0).by_duration(Duration::from_millis(FRAME_SIZE_ANIMATION_DURATION)),
    ])
}

enum FrameCaptureSequenceState {
    Counter(u16),
    Snap,
    FrameSize,
}

enum CaptureSequenceState {
    None,
    GetReady,
    FramesCapture(FrameCaptureSequenceState),
}

pub(crate) struct CameraScreen {
    feed: CameraFeed,
    config: Config,
    index: nokhwa::utils::CameraIndex,
    captured_frames: Vec<(image::ImageBuffer<Rgba<u8>, Vec<u8>>, Handle)>,

    capture_sequence_state: CaptureSequenceState,

    get_ready_timeline: Timeline<f32>,
    counter_timeline: Timeline<CounterAnimationState>,
    snap_timeline: Timeline<f32>,
    frame_size_timeline: Timeline<f32>,
}

#[derive(Clone, Debug)]
pub(crate) struct CameraScreenFlags {
    pub index: nokhwa::utils::CameraIndex,
    pub config: Config,
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
        let mut camera = Camera::new(flags.index.clone(), requested).unwrap();
        camera.open_stream().unwrap();
        let (feed, feed_command) = CameraFeed::new(camera, 48.into());
        (
            CameraScreen {
                feed,
                config: flags.config.clone(),
                index: flags.index,
                captured_frames: vec![],

                capture_sequence_state: CaptureSequenceState::None,

                get_ready_timeline: get_ready_animation().to_timeline(),
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
                match &self.capture_sequence_state {
                    CaptureSequenceState::None => {}
                    CaptureSequenceState::GetReady => {
                        self.get_ready_timeline.update();
                        if self.get_ready_timeline.status().is_completed() {
                            self.capture_sequence_state = CaptureSequenceState::FramesCapture(
                                FrameCaptureSequenceState::Counter(3),
                            );
                            self.counter_timeline.begin();
                        }
                    }
                    CaptureSequenceState::FramesCapture(frame_capture_sequence_state) => {
                        match frame_capture_sequence_state {
                            FrameCaptureSequenceState::Counter(value) => {
                                self.counter_timeline.update();
                                if self.counter_timeline.status().is_completed() {
                                    if value > &1 {
                                        self.capture_sequence_state =
                                            CaptureSequenceState::FramesCapture(
                                                FrameCaptureSequenceState::Counter(value - 1),
                                            );
                                        self.counter_timeline.begin();
                                    } else {
                                        // start the snap animation
                                        self.capture_sequence_state =
                                            CaptureSequenceState::FramesCapture(
                                                FrameCaptureSequenceState::Snap,
                                            );
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
                            FrameCaptureSequenceState::Snap => {
                                self.snap_timeline.update();
                                if self.snap_timeline.status().is_completed() {
                                    // start the frame size animation
                                    self.capture_sequence_state =
                                        CaptureSequenceState::FramesCapture(
                                            FrameCaptureSequenceState::FrameSize,
                                        );
                                }
                            }
                            FrameCaptureSequenceState::FrameSize => {
                                self.frame_size_timeline.update();
                                if self.frame_size_timeline.status().is_completed() {
                                    if self.captured_frames.len()
                                        < self.config.template.frames.len()
                                    {
                                        // start the next frame capture
                                        self.capture_sequence_state =
                                            CaptureSequenceState::FramesCapture(
                                                FrameCaptureSequenceState::Counter(3),
                                            );
                                        self.counter_timeline.begin();
                                    } else {
                                        // transition to the next screen if we're done
                                        let config = self.config.clone();
                                        let captured_frames = self
                                            .captured_frames
                                            .clone()
                                            .into_iter()
                                            .map(|frame| frame.0)
                                            .collect();
                                        let index = self.index.clone();
                                        return iced::Command::perform(
                                            async {
                                                super::ScreenFlags::GenerationScreenFlags(
                                                    super::generation_screen::GenerationScreenFlags {
                                                        config,
                                                        captured_frames,
                                                        index
                                                    },
                                                )
                                            },
                                            super::ScreenMessage::TransitionToScreen,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                iced::Command::none()
            }
            CameraScreenMessage::ImageCaptured(image) => {
                self.captured_frames.push((
                    image.clone(), // bad for performance, but we need to give a handle to iced to render the preview...
                    Handle::from_pixels(image.width(), image.height(), image.into_raw()),
                ));
                self.frame_size_timeline.begin();
                iced::Command::none()
            }
            CameraScreenMessage::CaptureButtonPressed => {
                self.capture_sequence_state = CaptureSequenceState::GetReady;
                self.get_ready_timeline.begin();
                iced::Command::none()
            }
        }
    }
    fn view(&self) -> Element<CameraScreenMessage> {
        let counter_animation_value = self.counter_timeline.value();
        let snap_animation_value = self.snap_timeline.value();
        let get_ready_size = self.get_ready_timeline.value();
        let frame_size_animation_value = self.frame_size_timeline.value();
        floating_element(
            container(
                Row::new()
                    .push(
                        floating_element(
                            floating_element(
                                self.feed.view().width(Length::Fill).height(Length::Fill),
                                camera_button().on_press(CameraScreenMessage::CaptureButtonPressed),
                            )
                            .hide(!matches!(
                                self.capture_sequence_state,
                                CaptureSequenceState::None
                            ))
                            .anchor(floating_element::Anchor::SouthEast)
                            .offset(8.0),
                            match self.capture_sequence_state {
                                CaptureSequenceState::FramesCapture(
                                    FrameCaptureSequenceState::Counter(counter),
                                ) => container(
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
                                CaptureSequenceState::GetReady => {
                                    container(text("Get ready").size(if get_ready_size > 0.0 {
                                        get_ready_size
                                    } else {
                                        f32::MIN_POSITIVE
                                    }))
                                    .width(Length::Fill)
                                    .height(Length::Fill)
                                    .align_x(iced::alignment::Horizontal::Center)
                                    .align_y(iced::alignment::Vertical::Center)
                                }
                                _ => container(space::Space::new(0, 0)),
                            }
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(iced::alignment::Horizontal::Center)
                            .align_y(iced::alignment::Vertical::Center),
                        )
                        .offset(0.0),
                    )
                    .push(element_strip_renderer(
                        "assets/template.png".into(),
                        &self.captured_frames,
                        &self.config.template,
                        if matches!(
                            self.capture_sequence_state,
                            CaptureSequenceState::FramesCapture(FrameCaptureSequenceState::Snap)
                        ) {
                            Some(snap_animation_value)
                        } else {
                            None
                        },
                        frame_size_animation_value,
                    ))
                    .spacing(20)
                    .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20),
            match self.capture_sequence_state {
                CaptureSequenceState::FramesCapture(FrameCaptureSequenceState::Snap) => {
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
                }
                _ => container(space::Space::new(0, 0)).width(0).height(0),
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
                || self.get_ready_timeline.status().is_animating()
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

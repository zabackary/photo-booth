use std::{rc::Rc, sync::Arc, time::Duration};

use crate::camera_feed::{CameraFeed, CameraMessage};
use anim::{Animation, Timeline};
use iced::{
    widget::{button, container, text, Row},
    Alignment, Element, Length,
};
use iced_aw::floating_element;
use image::Rgba;
use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{RequestedFormat, RequestedFormatType},
    Camera,
};
use tokio::sync::Mutex;

#[derive(Clone)]
struct CounterAnimationState {
    radius: f32,
    alpha: f32,
}

fn counter_animation() -> impl Animation<Item = CounterAnimationState> {
    const COUNTER_RADIUS: f32 = 40.0;
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
    CaptureFrame,
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
                    iced::Command::perform(async {}, |_| CameraScreenMessage::CaptureFrame)
                        .map(super::ScreenMessage::CameraScreenMessage)
                } else {
                    iced::Command::none()
                }
            }
            CameraScreenMessage::CaptureButtonPressed => iced::Command::perform(async {}, |_| {
                CameraScreenMessage::StartCaptureFrameAnimation
            })
            .map(super::ScreenMessage::CameraScreenMessage),
            CameraScreenMessage::CaptureFrame => iced::Command::none(),
            CameraScreenMessage::AllFramesCaptured => iced::Command::perform(async {}, |_| {
                super::ScreenMessage::TransitionToScreen(super::ScreenFlags::PrintingScreenFlags(
                    super::printing_screen::PrintingScreenFlags {},
                ))
            }),
        }
    }
    fn view(&self) -> Element<CameraScreenMessage> {
        container(
            Row::new()
                .push(floating_element(
                    self.feed.view().width(Length::Fill).height(Length::Fill),
                    container(text("test"))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(iced::alignment::Horizontal::Center)
                        .align_y(iced::alignment::Vertical::Center),
                ))
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
        self.feed
            .subscription()
            .map(CameraScreenMessage::CameraFeedMessage)
    }
}

impl Into<super::Screen> for CameraScreen {
    fn into(self) -> super::Screen {
        super::Screen::CameraScreen(self)
    }
}

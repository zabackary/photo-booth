mod border_radius;

use ::image::{ImageBuffer, Rgba};
use iced::widget::image;
use iced::{Command, Subscription};
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::Camera;
use std::sync::{Arc, Mutex};

use self::border_radius::BorderRadius;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CameraMessage {
    CaptureFrame,
    NewFrame(image::Handle),
}

/// Camera feed.
#[derive(Clone)]
pub struct CameraFeed {
    camera: Arc<Mutex<nokhwa::Camera>>,
    current_frame: Arc<Mutex<Option<image::Handle>>>,
    border_radius: BorderRadius,
}

fn frame_and_decode(camera: &mut Camera) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // TODO: it might be more performant to pre-allocate the buffer and use
    //       write_frame_to_buffer instead
    camera
        .frame()
        .expect("failed to capture a camera frame")
        .decode_image::<RgbAFormat>()
        .expect("failed to decode the camera frame")
}

impl CameraFeed {
    pub fn new(
        camera: nokhwa::Camera,
        border_radius: BorderRadius,
    ) -> (Self, Option<CameraMessage>) {
        (
            CameraFeed {
                camera: Arc::new(Mutex::new(camera)),
                current_frame: Arc::new(Mutex::new(None)),
                border_radius,
            },
            Some(CameraMessage::CaptureFrame),
        )
    }

    /// Take an image outside of the normal video capture cycle
    pub fn frame(&mut self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        frame_and_decode(&mut self.camera.lock().expect("failed to lock camera mutex"))
    }

    pub fn update(&mut self, message: CameraMessage) -> Command<CameraMessage> {
        match message {
            CameraMessage::CaptureFrame => {
                let cloned_camera = self.camera.clone();
                let border_radius = self.border_radius;
                Command::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            let mut frame = frame_and_decode(
                                &mut cloned_camera.lock().expect("failed to lock camera mutex"),
                            );
                            border_radius::round(&mut frame, &border_radius);
                            image::Handle::from_pixels(
                                frame.width(),
                                frame.height(),
                                frame.into_raw(),
                            )
                        })
                        .await
                        .unwrap()
                    },
                    CameraMessage::NewFrame,
                )
            }
            CameraMessage::NewFrame(data) => {
                *self.current_frame.lock().expect("failed to lock frame") = Some(data);
                Command::perform(async {}, |_| CameraMessage::CaptureFrame)
            }
        }
    }

    pub fn subscription(&self) -> Subscription<CameraMessage> {
        Subscription::none()
    }

    /// Get the image handle of the current frame.
    pub fn handle(&self) -> image::Handle {
        self.current_frame
            .lock()
            .expect("failed to lock frame")
            .clone()
            .unwrap_or_else(|| image::Handle::from_pixels(0, 0, vec![]))
    }

    /// Wrap the output of `frame_image` in an `Image` widget.
    pub fn view(&self) -> image::Image<image::Handle> {
        image::Image::new(self.handle())
    }
}

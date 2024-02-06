use iced::widget::image;
use iced::{Command, Subscription};
use nokhwa::pixel_format::RgbAFormat;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CameraMessage {
    CaptureFrame,
    NewFrame(image::Handle),
}

/// Camera feed.
pub struct CameraFeed {
    camera: Arc<Mutex<nokhwa::Camera>>,
    current_frame: Arc<Mutex<Option<image::Handle>>>,
}

impl CameraFeed {
    pub fn new(camera: nokhwa::Camera) -> (Self, Command<CameraMessage>) {
        (
            CameraFeed {
                camera: Arc::new(Mutex::new(camera)),
                current_frame: Arc::new(Mutex::new(None)),
            },
            Command::perform(async {}, |_| CameraMessage::CaptureFrame),
        )
    }

    pub fn update(&mut self, message: CameraMessage) -> Command<CameraMessage> {
        match message {
            CameraMessage::CaptureFrame => {
                let cloned_camera = self.camera.clone();
                Command::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            let frame = cloned_camera
                                .lock()
                                .unwrap()
                                .frame()
                                .unwrap()
                                .decode_image::<RgbAFormat>()
                                .unwrap();
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

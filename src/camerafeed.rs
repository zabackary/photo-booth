use ::image::{GenericImageView, ImageBuffer, Rgba};
use iced::widget::image;
use iced::{Command, Subscription};
use nokhwa::pixel_format::RgbAFormat;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraMessage {
    Tick,
}

/// Camera feed.
pub struct CameraFeed {
    width: u32,
    height: u32,
    framerate: u32,
    camera: nokhwa::Camera,

    frame: Arc<Mutex<Option<image::Handle>>>,
    // wait: mpsc::Receiver<()>,
}

impl CameraFeed {
    pub fn new(camera: nokhwa::Camera) -> Self {
        let format = camera.camera_format();
        CameraFeed {
            width: format.width(),
            height: format.height(),
            framerate: format.frame_rate(),
            camera,
            frame: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the size/resolution of the video as `(width, height)`.
    #[inline(always)]
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Get the framerate of the video as frames per second.
    #[inline(always)]
    pub fn framerate(&self) -> u32 {
        self.framerate
    }

    pub fn update(&mut self, message: CameraMessage) -> Command<CameraMessage> {
        match message {
            CameraMessage::Tick => {
                *self.frame.lock().expect("failed to lock frame") =
                    Some(image::Handle::from_pixels(
                        self.width,
                        self.height,
                        self.camera
                            .frame()
                            .unwrap()
                            .decode_image::<RgbAFormat>()
                            .unwrap()
                            .into_raw(),
                    ));
            }
        }
        Command::none()
    }

    pub fn subscription(&self) -> Subscription<CameraMessage> {
        iced::time::every(Duration::from_secs_f64(0.5 / self.framerate as f64))
            .map(|_| CameraMessage::Tick)
    }

    /// Get the image handle of the current frame.
    pub fn frame_image(&self) -> image::Handle {
        self.frame
            .lock()
            .expect("failed to lock frame")
            .clone()
            .unwrap_or_else(|| image::Handle::from_pixels(0, 0, vec![]))
    }

    /// Wrap the output of `frame_image` in an `Image` widget.
    pub fn frame_view(&self) -> image::Image<image::Handle> {
        image::Image::new(self.frame_image())
    }
}

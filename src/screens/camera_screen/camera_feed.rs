mod border_radius;

use iced::widget::image::Handle;
use iced::{Command, Subscription};
use image::RgbaImage;
use nokhwa::pixel_format::RgbAFormat;
use nokhwa::Camera;
use std::sync::{Arc, Mutex};

use self::border_radius::BorderRadius;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CameraMessage {
    CaptureFrame,
    NewFrame(Handle),
}

/// Camera feed.
#[derive(Clone)]
pub struct CameraFeed {
    camera: Arc<Mutex<nokhwa::Camera>>,
    current_frame: Arc<Mutex<Option<Handle>>>,
    border_radius: BorderRadius,
    mirror: bool,
    aspect_ratio: Option<f32>,
}

fn frame_and_decode(camera: &mut Camera) -> RgbaImage {
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
        mirror: bool,
        aspect_ratio: Option<f32>,
    ) -> (Self, Option<CameraMessage>) {
        (
            CameraFeed {
                camera: Arc::new(Mutex::new(camera)),
                current_frame: Arc::new(Mutex::new(None)),
                border_radius,
                mirror,
                aspect_ratio,
            },
            Some(CameraMessage::CaptureFrame),
        )
    }

    /// Take an image outside of the normal video capture cycle
    pub fn frame(&mut self) -> RgbaImage {
        frame_and_decode(&mut self.camera.lock().expect("failed to lock camera mutex"))
    }

    pub fn update(&mut self, message: CameraMessage) -> Command<CameraMessage> {
        match message {
            CameraMessage::CaptureFrame => {
                let cloned_camera = self.camera.clone();
                let border_radius = self.border_radius;
                let aspect_ratio = self.aspect_ratio;
                let mirror = self.mirror;
                Command::perform(
                    async move {
                        tokio::task::spawn_blocking(move || {
                            let mut frame = frame_and_decode(
                                &mut cloned_camera.lock().expect("failed to lock camera mutex"),
                            );

                            // crop the frame to meet the aspect ratio
                            let mut frame = if let Some(aspect_ratio) = aspect_ratio {
                                let frame_aspect_ratio =
                                    frame.width() as f32 / frame.height() as f32;
                                let new_width;
                                let new_height;
                                let left_offset;
                                let top_offset;
                                if aspect_ratio < frame_aspect_ratio {
                                    // trim off left and right
                                    new_height = frame.height();
                                    new_width = (frame.height() as f32 * aspect_ratio) as u32;
                                    left_offset = (frame.width() - new_width) / 2;
                                    top_offset = 0;
                                } else if aspect_ratio > frame_aspect_ratio {
                                    // trim off top and bottom
                                    new_width = frame.width();
                                    new_height = (frame.width() as f32 / aspect_ratio) as u32;
                                    top_offset = (frame.height() - new_height) / 2;
                                    left_offset = 0;
                                } else {
                                    // perfect aspect ratio!
                                    new_width = frame.width();
                                    new_height = frame.height();
                                    top_offset = 0;
                                    left_offset = 0;
                                }
                                image::imageops::crop(
                                    &mut frame,
                                    left_offset,
                                    top_offset,
                                    new_width,
                                    new_height,
                                )
                                .to_image() // this might be pricy...
                            } else {
                                frame
                            };

                            // mirror the frame
                            if mirror {
                                image::imageops::flip_horizontal_in_place(&mut frame);
                            }

                            // apply border radius
                            border_radius::round(&mut frame, &border_radius);

                            // output a handle
                            Handle::from_pixels(frame.width(), frame.height(), frame.into_raw())
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
    pub fn handle(&self) -> Handle {
        self.current_frame
            .lock()
            .expect("failed to lock frame")
            .clone()
            .unwrap_or_else(|| Handle::from_pixels(0, 0, vec![]))
    }

    /// Wrap the output of `frame_image` in an `Image` widget.
    pub fn view(&self) -> iced::widget::image::Image<Handle> {
        iced::widget::Image::new(self.handle())
    }
}

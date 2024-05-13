use iced::{
    widget::{container, image::Handle, Responsive, Space},
    Color, Length,
};
use iced_aw::floating_element;
use image::Rgba;

use crate::config::Template;

/// # Panics
///
/// Panics if the captured_frames count exceeds the amount in the template.
pub(super) fn element_strip_renderer<'a>(
    handle: iced::widget::image::Handle,
    captured_frames: &'a Vec<(image::ImageBuffer<Rgba<u8>, Vec<u8>>, Handle)>,
    template: &'a Template,
    snap_animation_value: Option<f32>,
    frame_size_animation_value: f32,
) -> iced::Element<'a, super::CameraScreenMessage> {
    if captured_frames.len() > template.frames.len() {
        panic!("captured_frames count exceeds number of frames in template");
    }
    floating_element(
        iced::widget::Image::new(handle),
        Responsive::new(move |size| {
            let mut element: iced::Element<'a, super::CameraScreenMessage> =
                container(Space::new(0, 0))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into();
            let x_factor = size.width / template.width;
            let y_factor = size.height / template.height;
            for (i, (_, frame_handle)) in captured_frames.iter().enumerate() {
                let template_frame = &template.frames[i];
                let is_last = i == captured_frames.len() - 1;
                let animation_factor = if is_last {
                    frame_size_animation_value
                } else {
                    1.0
                };
                // same as above for the .clone()
                let frame = iced::widget::Image::new(frame_handle.clone())
                    .content_fit(iced::ContentFit::Cover)
                    .width(template_frame.width * x_factor * animation_factor)
                    .height(template_frame.height * y_factor * animation_factor);
                element = floating_element(
                    element,
                    floating_element(
                        frame,
                        container(Space::new(0, 0))
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .style(container::Appearance::default().with_background(
                                Color::from_rgba8(
                                    255,
                                    255,
                                    255,
                                    snap_animation_value.unwrap_or(0.0),
                                ),
                            )),
                    )
                    .offset(0.0),
                )
                .anchor(floating_element::Anchor::NorthWest)
                .offset(floating_element::Offset {
                    x: template_frame.x * x_factor
                        + template_frame.width * x_factor * (1.0 - animation_factor) / 2.0,
                    y: template_frame.y * y_factor
                        + template_frame.height * y_factor * (1.0 - animation_factor) / 2.0,
                })
                .into();
            }
            element
        }),
    )
    .offset(0.0)
    .into()
}

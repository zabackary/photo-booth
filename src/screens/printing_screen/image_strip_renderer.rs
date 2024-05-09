use image::{imageops, GenericImageView};

use crate::config::Template;

/// # Panics
///
/// Panics if the captured_frames count exceeds the amount in the template.
pub(super) fn image_strip_renderer<'a>(
    mut background: image::RgbaImage,
    captured_frames: &'a Vec<image::RgbaImage>,
    template: &'a Template,
) -> image::RgbaImage {
    if captured_frames.len() > template.frames.len() {
        panic!("captured_frames count exceeds number of frames in template");
    }
    for (i, frame) in captured_frames.iter().enumerate() {
        let template_frame = &template.frames[i];

        // crop the frame to the template
        let template_aspect_ratio = template_frame.width / template_frame.height;
        let frame_aspect_ratio = frame.width() as f32 / frame.height() as f32;
        let cropped_frame = if template_aspect_ratio < frame_aspect_ratio {
            // trim off left and right
            let new_height = frame.height();
            let new_width = (frame.height() as f32 * template_aspect_ratio) as u32;
            let left_offset = (frame.width() - new_width) / 2;
            imageops::crop_imm(frame, left_offset, 0, new_width, new_height)
        } else if template_aspect_ratio > frame_aspect_ratio {
            // trim off top and bottom
            let new_width = frame.width();
            let new_height = (frame.width() as f32 / template_aspect_ratio) as u32;
            let top_offset = (frame.height() - new_height) / 2;
            imageops::crop_imm(frame, 0, top_offset, new_width, new_height)
        } else {
            // perfect aspect ratio!
            frame.view(0, 0, frame.width(), frame.height())
        };

        // resize the frame
        let resized_frame = imageops::resize(
            &cropped_frame.to_image(),
            template_frame.width as u32,
            template_frame.height as u32,
            imageops::FilterType::Lanczos3,
        );

        imageops::overlay(
            &mut background,
            &resized_frame,
            template_frame.x as i64,
            template_frame.y as i64,
        )
    }
    background
}

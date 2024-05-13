use iced::{border::Radius, widget::container::StyleSheet, Border, Color};

#[derive(Debug, Clone, Copy, Default)]
pub struct OutlinedContainerStyle {}

impl StyleSheet for OutlinedContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        let palette = style.extended_palette();
        iced::widget::container::Appearance {
            border: Border {
                radius: Radius::from(8),
                width: 2.0,
                color: palette.primary.weak.color,
            },
            text_color: Some(palette.primary.weak.text),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RoundedBoxContainerStyle {}

impl StyleSheet for RoundedBoxContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        let palette = style.extended_palette();
        iced::widget::container::Appearance {
            border: Border {
                radius: Radius::from(12),
                width: 2.0,
                color: Color::TRANSPARENT,
            },
            background: Some(palette.background.weak.color.into()),
            text_color: Some(palette.background.weak.text),
            ..Default::default()
        }
    }
}

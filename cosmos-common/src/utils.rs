use cosmic::{
    cosmic_theme::Spacing, iced::{self, Alignment, Background, Border, Color, Length, Shadow}, iced_widget, widget::{self}, Element, Theme
};

pub fn labelled_info<'a, Message: 'static + Clone>(
    label: impl Into<String>,
    info: impl Into<String>,
) -> Element<'a, Message> {
    iced_widget::row![
        widget::text(label.into())
            .align_x(Alignment::End)
            .width(Length::FillPortion(1)),
           // .class(theme::Text::Color(cosmic::theme::system_preference().cosmic().text_button.base.color.into())),
        widget::text(info.into()).width(Length::FillPortion(3)),
    ]
    .spacing(Spacing::default().space_s)
    .into()
}

fn alert<'a, Message: 'static + Clone>(
    message: impl Into<String>,
    on_close: Message,
    style: fn(&Theme) -> widget::container::Style,
) -> widget::Container<'a, Message, Theme> {
    widget::warning(message.into())
        .on_close(on_close)
        .into_widget()
        .style(style)
}

pub fn warning<'a, Message: 'static + Clone>(
    message: impl Into<String>,
    on_close: Message,
) -> widget::Container<'a, Message, Theme> {
    alert(message, on_close, warning_style)
}

pub fn error<'a, Message: 'static + Clone>(
    message: impl Into<String>,
    on_close: Message,
) -> widget::Container<'a, Message, Theme> {
    alert(message, on_close, error_style)
}

pub fn success<'a, Message: 'static + Clone>(
    message: impl Into<String>,
    on_close: Message,
) -> widget::Container<'a, Message, Theme> {
    alert(message, on_close, success_style)
}

pub fn info<'a, Message: 'static + Clone>(
    message: impl Into<String>,
    on_close: Message,
) -> widget::Container<'a, Message, Theme> {
    alert(message, on_close, info_style)
}

pub fn warning_style(theme: &Theme) -> widget::container::Style {
    let cosmic = theme.cosmic();
    widget::container::Style {
        icon_color: Some(theme.cosmic().warning.on.into()),
        text_color: Some(theme.cosmic().warning.on.into()),
        background: Some(Background::Color(theme.cosmic().warning_color().into())),
        border: Border {
            color: Color::TRANSPARENT,
            width: 1.0,
            radius: cosmic.corner_radii.radius_0.into(),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: iced::Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
    }
}

pub fn error_style(theme: &Theme) -> widget::container::Style {
    let cosmic = theme.cosmic();
    widget::container::Style {
        icon_color: Some(theme.cosmic().destructive.on.into()),
        text_color: Some(theme.cosmic().destructive.on.into()),
        background: Some(Background::Color(theme.cosmic().destructive_color().into())),
        border: Border {
            color: Color::TRANSPARENT,
            width: 1.0,
            radius: cosmic.corner_radii.radius_0.into(),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: iced::Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
    }
}

pub fn success_style(theme: &Theme) -> widget::container::Style {
    let cosmic = theme.cosmic();
    widget::container::Style {
        icon_color: Some(theme.cosmic().success.on.into()),
        text_color: Some(theme.cosmic().success.on.into()),
        background: Some(Background::Color(theme.cosmic().success_color().into())),
        border: Border {
            color: Color::TRANSPARENT,
            width: 1.0,
            radius: cosmic.corner_radii.radius_0.into(),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: iced::Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
    }
}

pub fn info_style(theme: &Theme) -> widget::container::Style {
    let cosmic = theme.cosmic();
    widget::container::Style {
        icon_color: Some(theme.cosmic().accent.on.into()),
        text_color: Some(theme.cosmic().accent.on.into()),
        background: Some(Background::Color(theme.cosmic().accent_color().into())),
        border: Border {
            color: Color::TRANSPARENT,
            width: 1.0,
            radius: cosmic.corner_radii.radius_0.into(),
        },
        shadow: Shadow {
            color: Color::TRANSPARENT,
            offset: iced::Vector::new(0.0, 0.0),
            blur_radius: 0.0,
        },
    }
}

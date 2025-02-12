use cosmic::{
    self,
    iced::{self, Alignment, Background, Border, Color, Length, Shadow},
    widget::{
        self,
    }, Theme,
};

use crate::app::Message;


pub fn warning(message: impl Into<String>, on_close: Message) -> widget::Container<'static, Message, Theme> {
    widget::warning(message.into())
        .on_close(on_close)
        .into_widget()
        .style(warning_style)
}

pub fn error(message: impl Into<String>, on_close: Message) -> widget::Container<'static, Message, Theme> {
    widget::warning(message.into())
    .on_close(on_close)
    .into_widget()
        .style(error_style)
   
}

pub fn success(message: impl Into<String>, on_close: Message) -> widget::Container<'static, Message, Theme> {
    widget::warning(message.into())
    .on_close(on_close)
    .into_widget()
        .style(success_style)
}


pub fn info(message: impl Into<String>, on_close: Message) -> widget::Container<'static, Message, Theme> {
    widget::warning(message.into())
    .on_close(on_close)
    .into_widget()
        .style(info_style)
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

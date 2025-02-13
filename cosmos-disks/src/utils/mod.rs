use cosmic::{
    cosmic_theme::palette::WithAlpha,
    iced::{
        Alignment, Background, Length,
        Shadow,
    },
    iced_widget::{
        self,
        column, row,
    },
    widget::{
        self,
        container, icon,
        text::{caption, caption_heading},
    }, Element,
};
use cosmos_common::bytes_to_pretty;
use cosmos_dbus::disks::{DriveModel, PartitionModel};

use crate::app::Message;

#[derive(Debug, Clone)]
pub enum VolumesControlMessage {
    SegmentSelected(usize),
}

pub struct VolumesControl {
    pub selected_segment: usize,
    pub segments: Vec<Segment>,
    #[allow(dead_code)]
    pub model: DriveModel,
}

pub struct Segment {
    pub label: String,
    pub name: String,
    pub partition_type: String,
    pub size: u64,
    #[allow(dead_code)]
    pub offset: u64,
    pub state: bool,
    pub is_free_space: bool,
    pub width: u16,
    pub partition: Option<PartitionModel>,
}

#[derive(Copy, Clone)]
pub enum ToggleState {
    Normal,
    Active,
    Disabled,
    Hovered,
    Pressed,
}

impl ToggleState {
    pub fn active_or(selected: &bool, toggle: ToggleState) -> Self {
        if *selected {
            ToggleState::Active
        } else {
            toggle
        }
    }
}

impl Segment {
    pub fn free_space(offset: u64, size: u64) -> Self {
        Self {
            label: "Free Space".into(),
            name: "".into(),
            partition_type: "".into(),
            size,
            offset,
            state: false,
            is_free_space: true,
            width: 0,
            partition: None,
        }
    }

    pub fn new(partition: &PartitionModel) -> Self {
        Self {
            label: "LABEL WILL GO HERE".into(),
            name: partition.pretty_name(),
            partition_type: partition.partition_type.clone(),
            size: partition.size,
            offset: partition.offset,
            state: false,
            is_free_space: false,
            width: 0,
            partition: Some(partition.clone()),
        }
    }

    pub fn get_segments(drive: &DriveModel) -> Vec<Segment> {
        if drive.partitions.len() == 0 {
            return vec![Segment::free_space(0, drive.size)];
        }

        let mut ordered_partitions = drive.partitions.clone();

        ordered_partitions.sort_by(|a, b| a.offset.cmp(&b.offset));

        let mut segments = vec![];
        let mut current_offset = ordered_partitions.first().unwrap().offset; //TODO: HANDLE UNWRAP

        if current_offset > 1048576 {
            //TODO: There seems to be 1024KB at the start of all drives.
            //      We need to make sure this is ALWAYS present, or the same size.
            current_offset = 0;
        }

        for p in ordered_partitions {
            if p.offset > current_offset {
                //add in a free space segment.
                segments.push(Segment::free_space(
                    current_offset,
                    p.offset - current_offset,
                ));
                current_offset = p.offset;
            }

            segments.push(Segment::new(&p));
            current_offset += p.size;
        }

        //TODO: Hack to hide weird end portion... find out what this is.
        if current_offset < drive.size - 5242880 {
            segments.push(Segment::free_space(
                current_offset,
                drive.size - current_offset,
            ));
        }

        //Figure out Portion value
        segments.iter_mut().for_each(|s| {
            s.width = (((s.size as f64 / drive.size as f64) * 1000.).log10().ceil() as u16).max(1);
        });

        segments
    }

    pub fn get_segment_control<'a>(&self) -> Element<'a, Message> {
        if self.is_free_space {
            container(
                iced_widget::column![
                    caption_heading("Free space").center(),
                    caption(bytes_to_pretty(&self.size, false)).center()
                ]
                .spacing(5)
                .align_x(Alignment::Center),
            )
            .padding(5)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
        } else {
            container(
                iced_widget::column![
                    caption_heading(self.name.clone()).center(),
                    caption("LABEL WILL GO HERE").center(),
                    caption(self.partition_type.clone()).center(),
                    caption(bytes_to_pretty(&self.size, false)).center()
                ]
                .spacing(5)
                .align_x(Alignment::Center),
            )
            .padding(5)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
        }
    }
}


impl VolumesControl {
    pub fn new(model: DriveModel) -> Self {
        let mut segments: Vec<Segment> = Segment::get_segments(&model);
        segments.first_mut().unwrap().state = true; //TODO: HANDLE UNWRAP.

        Self {
            model,
            selected_segment: 0,
            segments: segments,
        }
    }

    pub fn update(&mut self, message: VolumesControlMessage) {
        match message {
            VolumesControlMessage::SegmentSelected(index) => {
                self.selected_segment = index;
                self.segments.iter_mut().for_each(|s| s.state = false);
                self.segments.get_mut(index).unwrap().state = true;
            }
        }
    }

    pub fn view<'a>(&self) -> Element<'a, Message> {
        let segment_buttons: Vec<Element<'a, Message>> = self
            .segments
            .iter()
            .enumerate()
            .map(|(index, segment)| {
                let active_state = ToggleState::active_or(&segment.state, ToggleState::Normal);
                let hovered_state = ToggleState::active_or(&segment.state, ToggleState::Hovered);

                cosmic::widget::button::custom(segment.get_segment_control())
                    .on_press(Message::VolumesMessage(
                        VolumesControlMessage::SegmentSelected(index),
                    ))
                    .class(cosmic::theme::Button::Custom {
                        active: Box::new(move |_b, theme| get_button_style(active_state, theme)),
                        disabled: Box::new(|theme| get_button_style(ToggleState::Disabled, theme)),
                        hovered: Box::new(move |_, theme| get_button_style(hovered_state, theme)),
                        pressed: Box::new(|_, theme| get_button_style(ToggleState::Pressed, theme)),
                    })
                    .height(Length::Fixed(100.))
                    .width(Length::FillPortion(segment.width))
                    .into()
            })
            .collect();

        let selected = self.segments.get(self.selected_segment).unwrap(); //TODO: Handle unwrap
        let play_pause_icon = match &selected.partition {
            Some(p) => {
                match p.usage //TODO: More solid check than using the output of df to see if mounted.
            {
                Some(_) => "media-playback-stop-symbolic",
                None => "media-playback-start-symbolic",
            }
            }
            None => "media-playback-start-symbolic",
        };

        container(
            column![
                cosmic::widget::Row::from_vec(segment_buttons)
                    .spacing(10)
                    .width(Length::Fill),
                row![
                    widget::button::custom(icon::from_name(play_pause_icon)),
                    widget::button::custom(icon::from_name("edit-find-symbolic")),
                    widget::horizontal_space(),
                    widget::button::custom(icon::from_name("edit-delete-symbolic")),
                ] //TODO: Get better icons
            ]
            .spacing(10),
        )
        .width(Length::Fill)
        .padding(10)
        .class(cosmic::style::Container::Card)
        .into()
    }
}

fn get_button_style(
    state: ToggleState,
    theme: &cosmic::theme::Theme,
) -> cosmic::widget::button::Style {
    let mut base = cosmic::widget::button::Style {
        shadow_offset: Shadow::default().offset,
        background: Some(cosmic::iced::Background::Color(theme.cosmic().primary.base.into())),// Some(cosmic::iced::Background::Color(Color::TRANSPARENT)),
        overlay: None,
        border_radius: (theme.cosmic().corner_radii.radius_xs).into(),
        border_width: 0.,
        border_color: theme.cosmic().primary.base.into(),
        outline_width: 2.,
        outline_color: theme.cosmic().primary.base.into(),
        icon_color: None,
        text_color: None,
    };

    match state {
        ToggleState::Normal => {}
        ToggleState::Active => {
            base.border_color = theme.cosmic().accent_color().into();
            base.outline_color = theme.cosmic().accent_color().into();
            base.background = Some(Background::Color(
                theme.cosmic().accent_color().with_alpha(0.2).into(),
            ));
        }
        ToggleState::Disabled => todo!(),
        ToggleState::Hovered => {
            base.text_color = Some(theme.cosmic().accent_button.base.into());
            base.background = Some(Background::Color(theme.cosmic().button.hover.into()));
        }
        ToggleState::Pressed => {
            base.border_color = theme.cosmic().accent_color().into();
            base.outline_color = theme.cosmic().accent_color().into();
        }
    }

    base
}

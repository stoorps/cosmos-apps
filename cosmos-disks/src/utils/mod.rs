use cosmic::{
    iced::Alignment,
    iced_widget::{self, button},
    widget::{self, container, segmented_button::SingleSelectModel, text, Button},
    Element, Task,
};
use cosmos_common::bytes_to_pretty;
use cosmos_dbus::udisks::{DriveModel, PartitionModel};

#[derive(Debug, Clone)]
enum VolumesControlMessage {
    SegmentSelected(usize),
    CustomButtonClicked(usize), // Example message for custom button
}

pub struct VolumesControl<'a> {
    selected_segment: usize,
    segments: Vec<Segment<'a>>,
    model: &'a DriveModel,
}

struct Segment<'a> {
    //content: Element<'a, VolumesControlMessage>, // Store the custom content
    label: String,
    name: String,
    partition_type: String,
    size: u64,
    offset: u64,
    state: bool,
    is_free_space: bool, // State for the underlying button
    width: u32,
    view: Option<Element<'a, VolumesControlMessage>>,
}

impl<'a> Segment<'a> {
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
            view: None,
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
            view: None,
        }
    }

    pub fn get_segments(drive: &DriveModel) -> Vec<Segment> {
        if drive.partitions.len() == 0 {
            return vec![Segment::free_space(0, drive.size)];
        }

        let mut ordered_partitions = drive.partitions.clone();

        ordered_partitions.sort_by(|a, b| a.offset.cmp(&b.offset));

        let mut segments = vec![];
        let mut current_offset = 0u64;
        for mut p in ordered_partitions {
            if p.offset > current_offset {
                //add in a free space segment.
                segments.push(Segment::free_space(
                    current_offset,
                    p.offset - current_offset,
                ));
                current_offset = p.offset;
            }

            segments.push(Segment::new(&p));
            p.offset = p.size;
        }

        if current_offset < drive.size {
            segments.push(Segment::free_space(
                current_offset,
                drive.size - current_offset,
            ));
        }

        segments
            .iter_mut()
            .for_each(|s| s.width = ((drive.size / s.size) * 100) as u32);

        segments
    }

    pub fn get_segment_control(&self) -> Element<'a, VolumesControlMessage> {
        if self.is_free_space {
            container(
                iced_widget::column![text("Free space"), text(bytes_to_pretty(&self.size, false))]
                    .align_x(Alignment::Center),
            )
            .into()
        } else {
            container(
                iced_widget::column![
                    text("LABEL WILL GO HERE"),
                    text(self.name.clone()),
                    text(self.partition_type.clone()),
                    text(bytes_to_pretty(&self.size, false))
                ]
                .align_x(Alignment::Center),
            )
            .into()
        }
    }
}

pub type VolumesModel = Vec<PartitionModel>;

impl<'a> VolumesControl<'a> {
    pub fn new(model: &'a DriveModel) -> Self {
        let drive_size = model.size;

        let mut segments: Vec<Segment<'a>> = Segment::get_segments(model);
        segments.first_mut().unwrap().state = true; //TODO: HANDLE UNWRAP.

        Self {
            model,
            selected_segment: 0,
            segments: segments,
        }
    }

    pub fn update(&mut self, message: VolumesControlMessage) -> Task<VolumesControlMessage> {
        match message {
            VolumesControlMessage::SegmentSelected(index) => {
                self.selected_segment = index;
                Task::none()
            }
            VolumesControlMessage::CustomButtonClicked(index) => {
                // Handle the custom button click within the segment
                println!("Button in segment {} clicked!", index);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'a, VolumesControlMessage> {
        let segment_buttons: Vec<Element<'a, VolumesControlMessage>> = self
            .segments
            .iter()
            .enumerate()
            .map(|(index, segment)| {
                widget::button::custom(segment.get_segment_control())
                    .on_press(VolumesControlMessage::SegmentSelected(index))
                    // .style(|theme, status| {
                    //     status
                    // }) // Apply a custom style
                    .into()
            })
            .collect();

        cosmic::widget::Row::from_vec(segment_buttons).into()
    }
}

// Custom button style
struct SegmentButtonStyle {
    is_selected: bool,
}

// impl Style for SegmentButtonStyle {
//     fn style(&self) -> button::Style {
//         if self.is_selected {
//             button::Style {
//                 background: iced::Background::Color(iced::Color::from_rgb(0.7, 0.7, 0.7)), // Example
//                 text_color: iced::Color::WHITE,
//                 ..Default::default()
//             }
//         } else {
//             button::Style {
//                 background: iced::Background::Color(iced::Color::WHITE), // Example
//                 text_color: iced::Color::BLACK,
//                 ..Default::default()
//             }
//         }
//     }
// }

use super::volumes::CreateMessage;
use crate::app::Message;
use cosmic::{
    iced::Length,
    iced_widget,
    widget::{
        button, checkbox, container, dialog, dropdown, slider, spin_button, text_input, toggler,
    },
    Element,
};
use cosmos_dbus::disks::{CreatePartitionInfo, PARTITION_NAMES};
use std::borrow::Cow;

pub fn confirmation<'a>(
    title: impl Into<Cow<'a, str>>,
    prompt: impl Into<Cow<'a, str>>,
    ok_message: Message,
    cancel_message: Option<Message>,
) -> Element<'a, Message> {
    let mut dialog = dialog::dialog()
        .title(title)
        .body(prompt)
        .primary_action(button::destructive("Ok").on_press(ok_message.into()));

    match cancel_message {
        Some(c) => dialog = dialog.secondary_action(button::standard("Cancel").on_press(c.into())),
        None => {}
    };

    dialog.into()
}

pub fn add_partition<'a>(create: CreatePartitionInfo) -> Element<'a, Message> {
    let len = create.max_size as f64;

    let size = create.size as f64;
    let free = len - size;

    let create_clone = create.clone();

    let content = iced_widget::column![
        text_input("Volume name", create_clone.name)
            .label("Volume Name")
            .on_input(|t| CreateMessage::NameUpdate(t).into()),
        slider((0.0..=len), size, |v| CreateMessage::SizeUpdate(v as u64)
            .into()),
        container(spin_button("Partition Size", size, 1., 0., len, |v| {
            CreateMessage::SizeUpdate(v as u64).into()
        }))
        .width(Length::Fill),
        spin_button("Free Space Following", free, 1., 0., len, move |v| {
            CreateMessage::SizeUpdate((len - v) as u64).into()
        }),
        toggler(create_clone.erase)
            .label("Erase")
            .on_toggle(|v| CreateMessage::EraseUpdate(v).into()),
        dropdown(
            &PARTITION_NAMES,
            Some(create_clone.selected_partitition_type),
            |v| CreateMessage::PartitionTypeUpdate(v).into()
        ),
        checkbox("Password Protected", false),
        text_input::secure_input("", create_clone.password, None, true)
            .label("Password")
            .on_input(|v| CreateMessage::PasswordUpdate(v).into()),
        text_input::secure_input("", create_clone.confirmed_password, None, true)
            .label("Confirm")
            .on_input(|v| CreateMessage::ConfirmedPasswordUpdate(v).into()),
    ];

    let mut continue_button = button::destructive("Continue");

    // if create.can_continue
    //{
    continue_button = continue_button.on_press(CreateMessage::Partition(create).into());
    //}

    dialog::dialog()
        .title("Create Partition")
        .control(content)
        .primary_action(continue_button)
        .secondary_action(button::standard("Cancel").on_press(CreateMessage::Cancel.into()))
        .into()
}

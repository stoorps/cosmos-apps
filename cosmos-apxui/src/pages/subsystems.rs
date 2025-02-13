use super::PageModel;
use crate::app::Message;
use apx_shim::Subsystem;
use cosmic::{
    self,
    cosmic_theme::{self, Spacing},
    iced::Length,
    iced_wgpu::graphics::text,
    iced_widget, theme,
    widget::{
        self, nav_bar,
        segmented_button::{self, Entity, SingleSelect, VerticalSegmentedButton},
        Toasts,
    },
};

use tracing::{error, warn};

pub struct SubSystemsModel {
    nav_bar: nav_bar::Model,
    sub_actions: segmented_button::Model<SingleSelect>,
    destructive_actions: segmented_button::Model<SingleSelect>,
    error_status: Option<String>,
    action_status: Option<String>,
    dialog_string: Option<String>,
}

impl SubSystemsModel {
    pub fn new() -> Self {
        let mut sub_actions = segmented_button::Model::<SingleSelect>::default();

        sub_actions
            .insert()
            .text("Start subsystem")
            .data::<SubsystemMessage>(SubsystemMessage::Start);
        sub_actions
            .insert()
            .text("Stop subsystem")
            .data::<SubsystemMessage>(SubsystemMessage::Stop);
        sub_actions
            .insert()
            .text("Autoremove packages")
            .data::<SubsystemMessage>(SubsystemMessage::Autoremove);
        sub_actions
            .insert()
            .text("Clean Package Manager Cache")
            .data::<SubsystemMessage>(SubsystemMessage::CleanPackageManagerCache);

        let mut destructive_actions = segmented_button::Model::<SingleSelect>::default();

        destructive_actions
            .insert()
            .text("Reset subsystem")
            .data::<SubsystemMessage>(SubsystemMessage::Reset);
        destructive_actions
            .insert()
            .text("Delete subsystem")
            .data::<SubsystemMessage>(SubsystemMessage::Delete);

        Self {
            nav_bar: nav_bar::Model::default(),
            error_status: None,
            action_status: None,
            dialog_string: None,
            sub_actions,
            destructive_actions,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SubsystemMessage {
    Reset,
    Start,
    Stop,
    Autoremove,
    CleanPackageManagerCache,
    Delete,
    HandleSubButton(Entity),
    HandleDestButton(Entity),
    CloseError,
    CloseAction,
}

impl Into<Message> for SubsystemMessage {
    fn into(self) -> Message {
        Message::Subsystem(self)
    }
}

fn labelled_info(
    label: impl Into<String>,
    info: impl Into<String>,
) -> cosmic::Element<'static, Message> {
    widget::Column::new()
        .push(widget::text::heading(label.into()))
        .push(widget::text::body(info.into()))
        .spacing(5.)
        .into()
}

impl PageModel for SubSystemsModel {
    fn view(&self) -> cosmic::Element<'_, Message> {
        let data = self.nav_bar.active_data::<Subsystem>();
        let mut content = Vec::new();

        if data.is_none() {
            return widget::Column::new()
                .push(widget::Text::new("No subsystem selected").size(24))
                .into();
        }

        let data = data.unwrap();

        content.push(
            widget::Text::new(&data.name)
                .size(24)
                .width(Length::Fill)
                .into(),
        );

        if let Some(error) = &self.error_status {
            content.push(cosmos_common::error(error, SubsystemMessage::CloseError.into()).into());
        }

        if let Some(string) = &self.dialog_string {
            content.push(
                widget::Popover::new(
                    widget::dialog()
                        .title("Dialog")
                        .body(string)
                        .primary_action(
                            widget::button::destructive("Ok")
                                .on_press(SubsystemMessage::CloseAction.into()),
                        ),
                )
                .modal(true)
                .position(widget::popover::Position::Center)
                .into(),
            );
        }

        // if let Some(action) = &self.action_status {
        //     content.push(widget::toaster(&self.toasts,widget::text(action)).into());
        // }

        content.push(
            iced_widget::column![iced_widget::scrollable(
                iced_widget::column![
                    widget::Text::new("Details").size(18),
                    widget::Container::new(
                        iced_widget::column![
                            labelled_info("Status", &data.status),
                            labelled_info("Stack", &data.stack.name),
                            labelled_info("Package Manager", &data.stack.package_manager),
                            //TODO: Exported programs
                        ]
                        .spacing(20)
                        .padding(20)
                    )
                    .style(|_| theme::Container::primary(&cosmic_theme::Theme::default()))
                    .width(Length::Fill),
                    widget::Text::new("Subsystem actions").size(18),
                    widget::Container::new(
                        VerticalSegmentedButton::new(&self.sub_actions)
                            .button_height(32)
                            .button_padding([8, 16, 8, 16])
                            .button_spacing(8)
                            .width(Length::Fill)
                            .on_activate(|id| SubsystemMessage::HandleSubButton(id).into())
                            .style(theme::SegmentedButton::TabBar)
                            .padding(20)
                    )
                    .style(|_| theme::Container::primary(&cosmic_theme::Theme::default())),
                    widget::Text::new("Destructive Actions").size(18),
                    widget::Container::new(
                        VerticalSegmentedButton::new(&self.destructive_actions)
                            .button_height(32)
                            .button_padding([8, 16, 8, 16])
                            .button_spacing(8)
                            .width(Length::Fill)
                            .on_activate(|id| SubsystemMessage::HandleDestButton(id).into())
                            .style(theme::SegmentedButton::TabBar)
                            .padding(20)
                    )
                    .style(|_| theme::Container::primary(&cosmic_theme::Theme::default())),
                ]
                .spacing(Spacing::default().space_xs)
                .padding([20, 20, 0, 0])
            )
            .height(Length::Fill),]
            .into(),
        );

        iced_widget::column(content).into()
    }

    fn current_items(&self) -> &nav_bar::Model {
        &self.nav_bar
    }

    fn on_select(&mut self, item: widget::segmented_button::Entity) {
        self.nav_bar.activate(item);
    }

    fn update_items(&mut self) {
        let data = apx_shim::Subsystem::get_all();

        self.nav_bar = match data {
            Ok(data) => {
                let mut items = nav_bar::Model::default();
                for item in data {
                    items
                        .insert()
                        .text(item.name.clone())
                        .data::<Subsystem>(item);
                }
                items
            }
            Err(_) => nav_bar::Model::default(),
        };
    }

    fn on_message(&mut self, message: Message) {
        let data = match self.nav_bar.active_data_mut::<Subsystem>() {
            Some(data) => data,
            None => {
                warn!("No active data found");
                return;
            }
        };

        self.error_status = Some(format!("Error on reset: some test error"));

        match message {
            Message::Subsystem(msg) => match msg {
                SubsystemMessage::HandleDestButton(e) => {
                    match self
                        .destructive_actions
                        .data::<SubsystemMessage>(e)
                        .unwrap()
                    {
                        SubsystemMessage::Reset => match data.reset(true) {
                            Ok(_) => {}
                            Err(e) => {
                                self.error_status = Some(format!("Error on reset: {e}"));
                            }
                        },
                        SubsystemMessage::Delete => match data.remove(true) {
                            Ok(_) => {}
                            Err(e) => {
                                self.error_status = Some(format!("Error on delete: {e}"));
                            }
                        },

                        _ => (),
                    }
                }
                SubsystemMessage::HandleSubButton(e) => {
                    match self.sub_actions.data::<SubsystemMessage>(e).unwrap() {
                        SubsystemMessage::Start => {
                            self.dialog_string = Some("Starting subsystem".into());
                            match data.start() {
                                Ok(_) => {}
                                Err(e) => {
                                    self.error_status = Some(format!("Error on start: {e}"));
                                }
                            }
                        }
                        SubsystemMessage::Stop => match data.stop() {
                            Ok(_) => {}
                            Err(e) => {
                                self.error_status = Some(format!("Error on stop: {e}"));
                            }
                        },
                        SubsystemMessage::Autoremove => match data.autoremove() {
                            Ok(_) => {}
                            Err(e) => {
                                self.error_status = Some(format!("Error on autoremove: {e}"));
                            }
                        },
                        SubsystemMessage::CleanPackageManagerCache => match data.clean() {
                            Ok(_) => {}
                            Err(e) => {
                                self.error_status = Some(format!("Error on clean: {e}"));
                            }
                        },
                        _ => (),
                    }
                }
                SubsystemMessage::Reset => match data.reset(true) {
                    Ok(_) => {}
                    Err(e) => {
                        self.error_status = Some(format!("Error on reset: {e}"));
                    }
                },
                SubsystemMessage::Start => match data.start() {
                    Ok(_) => {}
                    Err(e) => {
                        self.error_status = Some(format!("Error on start: {e}"));
                    }
                },
                SubsystemMessage::Stop => match data.stop() {
                    Ok(_) => {}
                    Err(e) => {
                        self.error_status = Some(format!("Error on stop: {e}"));
                    }
                },
                SubsystemMessage::Autoremove => match data.autoremove() {
                    Ok(_) => {}
                    Err(e) => {
                        self.error_status = Some(format!("Error on autoremove: {e}"));
                    }
                },
                SubsystemMessage::CleanPackageManagerCache => match data.clean() {
                    Ok(_) => {}
                    Err(e) => {
                        self.error_status = Some(format!("Error on clean: {e}"));
                    }
                },
                SubsystemMessage::Delete => match data.remove(true) {
                    Ok(_) => {}
                    Err(e) => {
                        self.error_status = Some(format!("Error on delete: {e}"));
                    }
                },
                SubsystemMessage::CloseError => self.error_status = None,
                SubsystemMessage::CloseAction => self.action_status = None,
            },
            _ => {}
        }
    }
}

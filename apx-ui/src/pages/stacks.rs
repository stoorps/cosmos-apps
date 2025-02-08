use apx_shim::Stack;
use cosmic::{
    self, iced_widget,
    widget::{self, nav_bar},
};

use crate::app::Message;

use super::PageModel;

pub struct StacksModel {
    nav_bar: nav_bar::Model,
}

impl StacksModel {
    pub fn new() -> Self {
        Self {
            nav_bar: nav_bar::Model::default(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StackMessage {
    BaseEdited(String),
    PackageManagerEdited(String),
    PackagesEdited,
}

impl Into<Message> for StackMessage {
    fn into(self) -> Message {
        Message::Stack(self)
    }
}

impl PageModel for StacksModel {
    fn view(&self) -> cosmic::Element<'_, Message> {
        let selected = self.nav_bar.active();
        let data = self.nav_bar.active_data::<Stack>();

        if let Some(data) = data {
            iced_widget::column![
                widget::Text::new(&data.name).size(32),
                widget::Text::new("Details").size(24),
                widget::Container::new(iced_widget::column![
                    widget::TextInput::new("base:latest", &data.base)
                        .label("Base")
                        .editable()
                        .on_input(|text| StackMessage::BaseEdited(text).into()),
                    widget::TextInput::new("pkg manager", &data.package_manager)
                        .label("Package Manager")
                        .editable()
                        .on_input(|text| StackMessage::PackageManagerEdited(text).into()),
                ]),
            ]
            .into()
        } else {
            widget::Column::new()
                .push(widget::Text::new("No package manager selected").size(24))
                .into()
        }
    }

    fn current_items(&self) -> &nav_bar::Model {
        &self.nav_bar
    }

    fn update_items(&mut self) {
        let data = apx_shim::Stack::get_all();

        self.nav_bar = match data {
            Ok(data) => {
                let mut items = nav_bar::Model::default();
                for item in data {
                    items.insert().text(item.name.clone()).data::<Stack>(item);
                }
                items
            }
            Err(_) => nav_bar::Model::default(),
        };
    }

    fn on_select(&mut self, item: widget::segmented_button::Entity) {
        self.nav_bar.activate(item);
    }

    fn on_message(&mut self, message: Message) {
        let data = self.nav_bar.active_data_mut::<Stack>().unwrap(); //TODO: handle unwrap

        match message {
            Message::Stack(msg) => match msg {
                StackMessage::BaseEdited(text) => {
                    data.base = text;
                }
                StackMessage::PackageManagerEdited(text) => {
                    data.package_manager = text;
                }
                StackMessage::PackagesEdited => {}
            },
            _ => {}
        }
    }
}

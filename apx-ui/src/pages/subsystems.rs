use apx_shim::Subsystem;
use cosmic::{
    self,
    widget::{self, nav_bar},
};

use crate::app::Message;

use super::PageModel;

pub struct SubSystemsModel {
    nav_bar: nav_bar::Model,
}

impl SubSystemsModel {
    pub fn new() -> Self {
        Self {
            nav_bar: nav_bar::Model::default(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SubsystemMessage {
    Placeholder,
}

impl PageModel for SubSystemsModel {
    fn view(&self) -> cosmic::Element<'_, Message> {
        widget::Column::new()
            .push(widget::Text::new("Subsystems").size(50))
            .into()
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
        match message {
            Message::Subsystem(msg) => match msg {
                SubsystemMessage::Placeholder => {}
            },
            _ => {}
        }
    }
}

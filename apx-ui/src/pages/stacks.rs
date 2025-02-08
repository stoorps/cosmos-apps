use apx_shim::Stack;
use cosmic::{
    self,
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

impl PageModel for StacksModel {
    fn view(&self) -> cosmic::Element<'_, Message> {
        widget::Column::new()
            .push(widget::Text::new("Stacks").size(50))
            .into()
    }

    fn on_select(&mut self, item: widget::segmented_button::Entity) {
        self.nav_bar.activate(item);
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
}

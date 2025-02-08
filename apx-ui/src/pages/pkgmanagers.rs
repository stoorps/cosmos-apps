use apx_shim::PackageManager;
use cosmic::{
    self,
    widget::{self, nav_bar},
};

use crate::app::Message;

use super::PageModel;

pub struct PkgManagerModel {
    nav_bar: nav_bar::Model,
}

impl PkgManagerModel {
    pub fn new() -> Self {
        Self {
            nav_bar: nav_bar::Model::default(),
        }
    }
}

impl PageModel for PkgManagerModel {
    fn view(&self) -> cosmic::Element<'_, Message> {
        let selected = self.nav_bar.active();
        let data = self.nav_bar.active_data::<PackageManager>();

        match data {
            Some(data) => widget::Column::new()
                .push(widget::Text::new(&data.name).size(50))
                .into(),

            None => widget::Column::new()
                .push(widget::Text::new("No package manager selected").size(50))
                .into(),
        }

        // widget::Column::new()
        //     .push(widget::Text::new("Stacks").size(50))
        //     .into()
    }

    fn current_items(&self) -> &nav_bar::Model {
        &self.nav_bar
    }

    fn update_items(&mut self) {
        let data = apx_shim::PackageManager::get_all();
        let nav = match data {
            Ok(data) => {
                let mut items = nav_bar::Model::default();
                for item in data {
                    items
                        .insert()
                        .text(item.name.clone())
                        .data::<PackageManager>(item);
                }
                items
            }
            Err(_) => nav_bar::Model::default(),
        };

        self.nav_bar = nav;
    }

    fn on_select(&mut self, item: widget::segmented_button::Entity) {
        self.nav_bar.activate(item);
    }
}

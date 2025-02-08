use apx_shim::PackageManager;
use cosmic::{
    self, cosmic_theme::{self, Spacing}, iced_widget::{self, iced}, theme, widget::{self, nav_bar}, Element, Theme
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

#[derive(Debug, Clone)]
pub enum PkgManagerMessage {
    AutoRemoveEdited(String),
    CleanEdited(String),
    InstallEdited(String),
    ListEdited(String),
    PurgeEdited(String),
    RemoveEdited(String),
    SearchEdited(String),
    ShowEdited(String),
    UpdateEdited(String),
    UpgradeEdited(String),
}

impl Into<Message> for PkgManagerMessage {
    fn into(self) -> Message {
        Message::PkgManager(self)
    }
}

impl PageModel for PkgManagerModel {
    fn view(&self) -> cosmic::Element<'_, Message> {
        let selected = self.nav_bar.active();
        let data = self.nav_bar.active_data::<PackageManager>();

        if let Some(data) = data {
            println!("is built-in: {}", data.built_in);

            let mut editors: Vec<widget::TextInput<'_, Message>> = match data.built_in {
                false => vec![
                    widget::TextInput::new("autoremove command", &data.cmd_auto_remove)
                        .label("Autoremove")
                        .on_input(|text| PkgManagerMessage::AutoRemoveEdited(text).into()),
                    widget::TextInput::new("clean command", &data.cmd_clean)
                        .label("Clean")
                        .on_input(|text| PkgManagerMessage::CleanEdited(text).into()),
                    widget::TextInput::new("install command", &data.cmd_install)
                        .label("Install")
                        .on_input(|text| PkgManagerMessage::InstallEdited(text).into()),
                    widget::TextInput::new("list command", &data.cmd_list)
                        .label("List")
                        .on_input(|text| PkgManagerMessage::ListEdited(text).into()),
                    widget::TextInput::new("purge command", &data.cmd_purge)
                        .label("Purge")
                        .on_input(|text| PkgManagerMessage::PurgeEdited(text).into()),
                    widget::TextInput::new("remove command", &data.cmd_remove)
                        .label("Remove")
                        .on_input(|text| PkgManagerMessage::RemoveEdited(text).into()),
                    widget::TextInput::new("search command", &data.cmd_search)
                        .label("Search")
                        .on_input(|text| PkgManagerMessage::ShowEdited(text).into()),
                    widget::TextInput::new("show command", &data.cmd_show)
                        .label("Search")
                        .on_input(|text| PkgManagerMessage::SearchEdited(text).into()),
                    widget::TextInput::new("update command", &data.cmd_update)
                        .label("Update")
                        .on_input(|text| PkgManagerMessage::UpdateEdited(text).into()),
                    widget::TextInput::new("upgrade command", &data.cmd_upgrade)
                        .label("Upgrade")
                        .on_input(|text| PkgManagerMessage::UpgradeEdited(text).into()),
                ],
                true => vec![
                    widget::TextInput::new("autoremove command", &data.cmd_auto_remove)
                        .label("Autoremove"),
                    widget::TextInput::new("clean command", &data.cmd_clean).label("Clean"),
                    widget::TextInput::new("install command", &data.cmd_install).label("Install"),
                    widget::TextInput::new("list command", &data.cmd_list).label("List"),
                    widget::TextInput::new("purge command", &data.cmd_purge).label("Purge"),
                    widget::TextInput::new("remove command", &data.cmd_remove).label("Remove"),
                    widget::TextInput::new("search command", &data.cmd_search).label("Search"),
                    widget::TextInput::new("show command", &data.cmd_show).label("Search"),
                    widget::TextInput::new("update command", &data.cmd_update).label("Update"),
                    widget::TextInput::new("upgrade command", &data.cmd_upgrade).label("Upgrade"),
                ],
            };

            let mut column = widget::Column::new();
            for editor in editors.into_iter() {
                let element: Element<'_, Message> = editor.into(); // Type annotation is crucial
                column = column.push(element); // Reassign the column
            }

            iced_widget::scrollable(
            iced_widget::column![
                widget::Text::new("Details").size(18),
                widget::Container::new(widget::Text::new(&data.name).size(32)).style(|t| theme::Container::primary(&cosmic_theme::Theme::default())),
                widget::Text::new("Commands").size(18),
                widget::Container::new(column.spacing(20).padding(20)).style(|t| theme::Container::primary(&cosmic_theme::Theme::default()))
            ].spacing(Spacing::default().space_xs))
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

    fn on_message(&mut self, message: Message) {
        let selected = self.nav_bar.active();
        let data = self.nav_bar.active_data_mut::<PackageManager>().unwrap(); //TODO: handle unwrap

        match message {
            Message::PkgManager(msg) => match msg {
                PkgManagerMessage::AutoRemoveEdited(s) => data.cmd_auto_remove = s,
                PkgManagerMessage::CleanEdited(s) => data.cmd_clean = s,
                PkgManagerMessage::InstallEdited(s) => data.cmd_install = s,
                PkgManagerMessage::ListEdited(s) => data.cmd_list = s,
                PkgManagerMessage::PurgeEdited(s) => data.cmd_purge = s,
                PkgManagerMessage::RemoveEdited(s) => data.cmd_remove = s,
                PkgManagerMessage::SearchEdited(s) => data.cmd_search = s,
                PkgManagerMessage::ShowEdited(s) => data.cmd_show = s,
                PkgManagerMessage::UpdateEdited(s) => data.cmd_update = s,
                PkgManagerMessage::UpgradeEdited(s) => data.cmd_upgrade = s,
            },

            _ => (),
        }
    }
}

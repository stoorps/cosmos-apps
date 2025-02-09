use apx_shim::Stack;
use cosmic::{
    self,
    cosmic_theme::{self, Spacing},
    iced::{Alignment, Length},
    iced_widget, theme,
    widget::{self, button, nav_bar},
    Element,
};
use tracing::debug;
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
    Reset,
    Save,
    Delete,
}

impl Into<Message> for StackMessage {
    fn into(self) -> Message {
        Message::Stack(self)
    }
}

impl PageModel for StacksModel {
    fn view(&self) -> cosmic::Element<'_, Message> {
        let data = self.nav_bar.active_data::<Stack>();

        if let Some(data) = data {
            let editors: Vec<cosmic::Element<'_, Message>> = match data.built_in {
                false => vec![
                    widget::TextInput::new("base:latest", &data.base)
                        .label("Base")
                        .on_input(|text| StackMessage::BaseEdited(text).into())
                        .into(),
                    widget::TextInput::new("pkg manager", &data.package_manager)
                        .label("Package Manager")
                        .on_input(|text| StackMessage::PackageManagerEdited(text).into())
                        .into(),
                ],
                true => vec![
                    widget::TextInput::new("base:latest", &data.base)
                        .label("Base")
                        .into(),
                    widget::TextInput::new("pkg manager", &data.package_manager)
                        .label("Package Manager")
                        .into(),
                ],
            };

            let mut column = widget::Column::new();
            for editor in editors.into_iter() {
                let element: Element<'_, Message> = editor.into(); // Type annotation is crucial
                column = column.push(element); // Reassign the column
            }

            iced_widget::column![
                iced_widget::row![
                    widget::Text::new(&data.name).size(24).width(Length::Fill),
                    iced_widget::row![
                        button::link("Reset").on_press(StackMessage::Reset.into()),
                        button::link("Save").on_press(StackMessage::Save.into()),
                        button::link("Delete").on_press(StackMessage::Delete.into()),
                    ]
                    .spacing(20)
                    .width(Length::Shrink)
                    .align_y(Alignment::Center)
                ]
                .padding([0, 0, 20, 0])
                .height(Length::Shrink),
                iced_widget::scrollable(
                    iced_widget::column![
                        widget::Text::new("Commands").size(18),
                        widget::Container::new(column.spacing(20).padding(20))
                            .style(|_| theme::Container::primary(&cosmic_theme::Theme::default())),
                    ]
                    .spacing(Spacing::default().space_xs)
                )
                .height(Length::Fill),
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
                StackMessage::Save => match data.update() {
                    Ok(_) => return,
                    Err(e) => todo!("Handle error on saving: {e}"),
                },
                StackMessage::Reset => {
                    let name = self
                        .nav_bar
                        .active_data_mut::<Stack>()
                        .unwrap()
                        .name
                        .clone(); //TODO: handle unwrap

                    self.update_items();
                    let matched = self
                        .nav_bar
                        .iter()
                        .find(|e| self.nav_bar.data::<Stack>(*e).unwrap().name == name);

                    match matched {
                        Some(m) => self.nav_bar.activate(m),
                        None => todo!("Handle no match on reset"),
                    }
                }
                StackMessage::Delete => match data.remove(true) {
                    Ok(_) => {
                        debug!("Successfully deleted");
                        self.nav_bar.remove(self.nav_bar.active());
                    }
                    Err(_) => todo!(),
                },
            },

            _ => {}
        }
    }
}

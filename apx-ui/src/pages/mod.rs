use cosmic::widget::{nav_bar, segmented_button::Entity};

use crate::app::Message;

pub(crate) mod pkgmanagers;
pub(crate) mod stacks;
pub(crate) mod subsystems;

/// The page to display in the application.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    Subsystems,
    PkgManagers,
    Stacks,
}

pub trait PageModel {
    fn view(&self) -> cosmic::Element<'_, Message>;
    fn current_items(&self) -> &nav_bar::Model;
    fn update_items(&mut self);
    fn on_select(&mut self, item: Entity);
}

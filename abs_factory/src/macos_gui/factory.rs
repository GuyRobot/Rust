extern crate gui;

use gui::GuiFactory;
use super::checkbox::MacCheckbox;
use super::button::MacButton;

pub struct MacFactory {}

impl GuiFactory for MacFactory {

    type B = MacButton;
    type C = MacCheckbox;

    fn create_button(&self) -> Self::B {
        MacButton{}
    }
    fn create_checkbox(&self) -> Self::C {
        MacCheckbox{}
    }
}
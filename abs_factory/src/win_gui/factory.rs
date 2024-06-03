extern crate gui;

use gui::GuiFactory;
use super::checkbox::WindowsCheckbox;
use super::button::WindowsButton;

pub struct WindowsFactory {}

impl GuiFactory for WindowsFactory {
    type B = WindowsButton;
    type C = WindowsCheckbox;

    fn create_button(&self) -> Self::B {
        WindowsButton{}
    }
    fn create_checkbox(&self) -> Self::C {
        WindowsCheckbox{}
    }
}
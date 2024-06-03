extern crate gui;

use gui::Checkbox;

pub struct WindowsCheckbox {}

impl Checkbox for WindowsCheckbox {
    fn switch(&self) {
        println!("MacCheckbox switch");
    }
}
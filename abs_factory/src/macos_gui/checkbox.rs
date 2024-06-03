extern crate gui;

use gui::Checkbox;

pub struct MacCheckbox {}

impl Checkbox for MacCheckbox {
    fn switch(&self) {
        println!("MacCheckbox switch");
    }
}
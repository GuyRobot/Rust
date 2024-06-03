extern crate gui;

use gui::Button;

pub struct MacButton {}

impl Button for MacButton {
    fn press(&self) {
        println!("MacButton press");
    }
}
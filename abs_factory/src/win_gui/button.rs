extern crate gui;

use gui::Button;

pub struct WindowsButton {}

impl Button for WindowsButton {
    fn press(&self) {
        println!("MacButton press");
    }
}
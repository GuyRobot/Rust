mod render;
use render::render;

mod win_gui;
mod macos_gui;

use macos_gui::factory::MacFactory;
use win_gui::factory::WindowsFactory;

fn main() {
    let windows = true;
    if windows {
        render(WindowsFactory{});
    } else {
        render(MacFactory{});
    }
}
mod core;
mod detect;
mod drivers;
mod gui;
mod utils;

use gui::DriverUI;
use iced::Settings;

fn main() -> iced::Result {
    env_logger::init();
    DriverUI::run(Settings::default())
}
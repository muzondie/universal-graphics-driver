use iced::{
    button, executor, Application, Button, Column, Command, 
    Element, Settings, Text, 
};

#[derive(Debug, Clone)]
pub enum Message {
    ScanPressed,
    InstallPressed,
    ExitPressed,
}

pub enum StatusMessage {
    Idle,
    Scanning,
    Installing,
    Success,
    Error(String),
}

pub struct DriverUI {
    state: super::core::AppState,
    scan_btn: button::State,
    install_btn: button::State,
    exit_btn: button::State,
    status: StatusMessage,
}

impl Application for DriverUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self {
            state: super::core::AppState::new(),
            scan_btn: button::State::new(),
            install_btn: button::State::new(),
            exit_btn: button::State::new(),
            status: StatusMessage::Idle,
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Universal Graphics Driver")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ScanPressed => {
                self.state.full_scan();
                Command::none()
            }
            Message::InstallPressed => {
                let result = self.state.install_drivers();
                self.status = match result {
                    Ok(()) => StatusMessage::Success,
                    Err(e) => StatusMessage::Error(e.to_string()),
                };
                Command::none()
            }
            Message::ExitPressed => iced::window::close(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(Text::new("Universal Graphics Driver").size(24))
            .push(Button::new(&mut self.scan_btn, Text::new("Scan System"))
                .on_press(Message::ScanPressed))
            .push(Button::new(&mut self.install_btn, Text::new("Install Drivers"))
                .on_press(Message::InstallPressed))
            .push(Button::new(&mut self.exit_btn, Text::new("Exit"))
                .on_press(Message::ExitPressed))
            .into()
    }
}
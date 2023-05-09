used iced::{button, Align, Button, Column, Element, Row, Sandbox, Settings, Text};

fn main() -> iced::Result {

}

// Model
// View
// Message
// Update
struct Model {
    count: i64,
}

enum Msg {
    Increment,
    Decrement,
}

impl Sandbox for Model {
    type Message = Msg;

    fn new() -> Self {

    }

    fn title(&self) -> String {

    }

    fn update(&mut self, message: Self::Message) {

    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        
    }
}
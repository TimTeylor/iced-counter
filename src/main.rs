use iced::widget::{Column, Text};
use iced::{Sandbox, Element, Settings};

fn main() -> iced::Result {
    Model::run(Settings::default())
}

// Model
// View
// Message
// Update
struct Model {
    count: i64,
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    Increment,
    Decrement,
}

impl Sandbox for Model {
    type Message = Msg;

    fn new() -> Self {
        Self {
            count: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let label = Text::new(self.count.to_string());
        Column::new().push(label).into()
    }
}
use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

fn main() -> iced::Result {
    Model::run(Settings::default())
}

struct Model {
    count: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self { count: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.count).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
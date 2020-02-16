use iced::{
    button, image, text_input, Button, Column, Container, Element, Image, Length, Row, Sandbox,
    Settings, Text, TextInput,
};
use sorter_backend::Backend;

pub fn main() {
    // can change Settings to allow for resizable and different starting size
    Frontend::run(Settings::default())
}

#[derive(Default)]
struct Frontend {
    backend: Backend,
    file_name_state: text_input::State,
    file_name_value: String,
    go_to_button: button::State,
    load_button: button::State,

    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    FileNameChanged(String),
}

impl Sandbox for Frontend {
    type Message = Message;

    fn new() -> Self {
        let mut test = Frontend {
            backend: Backend::new(),
            file_name_state: text_input::State::new(),
            file_name_value: String::new(),
            go_to_button: button::State::new(),
            load_button: button::State::new(),
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
        };
        test.backend
            .load_folders_and_files("/home/nick/Pictures".to_string())
            .expect("well, it failed to find the pictures");

        test
    }

    fn title(&self) -> String {
        String::from("A simple counter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.backend.increment().expect("whoops");
            }
            Message::DecrementPressed => {
                self.backend.undo().expect("whoops");
            }
            Message::FileNameChanged(value) => {
                println!("file name field: {}", value);
                self.file_name_value = value;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Row::new()
            .padding(20)
            .push(
                // content container
                Column::new()
                    .push(
                        Container::new(
                            Image::new(image::Handle::from_path(self.backend.get_current_file()))
                                .width(Length::Fill),
                        )
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .center_x(),
                    )
                    .push(
                        // file information and load
                        Row::new().push(
                            Row::new()
                                .push(Text::new("File Name").width(Length::Shrink))
                                .padding(20)
                                .push(
                                    TextInput::new(
                                        &mut self.file_name_state,
                                        "This is the placeholder...",
                                        self.file_name_value.as_str(),
                                        Message::FileNameChanged,
                                    )
                                    .width(Length::Fill),
                                )
                                .push(Button::new(&mut self.go_to_button, Text::new("Go to")))
                                .push(Button::new(&mut self.load_button, Text::new("Load"))),
                        ),
                    )
                    .width(Length::Fill),
            )
            .push(
                // button and folder panel
                Column::new()
                    .padding(20)
                    .push(
                        Row::new()
                        .push(
                            Button::new(&mut self.increment_button, Text::new("Increment"))
                                .on_press(Message::IncrementPressed),
                        )
                        .push(
                            Button::new(&mut self.decrement_button, Text::new("Decrement"))
                                .on_press(Message::DecrementPressed),
                        )
                    )
                    
            )
            .into()

        /* old original
                Column::new()
                    .padding(20)
                    .push(
                        Button::new(&mut self.increment_button, Text::new("Increment"))
                            .on_press(Message::IncrementPressed),
                    )
        //            .push(Image::new(self.backend.get_current_file().to_string_lossy()))
                    .push(
                        Button::new(&mut self.decrement_button, Text::new("Decrement"))
                            .on_press(Message::DecrementPressed),
                    )
                    .push(picture(&self.backend))
                    .into()
                    */
    }
}

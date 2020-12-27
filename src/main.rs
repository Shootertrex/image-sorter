use iced::{
    button, executor, image, keyboard, scrollable, text_input, Application, Button, Column,
    Command, Container, Element, Image, Length, Row, Scrollable, Settings, Subscription, Text,
    TextInput,
};
use iced_native::{event, subscription, Event};
use sorter_backend::Backend;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub fn main() {
    // can change Settings to allow for resizable and different starting size
    Frontend::run(Settings::default());
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

    folder_buttons: Vec<Folder>,

    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
enum Message {
    Skip,
    Undo,
    Redo,
    FileNameChanged(String),
    FileMoved(PathBuf),
    NumberedFolder(usize),
    Load,
}

impl Application for Frontend {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let front_end = Frontend {
            backend: Backend::new(),
            file_name_state: text_input::State::new(),
            file_name_value: String::new(),
            go_to_button: button::State::new(),
            load_button: button::State::new(),
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
            folder_buttons: Vec::new(),
            scroll: scrollable::State::new(),
        };

        (front_end, Command::none())
    }

    fn title(&self) -> String {
        String::from("A simple counter")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Skip => {
                println!("incrementing");
                match self.backend.skip() {
                    Ok(_) => { /* do nothing */ }
                    Err(error) => match error.kind() {
                        ErrorKind::UnexpectedEof => println!("no remaining files!"),
                        _ => println!("whoops"),
                    },
                };
            }
            Message::Undo => {
                println!("undoing");
                self.backend.undo().expect("whoops");
            }
            Message::Redo => {
                println!("redoing");
                self.backend.redo().expect("whoops");
            }
            Message::FileNameChanged(value) => {
                println!("file name field: {}", value);
                self.file_name_value = value;
            }
            Message::FileMoved(value) => {
                println!("moving file to : {:?}", value);
                self.backend.move_file(value);
            }
            Message::NumberedFolder(value) => {
                println!("moving file to folder {:?}", value);
                println!( "{:?}", self.folder_buttons .get(value) .expect("no folders") .path .clone());
                self.update(Message::FileMoved(
                    self.folder_buttons
                        .get(value)
                        .expect("no folders")
                        .path
                        .clone(),
                ));
                // self.backend.move_file(value);
            }
            Message::Load => {
                if self.file_name_value == "" {
                    return Command::none();
                }
                println!("loading files from {:?}", self.file_name_value);
                match self
                    .backend
                    .load_folders_and_files(self.file_name_value.to_string())
                {
                    Ok(_) => {}
                    Err(_) => {
                        println!("well, it failed to find the pictures");
                        return Command::none();
                    }
                }

                self.folder_buttons = Vec::new();
                let folder_count = self.backend.folders.len();
                for x in 0..folder_count {
                    self.folder_buttons.push(Folder::new(self.backend.folders[x].clone()));
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| {
            if let event::Status::Captured = status {
                return None;
            }

            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    modifiers,
                    key_code,
                }) => handle_hotkey(modifiers, key_code),
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<Message> {
        let folder_column = self
            .folder_buttons
            .iter_mut()
            .fold(Column::new(), |column, button| {
                let label: &str = button.path.file_name().unwrap().to_str().unwrap();
                column
                    .push(
                        Button::new(&mut button.button_state, Text::new(label))
                            .on_press(Message::FileMoved(button.path.clone()))
                            .width(Length::Fill),
                    )
                    .spacing(10)
            });

        let scrolling_container = Scrollable::new(&mut self.scroll).push(folder_column);

        Row::new()
            .padding(20)
            .push(
                // content container
                Column::new()
                    .push(
                        Container::new(
                            Image::new(image::Handle::from_path(self.backend.get_current_file()))
                                .width(Length::Fill)
                                .height(Length::Fill),
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
                                    // TODO: this text input is temporarily handling folder loading
                                    TextInput::new(
                                        &mut self.file_name_state,
                                        "This is the placeholder...",
                                        self.file_name_value.as_str(),
                                        Message::FileNameChanged,
                                    )
                                    .width(Length::Fill),
                                )
                                .push(Button::new(&mut self.go_to_button, Text::new("Go to")))
                                .push(
                                    Button::new(&mut self.load_button, Text::new("Load"))
                                        .on_press(Message::Load),
                                ),
                        ),
                    )
                    .width(Length::Fill),
            )
            .push(
                // button and folder panel
                Column::new()
                    .padding(20)
                    .max_width(245)
                    .push(
                        Row::new()
                            .push(
                                Button::new(&mut self.increment_button, Text::new("Increment"))
                                    .on_press(Message::Skip),
                            )
                            .push(
                                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                                    .on_press(Message::Undo),
                            ),
                    )
                    .push(Column::new().push(Container::new(scrolling_container))),
            )
            .into()
    }
}

struct Folder {
    path: PathBuf,
    button_state: button::State,
}

impl Folder {
    pub fn new(path: PathBuf) -> Folder {
        Folder {
            path: path,
            button_state: button::State::new(),
        }
    }
}

fn handle_hotkey(modifiers: keyboard::Modifiers, key_code: keyboard::KeyCode) -> Option<Message> {
    println!("key code event");
    use keyboard::KeyCode;

    if modifiers.is_command_pressed() {
        return match key_code {
            KeyCode::Z => Some(Message::Undo),
            KeyCode::Y => Some(Message::Redo),
            _ => None,
        };
    }

    return match key_code {
        KeyCode::Space => Some(Message::Skip),
        KeyCode::Key1 | KeyCode::Numpad1 => Some(Message::NumberedFolder(0)),
        KeyCode::Key2 | KeyCode::Numpad2 => Some(Message::NumberedFolder(1)),
        KeyCode::Key3 | KeyCode::Numpad3 => Some(Message::NumberedFolder(2)),
        KeyCode::Key4 | KeyCode::Numpad4 => Some(Message::NumberedFolder(3)),
        KeyCode::Key5 | KeyCode::Numpad5 => Some(Message::NumberedFolder(4)),
        KeyCode::Key6 | KeyCode::Numpad6 => Some(Message::NumberedFolder(5)),
        KeyCode::Key7 | KeyCode::Numpad7 => Some(Message::NumberedFolder(6)),
        KeyCode::Key8 | KeyCode::Numpad8 => Some(Message::NumberedFolder(7)),
        KeyCode::Key9 | KeyCode::Numpad9 => Some(Message::NumberedFolder(8)),
        KeyCode::Key0 | KeyCode::Numpad0 => Some(Message::NumberedFolder(9)),
        _ => None,
    };
}

use iced::{
    button, Command, image, text_input, Button, Column, Container, Element, Image, Length, Row, Sandbox,
    Settings, Text, TextInput,
};
use iced_winit:: {Widget};
use sorter_backend::Backend;
use std::path::{Path, PathBuf};

pub fn main() {
    // can change Settings to allow for resizable and different starting size
    // Frontend::run(Settings::default())
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

    folder_buttons: Vec<button::State>,
    folder_buttons1: Vec<Folder>,
}

#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    FileNameChanged(String),
    FileMoved(PathBuf),
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
            folder_buttons: Vec::new(),
            folder_buttons1: Vec::new(),
        };

// ****** this is temporary until pictures are loaded with a button!!! *******************
        test.backend
            .load_folders_and_files("/home/nick/Pictures/movingTest".to_string())
            .expect("well, it failed to find the pictures");

	let folder_count = test.backend.folders.len();
        for x in 0..folder_count {
		test.folder_buttons1.push(Folder::new(test.backend.folders[x].clone()));
		//self.folder_buttons.push(button::State::new())
        }
// ************************************

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
            Message::FileMoved(value) => {
                println!("moving file to : {:?}", value);
		self.backend.move_file(value);
		self.update(Message::IncrementPressed);
                //self.file_name_value = value;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut myColumn = Column::new();
	myColumn = self.folder_buttons1.iter_mut().fold(Column::new(), |column, button| {
	    //column.push(Button::new(button, Text::new("frank")))
		let label:&str = button.path.file_name().unwrap().to_str().unwrap();
	    column.push(Button::new(&mut button.button_state, Text::new(label))
			.on_press(Message::FileMoved(button.path.clone())))
	});

        // Container::new(newRow)

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
                    // in todo example
                    // line ~90 is when tasks are created and added to the vec
                    // line ~170 is when tasks are being made into gui elements, i think
                    .push(
                        Column::new()
                        .push(
                            Container::new(myColumn)
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




// fn test() -> Element<Message> + 'static {
//     let newRows: Element<_>
// }

impl Frontend {
    // fn testFunc() -> ! {
    //     let mut newRow = Row::new().push(Text::new("File Name").width(Length::Shrink));
    //     return newRow.into();
    // }
    // fn test() -> Element<Message> + 'static {
    // let mut newRow = Row::new();
    // for x in 0..10 {
    // newRow.push(Text::new("File Name").width(Length::Shrink));
    // }
    //
    // Container::new(newRow)
    // }
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
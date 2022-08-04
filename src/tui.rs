use crate::request::{Request, RequestType};
use std::io::{self, Write};
use termion::{
    color,
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
};

pub async fn tui() {
    let mut sut = SweetUserTui::new();
    sut.sweet_user_tui();
}

pub struct SweetUserTui {
    highlighted_selection: RequestType,
    selection_index: usize,
    selections: Vec<RequestType>,
}

impl SweetUserTui {
    pub fn new() -> Self {
        let selections = vec![
            RequestType::Get,
            RequestType::Put,
            RequestType::Delete,
            RequestType::Ls,
        ];
        let selection_index = 0;
        let highlighted_selection = &selections[selection_index];

        Self {
            highlighted_selection: *highlighted_selection,
            selection_index,
            selections,
        }
    }

    pub fn sweet_user_tui(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout().into_raw_mode().unwrap();

        for event in stdin.events() {
            let event = event.unwrap();
            match event {
                Event::Key(Key::Up) => {
                    if self.selection_index >= 1 {
                        self.selection_index -= 1;
                        self.highlighted_selection =
                            self.selections[self.selection_index];
                    }
                    self.update_screen();
                }
                Event::Key(Key::Down) => {
                    if self.selection_index + 1 < self.selections.len() {
                        self.selection_index += 1;
                        self.highlighted_selection =
                            self.selections[self.selection_index];
                    }
                    self.update_screen();
                }
                Event::Key(Key::Char('\n')) => {
                    // TODO: Store the highlighted selection as in the Request.
                    break;
                }
                Event::Key(Key::Ctrl('d')) => break,
                Event::Key(Key::Ctrl('c')) => break,
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }

    fn update_screen(&self) {
        for (i, option) in self.selections.iter().enumerate() {
            if *option == self.highlighted_selection {
                println!("{}{}. {}", color::Fg(color::Yellow), i, option);
                println!("{}", color::Fg(color::Reset));
                continue;
            }
            println!("{}. {}", i, option);
        }
    }
}

async fn basic_tui() {
    let mut request_type = String::new();

    while request_type != "get"
        && request_type != "put"
        && request_type != "delete"
        && request_type != "ls"
    {
        println!("Select a request (GET, PUT, DELETE, ls): ");
        io::stdin().read_line(&mut request_type).unwrap();
        request_type = request_type.trim().to_string().to_lowercase();
    }

    let request_type = match request_type.to_lowercase().as_str() {
        "get" => RequestType::Get,
        "put" => RequestType::Put,
        "delete" => RequestType::Delete,
        "ls" => RequestType::Ls,
        _ => RequestType::Ls,
    };

    // Prompt user for key if not making an ls request. We know what the key
    // will be for an ls request so its easier to not make the user provide it.
    let mut key = String::new();
    if request_type != RequestType::Ls {
        println!("Enter key: ");
        io::stdin().read_line(&mut key).unwrap();
        key = key.trim().to_string();
    } else {
        key = "ls".to_string();
    }

    let mut value: Option<String> = None;
    if request_type == RequestType::Put {
        let mut buf = String::new();
        println!("Enter value: ");
        io::stdin().read_line(&mut buf).unwrap();
        value = Some(buf.trim().to_string());
    }

    let mut encryption_key: Option<String> = None;
    if request_type != RequestType::Put {
        let mut buf = String::new();
        println!("Enter server provided encryption key: ");
        io::stdin().read_line(&mut buf).unwrap();
        encryption_key = Some(buf.trim().to_string());
    }

    let request = Request::new(request_type, key, value, encryption_key);

    crate::request::request(request).await.unwrap();
}

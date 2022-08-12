use crate::request::{Request, RequestType};
use std::io::{self, Write};
use termion::{
    color,
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
    style,
};

pub async fn tui() {
    let menu_options = vec![
        RequestType::Get,
        RequestType::Put,
        RequestType::Delete,
        RequestType::Ls,
    ];

    let mut sut = Tui::new(&menu_options);
    sut.run_tui();
    sut.collect_request_information().await;
}

pub struct Tui<'a> {
    current_selection: RequestType,
    selection_index: usize,
    menu_options: &'a Vec<RequestType>,
}

impl<'a> Tui<'a> {
    pub fn new(menu_options: &'a Vec<RequestType>) -> Self {
        let selection_index = 0;
        let highlighted_selection = &menu_options[selection_index];
        Self {
            current_selection: *highlighted_selection,
            selection_index,
            menu_options,
        }
    }

    pub fn run_tui(&mut self) {
        self.request_selection_menu();
    }

    fn request_selection_menu(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout().into_raw_mode().unwrap();

        stdout.suspend_raw_mode().unwrap();
        self.update_selection_menu();
        stdout.activate_raw_mode().unwrap();

        for event in stdin.events() {
            let event = event.unwrap();
            match event {
                Event::Key(Key::Up) | Event::Key(Key::Ctrl('p')) => {
                    self.move_selection_up();
                    stdout.suspend_raw_mode().unwrap();
                    self.update_selection_menu();
                    stdout.activate_raw_mode().unwrap();
                }
                Event::Key(Key::Down) | Event::Key(Key::Ctrl('n')) => {
                    self.move_selection_down();
                    stdout.suspend_raw_mode().unwrap();
                    self.update_selection_menu();
                    stdout.activate_raw_mode().unwrap();
                }
                Event::Key(Key::Char('\n')) => break,
                Event::Key(Key::Ctrl('d')) => break,
                Event::Key(Key::Ctrl('c')) => break,
                _ => {}
            }
            stdout.flush().unwrap();
        }
    }

    fn move_selection_down(&mut self) {
        if self.selection_index + 1 < self.menu_options.len() {
            self.selection_index += 1;
            self.current_selection = self.menu_options[self.selection_index];
        }
    }

    fn move_selection_up(&mut self) {
        if self.selection_index >= 1 {
            self.selection_index -= 1;
            self.current_selection = self.menu_options[self.selection_index];
        }
    }

    fn update_selection_menu(&self) {
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));
        for (mut i, option) in self.menu_options.iter().enumerate() {
            i += 1;
            if *option == self.current_selection {
                println!(
                    "{}{}{}. {}",
                    style::Underline,
                    color::Fg(color::Yellow),
                    i,
                    option
                );
                print!("{}{}", color::Fg(color::Reset), style::Reset);
                continue;
            }
            println!("{}. {}", i, option);
        }
    }

    fn display_current_request(&self) {
        match self.current_selection {
            RequestType::Ls => {
                println!(
                    "{}Ls Request{}",
                    color::Fg(color::Red),
                    color::Fg(color::Reset)
                );
            }
            RequestType::Get => {
                println!(
                    "{}GET Request{}",
                    color::Fg(color::Red),
                    color::Fg(color::Reset)
                );
            }
            RequestType::Put => {
                println!(
                    "{}PUT Request{}",
                    color::Fg(color::Red),
                    color::Fg(color::Reset)
                );
            }
            RequestType::Delete => {
                println!(
                    "{}DELETE Request{}",
                    color::Fg(color::Red),
                    color::Fg(color::Reset)
                );
            }
        }
    }

    async fn collect_request_information(&self) {
        let request_type = self.current_selection;
        print!("{}", termion::clear::All);
        print!("{}", termion::cursor::Goto(1, 1));

        self.display_current_request();

        let mut port = String::from("3400");
        let mut user_port = String::new();
        println!("Enter port [default = 3400]: ");
        io::stdin().read_line(&mut user_port).unwrap();
        if !user_port.trim().is_empty() {
            port = user_port;
        }

        // Prompt user for key if not making an ls request. We know what the key
        // will be for an ls request so its easier to not make the user provide
        // it.
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

        let request =
            Request::new(request_type, key, value, port, encryption_key);
        request.execute().await.unwrap();
    }
}

use std::io;

pub fn tui() {
    basic_tui();
}

fn basic_tui() {
    let mut request = String::new();

    while request.to_lowercase() != "get"
        && request.to_lowercase() != "put"
        && request.to_lowercase() != "delete"
        && request.to_lowercase() != "ls"
    {
        println!("Select a request (GET, PUT, DELETE, ls): ");
        io::stdin().read_line(&mut request).unwrap();
        request = request.trim().to_string();
    }
}

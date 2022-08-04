use crate::request::{Request, RequestType};
use std::io;

pub async fn tui() {
    basic_tui().await;
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

use crate::request;
use std::io;

#[derive(Eq, Debug, PartialEq)]
pub enum RequestType {
    Get,
    Put,
    Delete,
    Ls,
}

pub struct Request {
    pub request_type: RequestType,
    pub key: String,
    pub value: Option<String>,
    pub encryption_key: Option<String>,
}

impl Request {
    fn new(
        request_type: RequestType,
        key: String,
        value: Option<String>,
        encryption_key: Option<String>,
    ) -> Self {
        Self {
            request_type,
            key,
            value,
            encryption_key,
        }
    }
}

pub async fn tui() {
    basic_tui().await;
}

async fn basic_tui() {
    let mut request_type = String::new();

    while request_type.to_lowercase() != "get"
        && request_type.to_lowercase() != "put"
        && request_type.to_lowercase() != "delete"
        && request_type.to_lowercase() != "ls"
    {
        println!("Select a request (GET, PUT, DELETE, ls): ");
        io::stdin().read_line(&mut request_type).unwrap();
        request_type = request_type.trim().to_string();
    }

    let request_type = match request_type.as_str() {
        "get" => RequestType::Get,
        "put" => RequestType::Put,
        "delete" => RequestType::Delete,
        "ls" => RequestType::Ls,
        _ => RequestType::Ls,
    };

    let mut key = String::new();
    println!("Enter key: ");
    io::stdin().read_line(&mut key).unwrap();
    key = key.trim().to_string();

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

    request::request(request).await.unwrap();
}

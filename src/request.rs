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
    pub fn new(
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

pub async fn request(
    request: Request,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let url = format!("http://localhost:3400/{}", request.key);

    let res = match request.request_type {
        RequestType::Get => {
            client
                .get(url)
                .header(
                    "key",
                    request
                        .encryption_key
                        .expect("You must provide an encryption key!"),
                )
                .send()
                .await?
                .bytes()
                .await?
        }
        RequestType::Put => {
            client
                .put(url)
                .body(request.value.expect("GET request requires value."))
                .send()
                .await?
                .bytes()
                .await?
        }
        RequestType::Delete => {
            client
                .delete(url)
                .header(
                    "key",
                    request
                        .encryption_key
                        .expect("You must provide an encryption key!"),
                )
                .send()
                .await?
                .bytes()
                .await?
        }
        RequestType::Ls => {
            client
                .get(url)
                .header(
                    "key",
                    request
                        .encryption_key
                        .expect("You must provide an encryption key!"),
                )
                .send()
                .await?
                .bytes()
                .await?
        }
    };

    // Write the server response out to the user's terminal.
    println!("{}", String::from_utf8_lossy(&res));

    Ok(())
}

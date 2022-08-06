use std::fmt::Display;

#[derive(Eq, Debug, PartialEq, Copy, Clone)]
pub enum RequestType {
    Get,
    Put,
    Delete,
    Ls,
}

impl Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestType::Get => {
                write!(f, "GET Request")
            }
            RequestType::Put => {
                write!(f, "PUT Request")
            }
            RequestType::Delete => {
                write!(f, "DELETE Request")
            }
            RequestType::Ls => {
                write!(f, "List all keys")
            }
        }
    }
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

    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

    let url = format!("http://localhost:3400/{}", request.key);

        let res = match self.request_type {
            RequestType::Get => {
                client
                    .get(url)
                    .header(
                        "key",
                        self.encryption_key
                            .as_ref()
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
                    .body(
                        self.value
                            .as_ref()
                            .expect("You must provide a value!")
                            .to_string(),
                    )
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
                        self.encryption_key
                            .as_ref()
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
                        self.encryption_key
                            .as_ref()
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
}

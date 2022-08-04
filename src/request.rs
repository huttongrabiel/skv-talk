use crate::tui::{Request, RequestType};

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
                .await?;
        }
        RequestType::Put => {
            client
                .put(url)
                .body(request.value.expect("GET request requires value."))
                .send()
                .await?;
        }
        RequestType::Delete => {
            client
                .get(url)
                .header(
                    "key",
                    request
                        .encryption_key
                        .expect("You must provide an encryption key!"),
                )
                .send()
                .await?;
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
                .await?;
        }
    };

    Ok(res)
}

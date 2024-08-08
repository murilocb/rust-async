use thiserror::Error;

#[derive(Error, Debug)]
pub enum ForError {
    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("Http request error")]
    HttpRequest(#[from] reqwest::Error)
}

type Result<T> = std::result::Result<T, ForError>;

#[tokio::main]
async fn main() -> Result<()> {
    let response = reqwest::get("http://httpbin.org/get").await?;

    println!("Status:{}", response.status());
    println!("Headers:\n{:#?}", response.headers());

    let body = response.text().await?;

    println!("Body:\n{}", body);

    Ok(())
}

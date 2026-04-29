use error_chain::error_chain;
use std::io::Read;

// error handling
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    println!("status {}", res.status());
    println!("header {}", res.headers());
    let body = res.text().await?;
    println!("body {}", body);
    Ok(())
}
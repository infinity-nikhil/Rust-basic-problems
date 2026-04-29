use error_chain::error_chain;
use std::io::Read;

// error handling
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}

//read_to_string It takes the response stream (data coming from the internet) and: reads EVERYTHING and converts it into a String
/*
Before:

The response is like a live stream of data (not stored yet).

After read_to_string:

It becomes a normal Rust String in memory
 */
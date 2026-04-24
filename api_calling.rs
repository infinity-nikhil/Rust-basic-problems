use serde::Deserialize;

//Struct format should strictly match the json format 
#[derive(Deserialize, Debug)]
struct Weather {
    main: Main,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
}

#[tokio::main]  //Just like async await
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let city = "Kathmandu";

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid=YOUR_API_KEY&units=metric",
        city
    );

    let response = reqwest::get(&url).await?;

    let data: Weather = response.json().await?;

    println!("Temperature in {}: {}°C", city, data.main.temp);

    Ok(())
}

/* 
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
*/

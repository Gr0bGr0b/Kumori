use dotenvy::dotenv;
use reqwest;
use serde::Deserialize;
use std::env;
use tokio;

#[derive(Deserialize)]
struct WeatherData {
    main: Main,
    weather: Vec<Weather>,
}
#[derive(Deserialize)]
struct Main {
    temp: f64,
}
#[derive(Deserialize)]
struct Weather {
    description: String,
}

async fn fetch_weather(api_key: String, city: String) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid={}",
        city.trim(),
        api_key
    );

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let weather_data: WeatherData = response.json().await?;
        let temperature = weather_data.main.temp;
        let description = &weather_data.weather[0].description;
        println!("Weather in {} is {}C, {}", city, temperature, description);
    } else {
        println!("Error: {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("API_KEY").expect("Variable not defined");

    let mut city = String::new();
    println!("Enter a city:");
    std::io::stdin()
        .read_line(&mut city)
        .expect("Faile to read the city name.");
    tokio::spawn(fetch_weather(api_key, city));
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}

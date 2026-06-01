use serde::deserialize;

#[derive(Deserialize)]
struct WeatherReponse {
    current_weather: WeatherData,
}

#[derive(Deserialize)]
struct WeatherData {
    main: {
        temp: f64,
        feel_like: f64,
        temp_min: f64,
        temp_max: f64
    },
    wind: {
        speed: f64
    ,
}

fn main() {}

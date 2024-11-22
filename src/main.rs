use std::env;
use std::error::Error;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use colored::*;
use chrono::{DateTime, Utc};

// Added derive Debug for better diagnostics
#[derive(Serialize, Deserialize, Debug)]
struct WeatherResponse {
    #[serde(default)]
    main: MainIndicators,
    
    #[serde(default)]
    weather: Vec<WeatherDescription>,
    
    #[serde(default)]
    wind: Wind,
    
    #[serde(default)]
    name: String,
    
    #[serde(default)]
    sys: SystemInfo,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct MainIndicators {
    #[serde(default)]
    temp: f64,
    
    #[serde(default)]
    feels_like: f64,
    
    #[serde(default)]
    humidity: u8,
    
    #[serde(default)]
    pressure: u16,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct WeatherDescription {
    #[serde(default)]
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Wind {
    #[serde(default)]
    speed: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct SystemInfo {
    #[serde(default)]
    country: String,
    
    #[serde(default)]
    sunrise: u64,
    
    #[serde(default)]
    sunset: u64,
}

fn convert_temperature(kelvin: f64) -> (f64, f64) {
    let celsius = kelvin - 273.15;
    let fahrenheit = (celsius * 9.0/5.0) + 32.0;
    (celsius, fahrenheit)
}

fn format_time(timestamp: u64) -> String {
    let datetime = DateTime::<Utc>::from_timestamp(timestamp as i64, 0)
        .unwrap_or_else(|| DateTime::<Utc>::from_timestamp(0, 0).unwrap());
    datetime.format("%H:%M").to_string()
}

fn get_weather_emoji(description: &str) -> &str {
    match description.to_lowercase().as_str() {
        x if x.contains("clear") => "☀️",
        x if x.contains("cloud") => "☁️",
        x if x.contains("rain") => "🌧️",
        x if x.contains("thunderstorm") => "⛈️",
        x if x.contains("snow") => "❄️",
        x if x.contains("fog") => "🌫️",
        _ => "🌈",
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Adding RUST_LOG for detailed diagnostics
    env_logger::init();

    let api_key = match env::var("OPENWEATHERMAP_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: OpenWeatherMap API key not found.");
            eprintln!("Please set the OPENWEATHERMAP_API_KEY environment variable.");
            std::process::exit(1);
        }
    };

    let args: Vec<String> = env::args().collect();
    let city = if args.len() > 1 { &args[1] } else { "Kyiv" };

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&lang=en",
        city, api_key
    );

    let client = reqwest::Client::new();
    let response = match client.get(&url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Network error: {}", e);
            std::process::exit(1);
        }
    };

    // Retrieve response text for diagnostics
    let response_text = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error fetching response text: {}", e);
            std::process::exit(1);
        }
    };

    // Add detailed JSON diagnostics
    println!("Received JSON response: {}", response_text);

    let weather: WeatherResponse = match serde_json::from_str(&response_text) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("JSON parsing error: {}", e);
            eprintln!("Response details: {}", response_text);
            std::process::exit(1);
        }
    };

    // Additional data verification
    if weather.main.temp == 0.0 {
        eprintln!("Warning: Unable to retrieve temperature. Check the city and API key.");
        std::process::exit(1);
    }

    let (temp_celsius, temp_fahrenheit) = convert_temperature(weather.main.temp);
    let (feels_temp_celsius, feels_temp_fahrenheit) = convert_temperature(weather.main.feels_like);

    let default_description = WeatherDescription { description: "Unknown".to_string() };
    let weather_description = weather.weather.first().unwrap_or(&default_description);

    let emoji = get_weather_emoji(&weather_description.description);
    println!("{} Weather Report {}", "🌍".green(), "🌍".green());
    println!("{} {}, {}", emoji, weather.name.blue(), weather.sys.country.blue());
    
    println!("\n{} Weather Conditions:", "📊".yellow());
    println!("   {}: {}", "Status".green(), weather_description.description.yellow());
    println!("   {}: {:.1}°C / {:.1}°F", "Temperature".green(), temp_celsius, temp_fahrenheit);
    println!("   {}: {:.1}°C / {:.1}°F", "Feels like".green(), feels_temp_celsius, feels_temp_fahrenheit);
    
    println!("\n{} Additional Details:", "🌬️".cyan());
    println!("   {}: {}%", "Humidity".green(), weather.main.humidity);
    println!("   {}: {:.1} m/s", "Wind speed".green(), weather.wind.speed);
    println!("   {}: {} hPa", "Pressure".green(), weather.main.pressure);
    
    println!("\n{} Celestial Events:", "🌅".magenta());
    println!("   {}: {}", "Sunrise".green(), format_time(weather.sys.sunrise));
    println!("   {}: {}", "Sunset".green(), format_time(weather.sys.sunset));

    Ok(())
}

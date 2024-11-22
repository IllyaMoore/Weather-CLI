# 🌦️ Rust Weather CLI

## Project Description
A command-line weather application written in Rust, powered by OpenWeatherMap API, providing real-time weather information with a user-friendly interface.

## ✨ Features
- Weather checking for any city worldwide
- Temperature display in Celsius and Fahrenheit
- Comprehensive weather details:
  - Current temperature
  - "Feels like" temperature
  - Humidity percentage
  - Wind speed
  - Atmospheric pressure
  - Sunrise and sunset times
- Colorful terminal output
- Weather condition emoji indicators

## 🛠️ Requirements
- Rust (version 1.70+)
- OpenWeatherMap API key
- Cargo package manager

## 🚀 Installation and Setup

### 1. Clone the Repository
```bash
git clone https://github.com/yourusername/rust-weather-cli.git
cd rust-weather-cli
```

### 2. Obtain API Key
1. Register at [OpenWeatherMap](https://openweathermap.org/)
2. Generate a API key

### 3. Environment Variable Configuration
#### Windows (PowerShell)
```powershell
$env:OPENWEATHERMAP_API_KEY = "your_api_key"
```

#### Linux/macOS
```bash
export OPENWEATHERMAP_API_KEY="your_api_key"
```

## 🖥️ Usage

### Basic Usage
```bash
# Defaults to Kyiv
cargo run
```

### Search Weather for Specific City examples
```bash
cargo run -- "London"
cargo run -- "New York"
cargo run -- "Paris"
```

## 📦 Dependencies
- `reqwest`: HTTP client
- `tokio`: Asynchronous runtime
- `serde`: Serialization/deserialization
- `colored`: Terminal color output
- `chrono`: Time manipulation
- 
## 🔒 Security
- API key stored in environment variable
- Secure JSON parsing
- Error handling for network and parsing issues

## 📃 License
Distributed under the MIT License.

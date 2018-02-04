# WeatherGov

This is a library for accessing weather data from US government sources. At the moment, it can only retrieve current
weather for a given location, though my goal is to add forecast data, and also some methods to help determine which
station a given user should use.

## Example Usage

```rust
fn main() {
    match weathergov::get_current_observation("KATT") {
        Ok(data) => {
            let temperature = match data.temp_c {
                Some(d) => format!("{}°C", d.round() as i64),
                None => "N/A".to_owned()
            };
            let weather = data.weather.unwrap_or("N/A".to_owned());
            println!("{} {}", weather, temperature);
        }
        Err(e) => { println!("Error: {:?}", e); }
    };
}
```
This will print something like:

`Fair 21°C`


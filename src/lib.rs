#[macro_use] extern crate serde_derive;
extern crate reqwest;
extern crate serde_xml_any;
pub mod parse;
pub mod api;


pub fn get_current_observation(station: &str) -> Result<parse::CurrentObservation, WeatherGovError> {
    // Downloads and parses current weather data for a given location.
    // Stations can be looked up at: https://w1.weather.gov/xml/current_obs/
    let xml = api::current_observation(station)?;
    let current_observation = parse::parse_current_observation(&xml)?;
    Ok(current_observation)
}


#[derive(Debug)]
pub enum WeatherGovError {
    APIError(reqwest::Error),
    ParseError(serde_xml_any::Error)
}

impl From<reqwest::Error> for WeatherGovError {
    fn from(err: reqwest::Error) -> WeatherGovError {
        WeatherGovError::APIError(err)
    }
}

impl From<serde_xml_any::Error> for WeatherGovError {
    fn from(err: serde_xml_any::Error) -> WeatherGovError {
        WeatherGovError::ParseError(err)
    }
}

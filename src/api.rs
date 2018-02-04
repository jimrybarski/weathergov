extern crate reqwest;

pub fn current_observation(station: &str) -> Result<String, reqwest::Error> {
    // Retrieves an XML-formatted string that describes the current weather conditions for a given
    // weather station.
    let url = format!("http://w1.weather.gov/xml/current_obs/{}.xml", station);
    let mut res = reqwest::get(&url)?;
    res.text()
}
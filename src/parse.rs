extern crate serde;
extern crate serde_xml_rs;


pub fn parse_current_observation(s: &str) -> Result<CurrentObservation, serde_xml_rs::Error> {
    // Takes an XML-formatted string containing current weather information and casts it into a
    // well-defined struct.
    serde_xml_rs::from_str(s)
}

#[derive(Serialize, Deserialize)]
pub struct CurrentObservation {
    // Because the API is not guaranteed to return every field each time, we have to wrap all of
    // these in Option. Exceptions are made where we can think of good defaults.
    #[serde(default)] pub weather: Option<String>,
    #[serde(default)] pub temp_c: Option<f32>,
    #[serde(default)] pub suggested_pickup_period: u8,
	#[serde(default)] pub location: Option<String>,
	#[serde(default)] pub station_id: Option<String>,
	#[serde(default)] pub latitude: Option<f32>,
	#[serde(default)] pub longitude: Option<f32>,
	#[serde(default)] pub observation_time: Option<String>,
    #[serde(default)] pub observation_time_rfc822: Option<String>,
	#[serde(default)] pub temperature_string: Option<String>,
	#[serde(default)] pub temp_f: Option<f32>,
	#[serde(default)] pub relative_humidity: Option<u16>,
	#[serde(default)] pub wind_string: Option<String>,
	#[serde(default)] pub wind_dir: Option<String>,
	#[serde(default)] pub wind_degrees: Option<u16>,
	#[serde(default)] pub wind_mph: Option<f32>,
	#[serde(default)] pub wind_kt: Option<u16>,
	#[serde(default)] pub pressure_string: Option<String>,
	#[serde(default)] pub pressure_mb: Option<f32>,
	#[serde(default)] pub pressure_in: Option<f32>,
	#[serde(default)] pub dewpoint_string: Option<String>,
	#[serde(default)] pub dewpoint_f: Option<f32>,
	#[serde(default)] pub dewpoint_c: Option<f32>,
	#[serde(default)] pub windchill_string: Option<String>,
    #[serde(default)] pub windchill_f: Option<f32>,
    #[serde(default)] pub windchill_c: Option<f32>,
	#[serde(default)] pub visibility_mi: Option<f32>
}

impl Default for CurrentObservation {
    fn default() -> CurrentObservation {
        CurrentObservation {
            weather: None,
            temp_c: None,
            suggested_pickup_period: 60,
            location: None,
            station_id: None,
            latitude: None,
            longitude: None,
            observation_time: None,
            observation_time_rfc822: None,
            temperature_string: None,
            temp_f: None,
            relative_humidity: None,
            wind_string: None,
            wind_dir: None,
            wind_degrees: None,
            wind_mph: None,
            wind_kt: None,
            pressure_string: None,
            pressure_mb: None,
            pressure_in: None,
            dewpoint_string: None,
            dewpoint_f: None,
            dewpoint_c: None,
            windchill_string: None,
            windchill_f: None,
            windchill_c: None,
            visibility_mi: None
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_deserialize_current_weather() {
        let input = r#"
<?xml version="1.0" encoding="ISO-8859-1"?>
<?xml-stylesheet href="latest_ob.xsl" type="text/xsl"?>
<current_observation version="1.0"
	 xmlns:xsd="http://www.w3.org/2001/XMLSchema"
	 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
	 xsi:noNamespaceSchemaLocation="http://www.weather.gov/view/current_observation.xsd">
	<credit>NOAA's National Weather Service</credit>
	<credit_URL>http://weather.gov/</credit_URL>
	<image>
		<url>http://weather.gov/images/xml_logo.gif</url>
		<title>NOAA's National Weather Service</title>
		<link>http://weather.gov</link>
	</image>
	<suggested_pickup>15 minutes after the hour</suggested_pickup>
	<suggested_pickup_period>60</suggested_pickup_period>
	<location>Austin City, Austin Camp Mabry, TX</location>
	<station_id>KATT</station_id>
	<latitude>30.31667</latitude>
	<longitude>-97.76667</longitude>
	<observation_time>Last Updated on Feb 3 2018, 1:51 pm CST</observation_time>
        <observation_time_rfc822>Sat, 03 Feb 2018 13:51:00 -0600</observation_time_rfc822>
	<weather>Overcast</weather>
	<temperature_string>53.0 F (11.7 C)</temperature_string>
	<temp_f>53.0</temp_f>
	<temp_c>11.7</temp_c>
	<relative_humidity>83</relative_humidity>
	<wind_string>South at 5.8 MPH (5 KT)</wind_string>
	<wind_dir>South</wind_dir>
	<wind_degrees>170</wind_degrees>
	<wind_mph>5.8</wind_mph>
	<wind_kt>5</wind_kt>
	<pressure_string>1016.0 mb</pressure_string>
	<pressure_mb>1016.0</pressure_mb>
	<pressure_in>30.00</pressure_in>
	<dewpoint_string>48.0 F (8.9 C)</dewpoint_string>
	<dewpoint_f>48.0</dewpoint_f>
	<dewpoint_c>8.9</dewpoint_c>
	<windchill_string>51 F (11 C)</windchill_string>
      	<windchill_f>51</windchill_f>
      	<windchill_c>11</windchill_c>
	<visibility_mi>10.00</visibility_mi>
 	<icon_url_base>http://forecast.weather.gov/images/wtf/small/</icon_url_base>
	<two_day_history_url>http://www.weather.gov/data/obhistory/KATT.html</two_day_history_url>
	<icon_url_name>ovc.png</icon_url_name>
	<ob_url>http://www.weather.gov/data/METAR/KATT.1.txt</ob_url>
	<disclaimer_url>http://weather.gov/disclaimer.html</disclaimer_url>
	<copyright_url>http://weather.gov/disclaimer.html</copyright_url>
	<privacy_policy_url>http://weather.gov/notice.html</privacy_policy_url>
</current_observation>"#;
        let current: CurrentObservation = serde_xml_rs::from_str(input).unwrap();
        assert_eq!(current.temp_c, Some(11.7));
        assert_eq!(current.weather, Some("Overcast".to_owned()));
    }

    #[test]
    fn test_deserialize_current_weather_missing_item() {
        let input = r#"
<?xml version="1.0" encoding="ISO-8859-1"?>
<?xml-stylesheet href="latest_ob.xsl" type="text/xsl"?>
<current_observation version="1.0"
	 xmlns:xsd="http://www.w3.org/2001/XMLSchema"
	 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
	 xsi:noNamespaceSchemaLocation="http://www.weather.gov/view/current_observation.xsd">
	<credit>NOAA's National Weather Service</credit>
	<credit_URL>http://weather.gov/</credit_URL>
	<image>
		<url>http://weather.gov/images/xml_logo.gif</url>
		<title>NOAA's National Weather Service</title>
		<link>http://weather.gov</link>
	</image>
	<suggested_pickup>15 minutes after the hour</suggested_pickup>
	<suggested_pickup_period>60</suggested_pickup_period>
	<location>Austin City, Austin Camp Mabry, TX</location>
	<station_id>KATT</station_id>
	<latitude>30.31667</latitude>
	<longitude>-97.76667</longitude>
	<observation_time>Last Updated on Feb 3 2018, 1:51 pm CST</observation_time>
    <observation_time_rfc822>Sat, 03 Feb 2018 13:51:00 -0600</observation_time_rfc822>
	<weather>Overcast</weather>
	<temperature_string>53.0 F (11.7 C)</temperature_string>
	<temp_f>53.0</temp_f>
	<temp_c>11.7</temp_c>
	<relative_humidity>83</relative_humidity>
	<wind_string>South at 5.8 MPH (5 KT)</wind_string>
	<wind_dir>South</wind_dir>
	<wind_degrees>170</wind_degrees>
	<wind_mph>5.8</wind_mph>
	<wind_kt>5</wind_kt>
	<pressure_string>1016.0 mb</pressure_string>
	<pressure_mb>1016.0</pressure_mb>
	<pressure_in>30.00</pressure_in>
	<dewpoint_string>48.0 F (8.9 C)</dewpoint_string>
	<dewpoint_f>48.0</dewpoint_f>
	<dewpoint_c>8.9</dewpoint_c>
	<windchill_string>51 F (11 C)</windchill_string>
    <windchill_f>51</windchill_f>
	<visibility_mi>10.00</visibility_mi>
 	<icon_url_base>http://forecast.weather.gov/images/wtf/small/</icon_url_base>
	<two_day_history_url>http://www.weather.gov/data/obhistory/KATT.html</two_day_history_url>
	<icon_url_name>ovc.png</icon_url_name>
	<ob_url>http://www.weather.gov/data/METAR/KATT.1.txt</ob_url>
	<disclaimer_url>http://weather.gov/disclaimer.html</disclaimer_url>
	<copyright_url>http://weather.gov/disclaimer.html</copyright_url>
	<privacy_policy_url>http://weather.gov/notice.html</privacy_policy_url>
</current_observation>"#;
        let current: CurrentObservation = serde_xml_rs::from_str(input).unwrap();
        assert_eq!(current.temp_c, Some(11.7));
        assert_eq!(current.weather, Some("Overcast".to_owned()));
        assert_eq!(current.windchill_c, None);
    }
}
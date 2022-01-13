use crate::data;
use crate::data::DailyForecast;
use crate::data::Forecast;
use crate::data::Geometry;
use crate::data::Place;
use crate::data::Prevision;
use crate::data::ProbabilityForecast;
use crate::data::Properties;
use crate::errors::MeteoErreurs;
use crate::util::OptionJSONUtils::value_to_f64;
use crate::util::OptionJSONUtils::value_to_i64;
use crate::util::OptionJSONUtils::value_to_string;
use serde_json::Value;

#[allow(non_snake_case)]
/// Fonction qui utilise la bibliothèque Serde JSON pour désérialiser
/// un contenu JSON en structure Place. Parce que notre structure Place est complexe,
/// il est plus efficace (mais plus long à préparer) d'assigner à chaque champ
/// d'une structure Rust son équivalent récupéré depuis le contenu JSON
pub fn JSONToPlace(string: String) -> Result<data::Place, MeteoErreurs> {
    let v = serde_json::from_str::<Value>(string.as_str());
    match v {
        Ok(json_place) => {
            if json_place.is_array() {
                let places = json_place.as_array();
                match places {
                    Some(p) => {
                        if p.len() > 0 {
                            let j = p[0].to_owned();
                            let mut p = Place::new();
                            p.insee = value_to_string(j["insee"].to_owned());
                            p.name = value_to_string(j["name"].to_owned());
                            p.lat = value_to_f64(j["lat"].to_owned());
                            p.lon = value_to_f64(j["lon"].to_owned());
                            p.country = value_to_string(j["country"].to_owned());
                            p.admin = value_to_string(j["admin"].to_owned());
                            p.admin2 = value_to_string(j["admin2"].to_owned());
                            p.postCode = value_to_string(j["postCode"].to_owned());
                            return Ok(p);
                        } else {
                            return Err(MeteoErreurs::StringParseErreur);
                        }
                    }
                    None => Err(MeteoErreurs::StringParseErreur),
                }
            } else {
                return Err(MeteoErreurs::StringParseErreur);
            }
        }
        Err(_) => Err(MeteoErreurs::StringParseErreur),
    }
}

#[allow(non_snake_case)]
/// Fonction qui utilise la bibliothèque Serde JSON pour désérialiser
/// un contenu JSON en structure Prevision.
pub fn JSONToPrevision(string: String) -> Result<data::Prevision, MeteoErreurs> {
    let v = serde_json::from_str::<Value>(string.as_str());
    match v {
        Ok(json_previsions) => {
            let mut prevision = Prevision::new();
            prevision.update_time = value_to_string(json_previsions["update_time"].to_owned());
            prevision.typee = value_to_string(json_previsions["type"].to_owned());

            // Geometry
            if json_previsions["geometry"].is_object() {
                let mut geometry = Geometry::new();
                geometry.typee = value_to_string(json_previsions["geometry"]["type"].to_owned());
                let mut coordinates = vec![];
                if json_previsions["geometry"]["coordinates"].is_array() {
                    match json_previsions["geometry"]["coordinates"].as_array() {
                        Some(v) => {
                            for c in v {
                                let coord = value_to_f64(c.to_owned());
                                match coord {
                                    Some(f) => coordinates.push(f),
                                    None => {}
                                }
                            }
                        }
                        None => {}
                    }
                    if coordinates.len() == 2 {
                        geometry.coordinates = Some(coordinates);
                    } else {
                        geometry.coordinates = None;
                    }
                } else {
                    geometry.coordinates = None;
                }

                prevision.geometry = Some(geometry);
            } else {
                prevision.geometry = None;
            }

            // Properties
            if json_previsions["properties"].is_object() {
                let mut properties = Properties::new();
                properties.altitude =
                    value_to_i64(json_previsions["properties"]["altitude"].to_owned());
                properties.name = value_to_string(json_previsions["properties"]["name"].to_owned());
                properties.country =
                    value_to_string(json_previsions["properties"]["country"].to_owned());
                properties.french_department =
                    value_to_string(json_previsions["properties"]["french_department"].to_owned());
                properties.rain_product_available = value_to_i64(
                    json_previsions["properties"]["rain_product_available"].to_owned(),
                );
                properties.timezone =
                    value_to_string(json_previsions["properties"]["timezone"].to_owned());
                properties.insee =
                    value_to_string(json_previsions["properties"]["insee"].to_owned());
                properties.bulletin_cote =
                    value_to_i64(json_previsions["properties"]["bulletin_cote"].to_owned());

                // Daily Forecast
                if json_previsions["properties"]["daily_forecast"].is_array() {
                    let mut daily_forecast = vec![];
                    match json_previsions["properties"]["daily_forecast"].as_array() {
                        Some(daily) => {
                            for d in daily {
                                let item = d.to_owned();
                                if item.is_object() {
                                    let mut s = DailyForecast::new();
                                    s.time = value_to_string(item["time"].to_owned());
                                    s.t_min = value_to_f64(item["T_min"].to_owned());
                                    s.t_max = value_to_f64(item["T_max"].to_owned());
                                    s.t_sea = value_to_f64(item["T_sea"].to_owned());
                                    s.relative_humidity_max =
                                        value_to_i64(item["relative_humidity_max"].to_owned());
                                    s.relative_humidity_min =
                                        value_to_i64(item["relative_humidity_min"].to_owned());
                                    s.total_precipitation_24h =
                                        value_to_f64(item["total_precipitation_24h"].to_owned());
                                    s.uv_index = value_to_f64(item["uv_index"].to_owned());
                                    s.daily_weather_icon =
                                        value_to_string(item["daily_weather_icon"].to_owned());
                                    s.daily_weather_description = value_to_string(
                                        item["daily_weather_description"].to_owned(),
                                    );
                                    s.sunrise_time =
                                        value_to_string(item["sunrise_time"].to_owned());
                                    s.sunset_time = value_to_string(item["sunset_time"].to_owned());
                                    daily_forecast.push(s);
                                }
                            }
                        }
                        None => {}
                    }

                    if daily_forecast.len() > 0 {
                        properties.daily_forecast = Some(daily_forecast);
                    } else {
                        properties.daily_forecast = None;
                    }
                } else {
                    properties.daily_forecast = None;
                }

                // Forecast
                if json_previsions["properties"]["forecast"].is_array() {
                    let mut forecast = vec![];
                    match json_previsions["properties"]["forecast"].as_array() {
                        Some(fc) => {
                            for d in fc {
                                let item = d.to_owned();
                                if item.is_object() {
                                    let mut s = Forecast::new();
                                    s.time = value_to_string(item["time"].to_owned());
                                    s.t = value_to_f64(item["T"].to_owned());
                                    s.t_windchill = value_to_f64(item["T_windchill"].to_owned());
                                    s.relative_humidity =
                                        value_to_i64(item["relative_humidity"].to_owned());
                                    s.p_sea = value_to_f64(item["P_sea"].to_owned());
                                    s.wind_speed = value_to_f64(item["wind_speed"].to_owned());
                                    s.wind_speed_gust =
                                        value_to_f64(item["wind_speed_gust"].to_owned());
                                    s.wind_direction =
                                        value_to_f64(item["wind_direction"].to_owned());
                                    s.wind_icon = value_to_string(item["wind_icon"].to_owned());
                                    s.rain_1h = value_to_f64(item["rain_1h"].to_owned());
                                    s.rain_3h = value_to_f64(item["rain_3h"].to_owned());
                                    s.rain_6h = value_to_f64(item["rain_6h"].to_owned());
                                    s.rain_12h = value_to_f64(item["rain_12h"].to_owned());
                                    s.rain_24h = value_to_f64(item["rain_24h"].to_owned());
                                    s.snow_1h = value_to_f64(item["snow_1h"].to_owned());
                                    s.snow_3h = value_to_f64(item["snow_3h"].to_owned());
                                    s.snow_6h = value_to_f64(item["snow_6h"].to_owned());
                                    s.snow_12h = value_to_f64(item["snow_12h"].to_owned());
                                    s.snow_24h = value_to_f64(item["snow_24h"].to_owned());
                                    s.iso0 = value_to_i64(item["iso0"].to_owned());
                                    s.rain_snow_limit =
                                        value_to_string(item["rain_snow_limit"].to_owned());
                                    s.total_cloud_cover =
                                        value_to_i64(item["total_cloud_cover"].to_owned());
                                    s.weather_icon =
                                        value_to_string(item["weather_icon"].to_owned());
                                    s.weather_description =
                                        value_to_string(item["weather_description"].to_owned());

                                    forecast.push(s);
                                }
                            }
                        }
                        None => {}
                    }

                    if forecast.len() > 0 {
                        properties.forecast = Some(forecast);
                    } else {
                        properties.forecast = None;
                    }
                } else {
                    properties.forecast = None;
                }

                // Probability Forecast
                if json_previsions["properties"]["probability_forecast"].is_array() {
                    let mut probability = vec![];
                    match json_previsions["properties"]["probability_forecast"].as_array() {
                        Some(fc) => {
                            for d in fc {
                                let item = d.to_owned();
                                if item.is_object() {
                                    let mut s = ProbabilityForecast::new();
                                    s.time = value_to_string(item["time"].to_owned());
                                    s.rain_hazard_3h =
                                        value_to_i64(item["rain_hazard_3h"].to_owned());
                                    s.rain_hazard_6h =
                                        value_to_i64(item["rain_hazard_6h"].to_owned());
                                    s.snow_hazard_3h =
                                        value_to_i64(item["snow_hazard_3h"].to_owned());
                                    s.snow_hazard_6h =
                                        value_to_i64(item["snow_hazard_6h"].to_owned());
                                    s.freezing_hazard =
                                        value_to_i64(item["freezing_hazard"].to_owned());
                                    s.storm_hazard = value_to_i64(item["storm_hazard"].to_owned());
                                    probability.push(s);
                                }
                            }
                        }
                        None => {}
                    }

                    if probability.len() > 0 {
                        properties.probability_forecast = Some(probability);
                    } else {
                        properties.probability_forecast = None;
                    }
                } else {
                    properties.probability_forecast = None;
                }

                prevision.properties = Some(properties);
            } else {
                prevision.properties = None;
            }

            return Ok(prevision);
        }
        Err(_) => Err(MeteoErreurs::StringParseErreur),
    }
}

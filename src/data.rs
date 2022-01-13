use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
/// Structure qui reprend le squelette de l'API Météo France
/// Ici dans le cas des données des villes (sous-repertoire /places/)
pub struct Place {
    pub insee: Option<String>,
    pub name: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub country: Option<String>,
    pub admin: Option<String>,
    pub admin2: Option<String>,
    pub postCode: Option<String>,
}

impl Default for Place {
    fn default() -> Self {
        return Place {
            insee: Some(String::from("75001")),
            name: Some(String::from("Paris 01")),
            lat: Some(48.8592),
            lon: Some(2.3417),
            country: Some(String::from("FR")),
            admin: Some(String::from("Île-de-France")),
            admin2: Some(String::from("75")),
            postCode: Some(String::from("75001")),
        };
    }
}

impl Place {
    pub fn new() -> Self {
        return Place {
            insee: None,
            name: None,
            lat: None,
            lon: None,
            country: None,
            admin: None,
            admin2: None,
            postCode: None,
        };
    }
}

#[derive(Debug, Clone, Serialize)]
/// Structure qui reprend le squelette de l'API Météo France
/// Ici dans le cas des données des prévisions (sous-repertoire /forecast/)
pub struct Prevision {
    pub update_time: Option<String>,
    pub typee: Option<String>,
    pub geometry: Option<Geometry>,
    pub properties: Option<Properties>,
}

impl Prevision {
    pub fn new() -> Self {
        return Prevision {
            update_time: None,
            typee: None,
            geometry: None,
            properties: None,
        };
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Geometry {
    pub typee: Option<String>,
    pub coordinates: Option<Vec<f64>>,
}

impl Geometry {
    pub fn new() -> Self {
        return Geometry {
            typee: None,
            coordinates: None,
        };
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Properties {
    pub altitude: Option<i64>,
    pub name: Option<String>,
    pub country: Option<String>,
    pub french_department: Option<String>,
    pub rain_product_available: Option<i64>,
    pub timezone: Option<String>,
    pub insee: Option<String>,
    pub bulletin_cote: Option<i64>,
    pub daily_forecast: Option<Vec<DailyForecast>>,
    pub forecast: Option<Vec<Forecast>>,
    pub probability_forecast: Option<Vec<ProbabilityForecast>>,
}

impl Properties {
    pub fn new() -> Self {
        return Properties {
            altitude: None,
            name: None,
            country: None,
            french_department: None,
            rain_product_available: None,
            timezone: None,
            insee: None,
            bulletin_cote: None,
            daily_forecast: None,
            forecast: None,
            probability_forecast: None,
        };
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DailyForecast {
    pub time: Option<String>,
    pub t_min: Option<f64>,
    pub t_max: Option<f64>,
    pub t_sea: Option<f64>,
    pub relative_humidity_min: Option<i64>,
    pub relative_humidity_max: Option<i64>,
    pub total_precipitation_24h: Option<f64>,
    pub uv_index: Option<f64>,
    pub daily_weather_icon: Option<String>,
    pub daily_weather_description: Option<String>,
    pub sunrise_time: Option<String>,
    pub sunset_time: Option<String>,
}

impl DailyForecast {
    pub fn new() -> Self {
        return DailyForecast {
            time: None,
            t_min: None,
            t_max: None,
            t_sea: None,
            relative_humidity_min: None,
            relative_humidity_max: None,
            total_precipitation_24h: None,
            uv_index: None,
            daily_weather_icon: None,
            daily_weather_description: None,
            sunrise_time: None,
            sunset_time: None,
        };
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Forecast {
    pub time: Option<String>,
    pub t: Option<f64>,
    pub t_windchill: Option<f64>,
    pub relative_humidity: Option<i64>,
    pub p_sea: Option<f64>,
    pub wind_speed: Option<f64>,
    pub wind_speed_gust: Option<f64>,
    pub wind_direction: Option<f64>,
    pub wind_icon: Option<String>,
    pub rain_1h: Option<f64>,
    pub rain_3h: Option<f64>,
    pub rain_6h: Option<f64>,
    pub rain_12h: Option<f64>,
    pub rain_24h: Option<f64>,
    pub snow_1h: Option<f64>,
    pub snow_3h: Option<f64>,
    pub snow_6h: Option<f64>,
    pub snow_12h: Option<f64>,
    pub snow_24h: Option<f64>,
    pub iso0: Option<i64>,
    pub rain_snow_limit: Option<String>,
    pub total_cloud_cover: Option<i64>,
    pub weather_icon: Option<String>,
    pub weather_description: Option<String>,
}

impl Forecast {
    pub fn new() -> Forecast {
        return Forecast {
            time: None,
            t: None,
            t_windchill: None,
            relative_humidity: None,
            p_sea: None,
            wind_speed: None,
            wind_speed_gust: None,
            wind_direction: None,
            wind_icon: None,
            rain_1h: None,
            rain_3h: None,
            rain_6h: None,
            rain_12h: None,
            rain_24h: None,
            snow_1h: None,
            snow_3h: None,
            snow_6h: None,
            snow_12h: None,
            snow_24h: None,
            iso0: None,
            rain_snow_limit: None,
            total_cloud_cover: None,
            weather_icon: None,
            weather_description: None,
        };
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProbabilityForecast {
    pub time: Option<String>,
    pub rain_hazard_3h: Option<i64>,
    pub rain_hazard_6h: Option<i64>,
    pub snow_hazard_3h: Option<i64>,
    pub snow_hazard_6h: Option<i64>,
    pub freezing_hazard: Option<i64>,
    pub storm_hazard: Option<i64>,
}

impl ProbabilityForecast {
    pub fn new() -> Self {
        return ProbabilityForecast {
            time: None,
            rain_hazard_3h: None,
            rain_hazard_6h: None,
            snow_hazard_3h: None,
            snow_hazard_6h: None,
            freezing_hazard: None,
            storm_hazard: None,
        };
    }
}

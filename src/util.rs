/// Module de fonctions utilitaires permettant de facilier des conversions
#[allow(non_snake_case)]
pub mod OptionJSONUtils {
    pub fn value_to_string(val: serde_json::Value) -> Option<String> {
        if !val.is_null() && val.is_string() {
            return Some(val.as_str().unwrap().to_string());
        } else {
            return None;
        }
    }

    pub fn value_to_f64(val: serde_json::Value) -> Option<f64> {
        if !val.is_null() && val.is_f64() {
            return Some(val.to_string().parse::<f64>().unwrap_or(0.0));
        } else {
            return None;
        }
    }

    pub fn value_to_i64(val: serde_json::Value) -> Option<i64> {
        if !val.is_null() && val.is_i64() {
            return Some(val.to_string().parse::<i64>().unwrap_or(0));
        } else {
            return None;
        }
    }
}

#[allow(non_snake_case)]
pub mod DateHeure {

    use chrono::{DateTime, Utc};

    pub fn get_dateheure_from_str(string: String) -> DateTime<Utc> {
        let parse = string.parse::<DateTime<Utc>>();
        match parse {
            Ok(d) => d,
            Err(_) => Utc::now(),
        }
    }
}

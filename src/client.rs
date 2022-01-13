use crate::data::Place;
use crate::data::Prevision;
use crate::errors::MeteoErreurs;
use crate::parser;
use crate::storage::{self};
use std::collections::HashMap;

mod http;

#[derive(Debug, Clone)]
pub struct MeteoFranceAPI {
    pub base_url: &'static str,
    pub token: &'static str,
}

impl MeteoFranceAPI {
    #[allow(unused_must_use)]
    #[allow(non_snake_case)]
    /// Requêtes HTTP pour trouver une ville, retourne le résultat JSON
    pub fn get_JSON_place(&self, code_postal: i32) -> String {
        let mut arguments = HashMap::new();
        arguments.insert(String::from("q"), String::from(code_postal.to_string()));
        arguments.insert(String::from("token"), String::from(self.token.clone()));
        return http::request(
            String::from(self.base_url.clone()),
            String::from("places"),
            arguments,
        );
    }

    #[allow(unused_must_use)]
    #[allow(non_snake_case)]
    /// Requêtes HTTP pour trouver une prévision météo, retourne le résultat JSON
    pub fn get_JSON_forecast(&self, place: Place) -> Result<String, MeteoErreurs> {
        match place.lat {
            Some(lat) => match place.lon {
                Some(lon) => {
                    let mut arguments = HashMap::new();
                    arguments.insert(String::from("lat"), lat.to_string());
                    arguments.insert(String::from("lon"), lon.to_string());
                    arguments.insert(String::from("token"), String::from(self.token.clone()));
                    return Ok(http::request(
                        String::from(self.base_url.clone()),
                        String::from("forecast"),
                        arguments,
                    ));
                }
                None => Err(MeteoErreurs::BadForecastLocation),
            },
            None => Err(MeteoErreurs::BadForecastLocation),
        }
    }

    /// Requête HTTP puis envoie du contenu JSON dans le parser. Retourne une structure Place
    pub fn get_place(&self, code_postal: i32) -> Result<Place, MeteoErreurs> {
        let json_content = self.get_JSON_place(code_postal);
        let place = parser::JSONToPlace(json_content)?;
        Ok(place)
    }

    /// Requête HTTP puis envoie du contenu JSON dans le parser. Retourne une structure Prevision
    pub fn get_prevision(&self, place: Place) -> Result<Prevision, MeteoErreurs> {
        let json_content = self.get_JSON_forecast(place)?;
        let prevision = parser::JSONToPrevision(json_content)?;
        return Ok(prevision);
    }

    /// Ecriture du fichier de configuration pour changer la ville par défaut
    pub fn changer_default_ville(&self, code_postal: i32) -> bool {
        let place_result = self.get_place(code_postal);
        match place_result {
            Ok(place) => {
                let storage_request = storage::edit_default_place(place);
                match storage_request {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }

    /// Lecture du fichier de configuration pour récupérer la ville par défaut
    pub fn get_default_place(&self) -> Result<Place, MeteoErreurs> {
        let place = storage::get_default_place(self)?;
        Ok(place)
    }
}

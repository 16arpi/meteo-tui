use confy::ConfyError;

use crate::client::MeteoFranceAPI;
use crate::data::Place;
use crate::errors::MeteoErreurs;

/// Lecture de la ville par défaut depuis le fichier de configuration
pub fn get_default_place(_: &MeteoFranceAPI) -> Result<Place, MeteoErreurs> {
    let cfg: Result<Place, ConfyError> = confy::load("meteo");
    match cfg {
        Ok(c) => Ok(c),
        Err(_) => Err(MeteoErreurs::BadPlaceRequest),
    }
}

/// Ecriture de la ville par défaut depuis le fichier de configuration
pub fn edit_default_place(place: Place) -> Result<(), MeteoErreurs> {
    let result = confy::store("meteo", place);
    match result {
        Ok(e) => Ok(e),
        Err(_) => Err(MeteoErreurs::BadPlaceRequest),
    }
}

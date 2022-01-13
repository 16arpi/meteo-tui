/// Enum publique utilisée pour retourner les erreurs des fonctions
/// utilisées par le client de l'API Météo France
pub enum MeteoErreurs {
    BadForecastLocation,
    BadPlaceRequest,
    StringParseErreur,
}

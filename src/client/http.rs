use std::collections::HashMap;
use std::io::Read;

/// Utilisation de la bibliothèque Reqwest
/// et de son module "blocking" permettant d'effectuer des requêtes HTTP
/// de manière synchrone.
/// Cf. https://docs.rs/reqwest/0.11.8/reqwest/blocking/index.html
#[allow(unused_must_use)]
pub fn request(base_url: String, sub_dir: String, arguments: HashMap<String, String>) -> String {
    // URL + /SOUS_REPERTOIRE/
    let mut url_str = base_url + sub_dir.chars().as_str() + "/";

    // Pour chaque argument, l'ajouter à l'URL
    if arguments.len() > 0 {
        url_str += "?";
        let mut c = 0;
        for i in arguments {
            if c > 0 {
                url_str += "&";
            }
            url_str += i.0.chars().as_str();
            url_str += "=";
            url_str += i.1.chars().as_str();
            c += 1;
        }
    }

    // Vérification URL bien construit
    // On execute la commande HTTP
    // On injecte dans notre String le contenu de la page
    let reponse = reqwest::blocking::get(url_str.as_str());
    match reponse {
        Ok(mut resp) => {
            let mut content = String::new();
            resp.read_to_string(&mut content);
            return content;
        }
        Err(_) => {
            return String::from("");
        }
    }
}

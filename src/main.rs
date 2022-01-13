mod ascii_icons;
mod client;
mod data;
mod errors;
mod parser;
mod storage;
mod ui;
mod util;
use std::env;

static METEO_CLIENT: client::MeteoFranceAPI = client::MeteoFranceAPI {
    base_url: "https://webservice.meteofrance.com/",
    token: "__Wj7dVSTjV9YGu1guveLyDq0g7S7TfTjaHBTPTpO0kj8__",
};

#[allow(unused_must_use)]
fn main() {
    let args: Vec<String> = env::args().collect();

    // S'il y a plus d'un argument...
    if args.len() > 1 {
        // ... on récupère les commandes et on agit en conséquence ...
        let command = args[1].clone();
        if command.eq(String::from("carte").as_str()) {
            ui::init(ui::MeteoTabs::TabCarte, None, &METEO_CLIENT);
        } else if command.eq(String::from("help").as_str()) {
            help();
        } else if command.len() == 5 {
            match command.parse::<i32>() {
                Ok(code) => {
                    ui::init(ui::MeteoTabs::TabPrevision, Some(code), &METEO_CLIENT);
                }
                Err(_) => {
                    print!("Code postal incorrect.\n");
                }
            }
        } else {
            print!("Commande inconnue.\n\n");
            help();
        }
    } else {
        // ... sinon on ouvre par défaut l'onglet "prévisions"
        ui::init(ui::MeteoTabs::TabPrevision, None, &METEO_CLIENT);
    }
}

fn help() {
    print!(
        "Commandes possibles :
      meteo                   Ouvrir les prévisions météo de la ville par défaut
      meteo carte             Ouvrir les prévisions météo des principales villes de France
      meteo <code postal>     Ouvrir les prévisions météo d'une ville particulière
    \n"
    );
}

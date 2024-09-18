/*  Module pour l'interface utilisateur dans le terminal
    Cf. https://docs.rs/tui/latest/tui/
*/

use crossterm::event::{self, DisableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::convert::TryInto;
use std::io::{self, Stdout};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::text::{Span, Spans, Text};
use tui::widgets::canvas::{Canvas, Rectangle};
use tui::widgets::{BarChart, Block, Borders, Paragraph, Tabs, Wrap};
use tui::Terminal;
use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::ascii_icons;
use crate::client::MeteoFranceAPI;
use crate::data::{Place, Prevision};
use crate::errors::MeteoErreurs;
use crate::util::DateHeure::{self, get_dateheure_from_str};

#[derive(Clone)]
/// Structure qui permettra de garder en mémoire
/// les infos utiles concernant les villes françaises
/// affichées sur la carte.
///
/// On aura besoin de leurs prévisions météo, mais aussi de leur coordonnées
/// sur la carte, de leur position relatives aux autres villes etc.
struct FrancePrevision {
    code: i32,
    name: &'static str,
    prev: Option<Prevision>,
    lat: f64,
    lon: f64,
    x: f32,
    y: f32,
    left_to: i32,
    right_to: i32,
    down_to: i32,
    up_to: i32,
}

/// Au démarrage, on génére un vecteur des principales villes de France
impl FrancePrevision {
    fn generate() -> Vec<FrancePrevision> {
        return vec![
            FrancePrevision {
                code: 75001,
                name: "Paris",
                prev: None,
                lat: 48.8592,
                lon: 2.3417,
                x: 85.0,
                y: 120.0,
                left_to: 5,
                right_to: 6,
                down_to: 2,
                up_to: 8,
            },
            FrancePrevision {
                code: 13000,
                name: "Marseille",
                prev: None,
                lat: 43.29667,
                lon: 5.37639,
                x: 115.0,
                y: 30.0,
                left_to: 3,
                right_to: 4,
                down_to: 8,
                up_to: 2,
            },
            FrancePrevision {
                code: 69000,
                name: "Lyon",
                prev: None,
                lat: 0.0,
                lon: 0.0,
                x: 110.0,
                y: 73.0,
                left_to: 7,
                right_to: 7,
                down_to: 1,
                up_to: 0,
            },
            FrancePrevision {
                code: 31000,
                name: "Toulouse",
                prev: None,
                lat: 45.75889,
                lon: 4.84139,
                x: 70.0,
                y: 40.0,
                left_to: 4,
                right_to: 1,
                down_to: 3,
                up_to: 7,
            },
            FrancePrevision {
                code: 06000,
                name: "Nice",
                prev: None,
                lat: 43.70194,
                lon: 7.26833,
                x: 142.0,
                y: 38.0,
                left_to: 1,
                right_to: 3,
                down_to: 1,
                up_to: 2,
            },
            FrancePrevision {
                code: 44000,
                name: "Nantes",
                prev: None,
                lat: 0.0,
                lon: 0.0,
                x: 40.0,
                y: 98.0,
                left_to: 6,
                right_to: 0,
                down_to: 7,
                up_to: 0,
            },
            FrancePrevision {
                code: 67000,
                name: "Strasbourg",
                prev: None,
                lat: 48.58361,
                lon: 7.74806,
                x: 140.0,
                y: 123.0,
                left_to: 0,
                right_to: 5,
                down_to: 2,
                up_to: 8,
            },
            FrancePrevision {
                code: 33000,
                name: "Bordeaux",
                prev: None,
                lat: 44.83778,
                lon: -0.57944,
                x: 48.0,
                y: 58.0,
                left_to: 2,
                right_to: 2,
                down_to: 3,
                up_to: 5,
            },
            FrancePrevision {
                code: 59000,
                name: "Lille",
                prev: None,
                lat: 50.63194,
                lon: 3.0575,
                x: 90.0,
                y: 155.0,
                left_to: 5,
                right_to: 6,
                down_to: 0,
                up_to: 1,
            },
            /*FrancePrevision {
                code: 14000,
                name: "Caen",
                prev: None,
                lat: 49.18222,
                lon: -0.37056,
                x: 53.0,
                y: 130.0,
                left_to: 0,
                right_to: 0,
                down_to: 0,
                up_to: 0
            }
            FrancePrevision {
                code: 34000,
                name: "Montepellier",
                prev: None,
                lat: 47.21722,
                lon: -1.55389,
                x: 85.0,
                y: 25.0,
                left_to: 0,
                right_to: 0,
                down_to: 0,
                up_to: 0
            },*/
        ];
    }
}

#[derive(Clone)]
pub enum MeteoTabs {
    TabCarte,
    TabPrevision,
}

#[derive(Clone)]
enum Mode {
    InputMode,
    ReadMode,
}

#[derive(Clone)]
/// Tui-rs fonctionne grâce à une boucle qui actualise à chaque tour l'état du terminal.
/// L'interaction utilisateur est possible grâce à une structure qui garde en mémoire
/// l'état actuel de l'interface : onglet ouvert, éléments affichés etc.
/// Dans notre cas, il s'agit de MeteoApp.
pub struct MeteoApp {
    tab: MeteoTabs,
    mode: Mode,
    search: String,
    search_error: bool,
    search_error_message: String,
    previsions_france: Vec<FrancePrevision>,
    previsions_france_selected: i32,
    prevision: Option<Prevision>,
    default_place: Place,
    actual_place: Place,
    exit: bool,
}

impl MeteoApp {
    fn new(meteo_client: &MeteoFranceAPI, postcode: Option<i32>) -> Result<MeteoApp, MeteoErreurs> {
        let mut p = meteo_client.get_default_place()?;
        let default_place = p.clone();

        p = match postcode {
            Some(np) => match meteo_client.get_place(np) {
                Ok(nnp) => nnp,
                Err(_) => p,
            },
            None => p,
        };

        let mut previsions_france = FrancePrevision::generate();
        for c in &mut previsions_france {
            let place = Place {
                name: Some(c.name.to_string()),
                insee: Some(String::new()),
                lat: Some(c.lat),
                lon: Some(c.lon),
                country: Some(String::new()),
                admin: Some(String::new()),
                admin2: Some(String::new()),
                postCode: Some(c.code.to_string()),
            };
            let prev_option = meteo_client.get_prevision(place)?;
            c.prev = Some(prev_option);
        }

        let prevision = match meteo_client.get_prevision(p.clone()) {
            Ok(prev) => Some(prev),
            Err(_) => None,
        };

        return Ok(MeteoApp {
            tab: MeteoTabs::TabCarte,
            mode: Mode::ReadMode,
            search: String::new(),
            search_error: false,
            search_error_message: String::new(),
            previsions_france: previsions_france,
            previsions_france_selected: 0,
            prevision: prevision,
            default_place: default_place.clone(),
            actual_place: p,
            exit: false,
        });
    }

    fn tab_key(&mut self) {
        self.tab = match self.tab {
            MeteoTabs::TabCarte => MeteoTabs::TabPrevision,
            MeteoTabs::TabPrevision => MeteoTabs::TabCarte,
        };
    }

    fn search(&mut self) {
        match self.mode {
            Mode::ReadMode => {
                self.mode = Mode::InputMode;
                self.search_error = false;
            }
            _ => {}
        }

        match self.tab {
            MeteoTabs::TabCarte => {
                self.tab = MeteoTabs::TabCarte;
            }
            _ => {}
        }
    }

    fn esc(&mut self) {
        match self.mode {
            Mode::InputMode => self.mode = Mode::ReadMode,
            _ => self.exit = true,
        }
    }

    fn fav(&mut self, client: &MeteoFranceAPI) {
        let code = self
            .actual_place
            .postCode
            .to_owned()
            .unwrap()
            .parse::<i32>();
        match code {
            Ok(c) => {
                client.changer_default_ville(c);
                self.default_place = self.actual_place.clone();
            }
            Err(_) => {}
        }
    }

    fn enter(&mut self, client: &MeteoFranceAPI) {
        match self.mode {
            Mode::InputMode => {
                self.mode = Mode::ReadMode;
                let code_postal = self.search.parse::<i32>();
                match code_postal {
                    Ok(c) => {
                        let new_prevision = client.get_place(c);
                        match new_prevision {
                            Ok(p) => {
                                match client.get_prevision(p.clone()) {
                                    Ok(prev) => {
                                        self.prevision = Some(prev);
                                        self.actual_place = p;
                                        self.tab = MeteoTabs::TabPrevision;
                                        self.search_error = false
                                    }
                                    Err(_) => {
                                        // Si aucune prévision trouvée
                                        self.search_error = true;
                                        self.search_error_message = String::from("Erreur")
                                    }
                                }
                            }
                            Err(_) => {
                                // Si aucun endroit trouvé dans l'API météo France
                                self.search_error = true;
                                self.search_error_message =
                                    String::from("Erreur : code postal incorrect");
                            }
                        }
                    }
                    Err(_) => {
                        // Si la recherche n'est pas un entier
                        self.search_error = true;
                        self.search_error_message = String::from("Erreur : entrez un code postal")
                    }
                }
            }
            _ => {}
        }
    }

    fn select(&mut self, client: &MeteoFranceAPI) {
        let i: usize = self.previsions_france_selected.try_into().unwrap();
        let ville = self.previsions_france[i].clone();

        self.tab = MeteoTabs::TabPrevision;
        let new_prevision = client.get_place(ville.code);
        match new_prevision {
            Ok(p) => {
                match client.get_prevision(p.clone()) {
                    Ok(prev) => {
                        self.prevision = Some(prev);
                        self.search = ville.code.to_string();
                        self.actual_place = p;
                        self.search_error = false
                    }
                    Err(_) => {
                        // Si aucune prévision trouvée
                        self.search_error = true;
                        self.search_error_message = String::from("Erreur")
                    }
                }
            }
            Err(_) => {
                // Si aucun endroit trouvé dans l'API météo France
                self.search_error = true;
                self.search_error_message = String::from("Erreur : code postal incorrect");
            }
        }
    }

    fn left(&mut self) {
        match self.tab {
            MeteoTabs::TabCarte => {
                let i: usize = self.previsions_france_selected.try_into().unwrap();
                let ville = self.previsions_france[i].clone();

                self.previsions_france_selected = ville.left_to;
            }
            _ => {}
        }
    }

    fn right(&mut self) {
        match self.tab {
            MeteoTabs::TabCarte => {
                let i: usize = self.previsions_france_selected.try_into().unwrap();
                let ville = self.previsions_france[i].clone();

                self.previsions_france_selected = ville.right_to;
            }
            _ => {}
        }
    }

    fn up(&mut self) {
        match self.tab {
            MeteoTabs::TabCarte => {
                let i: usize = self.previsions_france_selected.try_into().unwrap();
                let ville = self.previsions_france[i].clone();

                self.previsions_france_selected = ville.up_to;
            }
            _ => {}
        }
    }

    fn down(&mut self) {
        match self.tab {
            MeteoTabs::TabCarte => {
                let i: usize = self.previsions_france_selected.try_into().unwrap();
                let ville = self.previsions_france[i].clone();

                self.previsions_france_selected = ville.down_to;
            }
            _ => {}
        }
    }
}

pub fn init(
    opening_tab: MeteoTabs,
    opening_postcode: Option<i32>,
    meteo_client: &MeteoFranceAPI,
) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    draw_fullpage_message(&mut terminal, "Chargement...", true);

    let meteo_app = MeteoApp::new(meteo_client, opening_postcode);

    match meteo_app {
        Ok(mut app) => {
            app.tab = opening_tab;
            ui(&mut terminal, &mut app, meteo_client)?
        },
        Err(_) => {
            draw_fullpage_message(&mut terminal, "Erreur, impossible de lancer le programme. L'erreur peut être : mauvaise connexion internet; impossible de récupérer les données météo france; droits utilisateur non accordés pour enregistrer la configuration.", false)
        },
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn ui(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    meteo_app: &mut MeteoApp,
    meteo_client: &MeteoFranceAPI,
) -> Result<(), io::Error> {
    loop {
        // "Dessin" de notre UI
        terminal.draw(|f| draw(f, meteo_app))?;

        // Gestion des touches du clavier
        if let Event::Key(key) = event::read()? {
            match meteo_app.tab {
                MeteoTabs::TabPrevision => match meteo_app.mode {
                    Mode::ReadMode => match key.code {
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('s') => {
                            meteo_app.search();
                        }
                        KeyCode::Char('f') => {
                            meteo_app.fav(meteo_client);
                        }
                        KeyCode::Esc => {
                            meteo_app.esc();
                        }
                        KeyCode::Tab => {
                            meteo_app.tab_key();
                        }
                        KeyCode::Enter => meteo_app.enter(meteo_client),
                        _ => {}
                    },
                    Mode::InputMode => match key.code {
                        KeyCode::Esc => {
                            meteo_app.esc();
                        }
                        KeyCode::Enter => {
                            meteo_app.enter(meteo_client);
                        }
                        KeyCode::Backspace => {
                            meteo_app.search.pop();
                        }
                        KeyCode::Char(c) => {
                            meteo_app.search.push(c);
                        }
                        _ => {}
                    },
                },
                MeteoTabs::TabCarte => match meteo_app.mode {
                    Mode::ReadMode => match key.code {
                        KeyCode::Enter => {
                            meteo_app.select(meteo_client);
                        }
                        KeyCode::Left => {
                            meteo_app.left();
                        }
                        KeyCode::Right => {
                            meteo_app.right();
                        }
                        KeyCode::Up => {
                            meteo_app.up();
                        }
                        KeyCode::Down => {
                            meteo_app.down();
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('s') => {
                            meteo_app.search();
                        }
                        KeyCode::Tab => {
                            meteo_app.tab_key();
                        }
                        _ => {}
                    },
                    Mode::InputMode => match key.code {
                        KeyCode::Esc => {
                            meteo_app.esc();
                        }
                        KeyCode::Enter => {
                            meteo_app.enter(meteo_client);
                        }
                        KeyCode::Backspace => {
                            meteo_app.search.pop();
                        }
                        KeyCode::Char(c) => {
                            meteo_app.search.push(c);
                        }
                        _ => {}
                    },
                },
            }
        }

        if meteo_app.exit {
            return Ok(());
        }
    }
}

/// Fonction pour afficher un message en plein écran.
/// Peut prendre le parallèle booléen "instant" qui détermine
/// si le message doit rester à l'écran éternellement ou non
fn draw_fullpage_message(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mess: &str,
    instant: bool,
) {
    loop {
        // "Dessin" de notre UI
        let _ = terminal.draw(|f| {
            let size = f.size();
            let frame = Layout::default()
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .direction(Direction::Vertical)
                .split(size);
            let charging_text = Spans::from(mess);
            let paragraph = Paragraph::new(charging_text.clone()).wrap(Wrap { trim: true });
            f.render_widget(paragraph, frame[0])
        });

        if instant {
            return;
        }

        // Gestion des touches du clavier
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                _ => {
                    return;
                }
            }
        }
    }
}

fn draw<B: Backend>(f: &mut Frame<B>, meteo_app: &mut MeteoApp) {
    // Encadré principal (top bar, contenu)
    let main_frame = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    // Encadré du haut (navigation, recherche)
    let sub_top_frame = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(main_frame[0]);

    // Tabs
    let tabs_names = vec![Spans::from("Carte"), Spans::from("Prévisions")];
    let block_navigation = Block::default().title("Navigation").borders(Borders::ALL);
    let tabs = Tabs::new(tabs_names)
        .block(block_navigation)
        .select(match meteo_app.tab {
            MeteoTabs::TabCarte => 0,
            MeteoTabs::TabPrevision => 1,
        })
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Yellow),
        );
    f.render_widget(tabs, sub_top_frame[0]);

    // Si touche "s" préssé (mode recherche)
    let block_search = Block::default()
        .title(match meteo_app.search_error {
            false => "Recherche (code postal)".as_ref(),
            true => meteo_app.search_error_message.as_ref(),
        })
        .borders(Borders::ALL);
    let search_input = Paragraph::new(meteo_app.search.as_ref())
        .style(match meteo_app.mode {
            Mode::InputMode => Style::default().fg(Color::Yellow),
            Mode::ReadMode => match meteo_app.search_error {
                true => Style::default().fg(Color::Red),
                false => Style::default(),
            },
        })
        .block(block_search);
    f.render_widget(search_input, sub_top_frame[1]);

    // Le placement du curseur à la fin du texte est une solution
    // trouvée depuis l'exemple "user input" de tui-rs
    match meteo_app.mode {
        Mode::InputMode => f.set_cursor(
            sub_top_frame[1].x + meteo_app.search.width() as u16 + 1,
            sub_top_frame[1].y + 1,
        ),
        Mode::ReadMode => {}
    }

    // Selon l'onglet sélectionné, on exécute des fonctions différentes
    match meteo_app.tab {
        MeteoTabs::TabCarte => onglet_carte(f, meteo_app, main_frame[1]),
        MeteoTabs::TabPrevision => onglet_prevision(f, meteo_app, main_frame[1]),
    }
}

/// Fonction pour afficher l'interface de la carte de France
fn onglet_carte<B: Backend>(f: &mut Frame<B>, meteo_app: &mut MeteoApp, frame: Rect) {
    let sub_frame = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)].as_ref())
        .horizontal_margin(2)
        .vertical_margin(1)
        .split(frame);

    let rect_main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .margin(1)
        .split(sub_frame[0]);

    let help_text = vec![
        Span::raw("Utilisez les touches "),
        Span::styled(
            "←",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(", "),
        Span::styled(
            "↑",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(", "),
        Span::styled(
            "→",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" et "),
        Span::styled(
            "↓",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" pour changer de ville."),
        Span::styled(
            " ENTRER",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" pour sélectionner la ville."),
        Span::styled(
            " TAB",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" pour changer d'onglet."),
        Span::styled(
            " q",
            Style::default()
                .fg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" pour quitter le programme."),
    ];
    let help_paragraph = Paragraph::new(Text::from(Spans::from(help_text)))
        //.block(Block::default().title("Aide").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(help_paragraph, sub_frame[1]);

    let block = Block::default()
        .title("Météo en France")
        .borders(Borders::ALL);
    f.render_widget(block, frame);

    // Block carte France
    let app_copy = meteo_app.clone();
    let france_canvas = Canvas::default()
        .paint(move |ctx| {
            let coords = ascii_icons::get_france_coords();
            for c in coords {
                // coords: &[*c],
                ctx.draw(&Rectangle {
                    x: c.0,
                    y: c.1,
                    width: 1.0,
                    height: 1.0,
                    color: Color::White,
                })
            }

            let mut i = 0;
            let previsions = app_copy.previsions_france.clone();
            let ville_selected = app_copy.previsions_france_selected;
            for p in previsions {
                let _color = if ville_selected == i {
                    Color::LightMagenta
                } else {
                    Color::White
                };

                ctx.print(p.x.into(), p.y.into(), p.name);

                i += 1;
            }
        })
        .x_bounds([0.0, 170.0])
        .y_bounds([0.0, 170.0]);
    f.render_widget(france_canvas, rect_main[0]);

    // Block prévision
    // Block meteo now
    let cases_meteo_now = Layout::default()
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .horizontal_margin(4)
        .vertical_margin(1)
        .split(rect_main[1]);

    let i: usize = meteo_app.previsions_france_selected.try_into().unwrap();
    let prevs = meteo_app.previsions_france.clone();
    let prev = prevs[i].prev.to_owned();

    match prev {
        Some(prev) => {
            let today_prev = prev
                .properties
                .to_owned()
                .unwrap()
                .daily_forecast
                .to_owned()
                .unwrap()[0]
                .to_owned();

            let date_prevision = Paragraph::new(Text::styled(
                prevs[i].name,
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .alignment(Alignment::Center);
            f.render_widget(date_prevision, cases_meteo_now[0]);

            let today_icon = Canvas::default()
                .paint(|ctx| {
                    let icon_str = today_prev.daily_weather_icon.to_owned().unwrap();
                    let coords = &mut ascii_icons::get_coords_icon_from_str(icon_str);
                    for c in coords {
                        // coords: &[*c],
                        ctx.draw(&Rectangle {
                            x: c.0,
                            y: c.1,
                            width: 4.0,
                            height: 4.0,
                            color: Color::White,
                        })
                    }
                })
                .x_bounds([0.0, 160.0])
                .y_bounds([0.0, 160.0]);
            f.render_widget(today_icon, cases_meteo_now[1]);

            let today_meteo_rect = Layout::default()
                .constraints(vec![
                    Constraint::Length(1), // Temps écrit
                    Constraint::Length(1), // Temperature
                    Constraint::Length(1), // Humidity
                    Constraint::Length(1), // Vent
                ])
                .horizontal_margin(2)
                .vertical_margin(0)
                .split(cases_meteo_now[2]);

            f.render_widget(
                Paragraph::new(Text::styled(
                    today_prev
                        .daily_weather_description
                        .to_owned()
                        .unwrap_or(String::new()),
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .alignment(Alignment::Center),
                today_meteo_rect[0],
            );

            let today_meteo_temp = vec![
                Span::raw("↑ "),
                Span::styled(
                    today_prev.t_max.to_owned().unwrap_or(0.0).to_string(),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("°c ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("↓ "),
                Span::styled(
                    today_prev.t_min.to_owned().unwrap_or(0.0).to_string(),
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("°c", Style::default().add_modifier(Modifier::BOLD)),
            ];
            f.render_widget(
                Paragraph::new(Text::from(Spans::from(today_meteo_temp)))
                    .alignment(Alignment::Center),
                today_meteo_rect[1],
            );

            let today_meteo_humidity = vec![
                Span::raw("• "),
                Span::raw(
                    today_prev
                        .relative_humidity_min
                        .to_owned()
                        .unwrap_or(0)
                        .to_string(),
                ),
                Span::raw("-"),
                Span::raw(
                    today_prev
                        .relative_humidity_max
                        .to_owned()
                        .unwrap_or(0)
                        .to_string(),
                ),
                Span::raw("% humidité"),
            ];
            f.render_widget(
                Paragraph::new(Text::from(Spans::from(today_meteo_humidity)))
                    .alignment(Alignment::Center),
                today_meteo_rect[2],
            );

            let sunrise_time =
                DateHeure::get_dateheure_from_str(today_prev.sunrise_time.to_owned().unwrap());
            let sunset_time =
                DateHeure::get_dateheure_from_str(today_prev.sunset_time.to_owned().unwrap());
            let today_meteo_sun = vec![
                Span::styled("☀ ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    sunrise_time.format("%Hh%M").to_string(),
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(" - "),
                Span::styled("☾ ", Style::default().fg(Color::LightCyan)),
                Span::styled(
                    sunset_time.format("%Hh%M").to_string(),
                    Style::default().fg(Color::LightCyan),
                ),
            ];
            f.render_widget(
                Paragraph::new(Text::from(Spans::from(today_meteo_sun)))
                    .alignment(Alignment::Center),
                today_meteo_rect[3],
            );
        }
        None => {}
    }
}

/// Fonction pour afficher l'interface des prévisions météo d'une ville
fn onglet_prevision<B: Backend>(f: &mut Frame<B>, meteo_app: &mut MeteoApp, frame: Rect) {
    match meteo_app.prevision.to_owned() {
        Some(prev) => {
            let rect_main = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(frame);

            let rect_sub_left = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(rect_main[0]);

            let rect_sub_right = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(rect_main[1]);

            // Blocks

            // Block meteo now
            let cases_meteo_now = Layout::default()
                .constraints(vec![
                    Constraint::Length(1),
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ])
                .horizontal_margin(4)
                .vertical_margin(1)
                .split(rect_sub_left[0]);

            let mut nom_ville = prev
                .properties
                .to_owned()
                .unwrap()
                .name
                .unwrap_or(String::from("Ville"));

            if meteo_app.default_place.postCode == meteo_app.actual_place.postCode {
                nom_ville.push_str(String::from(" (par défaut)").as_str());
            }

            let block_meteo_now = Block::default().title(nom_ville).borders(Borders::ALL);
            f.render_widget(block_meteo_now, rect_sub_left[0]);

            let help_text = vec![
                Span::raw("Pressez "),
                Span::styled(
                    "s ",
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("(puis "),
                Span::styled(
                    "ENTRER",
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(") pour rechercher une ville, "),
                Span::styled(
                    "f ",
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("pour changer la ville au démarrage, "),
                Span::styled(
                    "TAB",
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" pour changer d'onglet et "),
                Span::styled(
                    "q ",
                    Style::default()
                        .fg(Color::LightMagenta)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("pour quitter le programme."),
                Span::raw(""),
            ];
            let help_paragraph = Paragraph::new(Text::from(Spans::from(help_text)))
                .block(Block::default().title("Aide").borders(Borders::ALL))
                .wrap(Wrap { trim: true });
            f.render_widget(help_paragraph, rect_sub_left[1]);

            let block_meteo_jours = Block::default()
                .title("Prévisions".as_ref())
                .borders(Borders::ALL);
            f.render_widget(block_meteo_jours, rect_sub_right[0]);

            let block_meteo_temperatures = Block::default()
                .title("Températures".as_ref())
                .borders(Borders::ALL);
            f.render_widget(block_meteo_temperatures, rect_sub_right[1]);

            let today_prev = prev
                .properties
                .to_owned()
                .unwrap()
                .daily_forecast
                .to_owned()
                .unwrap()[0]
                .to_owned();

            let date_prevision = Paragraph::new(Text::styled(
                "Aujourd'hui",
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .alignment(Alignment::Center);
            f.render_widget(date_prevision, cases_meteo_now[0]);

            let today_icon = Canvas::default()
                .paint(|ctx| {
                    let icon_str = today_prev.daily_weather_icon.to_owned().unwrap();
                    let coords = &mut ascii_icons::get_coords_icon_from_str(icon_str);
                    for c in coords {
                        // coords: &[*c],
                        ctx.draw(&Rectangle {
                            x: c.0,
                            y: c.1,
                            width: 4.0,
                            height: 4.0,
                            color: Color::White,
                        })
                    }
                })
                .x_bounds([0.0, 160.0])
                .y_bounds([0.0, 160.0]);
            f.render_widget(today_icon, cases_meteo_now[1]);

            let today_meteo_rect = Layout::default()
                .constraints(vec![
                    Constraint::Length(1), // Temps écrit
                    Constraint::Length(1), // Temperature
                    Constraint::Length(1), // Humidity
                    Constraint::Length(1), // Vent
                ])
                .horizontal_margin(2)
                .vertical_margin(0)
                .split(cases_meteo_now[2]);

            f.render_widget(
                Paragraph::new(Text::styled(
                    today_prev
                        .daily_weather_description
                        .to_owned()
                        .unwrap_or(String::new()),
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .alignment(Alignment::Center),
                today_meteo_rect[0],
            );

            let today_meteo_temp = vec![
                Span::raw("↑ "),
                Span::styled(
                    today_prev.t_max.to_owned().unwrap_or(0.0).to_string(),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("°c ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("↓ "),
                Span::styled(
                    today_prev.t_min.to_owned().unwrap_or(0.0).to_string(),
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("°c", Style::default().add_modifier(Modifier::BOLD)),
            ];
            f.render_widget(
                Paragraph::new(Text::from(Spans::from(today_meteo_temp)))
                    .alignment(Alignment::Center),
                today_meteo_rect[1],
            );

            let today_meteo_humidity = vec![
                Span::raw("• "),
                Span::raw(
                    today_prev
                        .relative_humidity_min
                        .to_owned()
                        .unwrap_or(0)
                        .to_string(),
                ),
                Span::raw("-"),
                Span::raw(
                    today_prev
                        .relative_humidity_max
                        .to_owned()
                        .unwrap_or(0)
                        .to_string(),
                ),
                Span::raw("% humidité"),
            ];
            f.render_widget(
                Paragraph::new(Text::from(Spans::from(today_meteo_humidity)))
                    .alignment(Alignment::Center),
                today_meteo_rect[2],
            );

            let sunrise_time =
                DateHeure::get_dateheure_from_str(today_prev.sunrise_time.to_owned().unwrap());
            let sunset_time =
                DateHeure::get_dateheure_from_str(today_prev.sunset_time.to_owned().unwrap());
            let today_meteo_sun = vec![
                Span::styled("☀ ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    sunrise_time.format("%Hh%M").to_string(),
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(" - "),
                Span::styled("☾ ", Style::default().fg(Color::LightCyan)),
                Span::styled(
                    sunset_time.format("%Hh%M").to_string(),
                    Style::default().fg(Color::LightCyan),
                ),
            ];
            f.render_widget(
                Paragraph::new(Text::from(Spans::from(today_meteo_sun)))
                    .alignment(Alignment::Center),
                today_meteo_rect[3],
            );

            // Block météo prochains jours (rect $rect_sub_right)
            let cases_daily_forecast = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ])
                .margin(2)
                .split(rect_sub_right[0]);

            for i in 0..8 {
                let case_frame = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Percentage(50),
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Percentage(50),
                        ]
                        .as_ref(),
                    )
                    .split(cases_daily_forecast[i % 4]);

                let col = if i < 4 { 0 } else { 3 };

                let today_prev = prev
                    .properties
                    .to_owned()
                    .unwrap()
                    .daily_forecast
                    .to_owned()
                    .unwrap()[i + 1]
                    .to_owned();

                let str_date_prevision =
                    get_dateheure_from_str(today_prev.time.to_owned().unwrap_or(String::new()));
                let date_prevision = Paragraph::new(Text::styled(
                    str_date_prevision.format("%d/%m").to_string(),
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ))
                .alignment(Alignment::Center);
                f.render_widget(date_prevision, case_frame[col]);

                let today_meteo_rect = Layout::default()
                    .constraints(vec![
                        Constraint::Length(1), // Temps écrit
                        Constraint::Length(1), // Temperature
                        Constraint::Length(1), // Humidity
                        Constraint::Length(1), // Vent
                    ])
                    .horizontal_margin(2)
                    .split(case_frame[col + 2]);

                f.render_widget(
                    Paragraph::new(Text::styled(
                        today_prev
                            .daily_weather_description
                            .to_owned()
                            .unwrap_or(String::new()),
                        Style::default().add_modifier(Modifier::BOLD),
                    )),
                    today_meteo_rect[0],
                );

                let today_meteo_temp = vec![
                    Span::raw("↑ "),
                    Span::styled(
                        today_prev.t_max.to_owned().unwrap_or(0.0).to_string(),
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("°c ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("↓ "),
                    Span::styled(
                        today_prev.t_min.to_owned().unwrap_or(0.0).to_string(),
                        Style::default()
                            .fg(Color::LightBlue)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("°c", Style::default().add_modifier(Modifier::BOLD)),
                ];
                f.render_widget(
                    Paragraph::new(Text::from(Spans::from(today_meteo_temp))),
                    today_meteo_rect[1],
                );

                let today_meteo_humidity = vec![
                    Span::raw("• "),
                    Span::raw(
                        today_prev
                            .relative_humidity_min
                            .to_owned()
                            .unwrap_or(0)
                            .to_string(),
                    ),
                    Span::raw("-"),
                    Span::raw(
                        today_prev
                            .relative_humidity_max
                            .to_owned()
                            .unwrap_or(0)
                            .to_string(),
                    ),
                    Span::raw("% humidité"),
                ];
                f.render_widget(
                    Paragraph::new(Text::from(Spans::from(today_meteo_humidity))),
                    today_meteo_rect[2],
                );

                let sunrise_time =
                    DateHeure::get_dateheure_from_str(today_prev.sunrise_time.to_owned().unwrap());
                let sunset_time =
                    DateHeure::get_dateheure_from_str(today_prev.sunset_time.to_owned().unwrap());
                let today_meteo_sun = vec![
                    Span::styled("☀ ", Style::default().fg(Color::Yellow)),
                    Span::styled(
                        sunrise_time.format("%Hh%M").to_string(),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::raw(" - "),
                    Span::styled("☾ ", Style::default().fg(Color::LightCyan)),
                    Span::styled(
                        sunset_time.format("%Hh%M").to_string(),
                        Style::default().fg(Color::LightCyan),
                    ),
                ];
                f.render_widget(
                    Paragraph::new(Text::from(Spans::from(today_meteo_sun))),
                    today_meteo_rect[3],
                );
            }

            // Block Températures (rect_sub_right)
            let mut daily_forecast_vec =
                prev.properties.unwrap().daily_forecast.to_owned().unwrap();
            daily_forecast_vec.remove(0);

            let mut data_string: Vec<(String, u64)> = vec![];
            for d in daily_forecast_vec {
                let mut temp = 0.0;
                let t_min = d.t_min.unwrap_or(99.1);
                let t_max = d.t_min.unwrap_or(99.1);

                if t_min > 99.0 && t_max > 99.0 {
                    continue;
                } else if t_min > 99.0 {
                    temp = t_max;
                } else if t_max > 99.0 {
                    temp = t_min;
                } else {
                    temp = (t_min + t_max) / 2.0;
                }

                let date = get_dateheure_from_str(d.time.to_owned().unwrap());
                let string_date = date.format("%d/%m").to_string();
                data_string.push((string_date, temp as u64))
            }

            // Expliquer le gros casse-tête ici
            // Cf. https://stackoverflow.com/questions/55931401/why-cant-i-return-a-vecstr-from-a-function
            let data: Vec<(&str, u64)> = data_string.iter().map(|(n, d)| (&**n, *d)).collect();

            let barchart = BarChart::default()
                .block(
                    Block::default()
                        .title("Températures moyennes (en °c)")
                        .borders(Borders::ALL),
                )
                .data(data.as_slice())
                .bar_width(10)
                .bar_gap(3)
                .bar_style(Style::default().fg(Color::Yellow))
                .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
            f.render_widget(barchart, rect_sub_right[1]);
        }
        None => {
            let message = "Aucune ville de renseignée ou de recherchée";
            let block_no_place = Paragraph::new(message.as_ref()).style(Style::default());
            f.render_widget(block_no_place, frame)
        }
    }
}

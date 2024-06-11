use serde::Deserialize;
use sqlx::FromRow;
use egui_notify::Toasts;

#[derive(Clone, FromRow, Default, Debug, Deserialize)]
pub struct Comic {
    pub id_comic: i32,
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub image: String,
    pub price: f32,
    pub quantity: i32,
    pub volume: i32,
    pub active: bool,
    pub external_link: String
} 

#[derive(Clone, Default, Debug)]
pub enum DetailType {
    #[default]
    Detail,
    New,
    Modify,
    Carico,
    Scarico
}

#[derive(Clone, Default, Debug)]
pub struct DetailComic {
    pub comic: Comic,
    pub mag_mov_quantity: i32,
    pub note: String,
    pub detail_type: DetailType
}

#[derive(Clone)]
pub enum Theme {
    Light,
    Dark
}

#[derive(Clone)]
pub struct Settings {
    pub font_size: f32,
    pub theme: Theme
}

#[derive(Clone)]
pub enum Modal<T> {
    Opened(T),
    Closed(T)
}

pub struct MyApp {
    pub search: Comic,
    pub online_search: bool,
    pub online_search_results: Option<Vec<Comic>>,
    pub comics: Vec<Comic>,
    pub detail_opened: Option<DetailComic>,
    pub toasts: Toasts,
    pub settings: Modal<Settings>
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            search: Comic::default(),
            online_search: false,
            online_search_results: None,
            comics: Vec::new(),
            detail_opened: None,
            toasts: Toasts::default(),
            settings: Modal::Closed(Settings { font_size: 1., theme: Theme::Dark })
        }
    }
}

use sqlx::FromRow;
#[derive(Clone, FromRow, Default, Debug)]
pub struct Comic {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub image: String,
    pub price: f32,
    pub quantity: i32
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
    pub str_price: String,
    pub str_quantity: String,
    pub str_sc_quantity: String,
    pub note: String,
    pub quantity: i32,
    pub detail_type: DetailType
}

pub struct MyApp {
    pub search: Comic,
    pub online_search: bool,
    pub online_search_results: Option<Vec<Comic>>,
    pub comics: Vec<Comic>,
    pub detail_opened: Option<DetailComic>
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            search: Comic::default(),
            online_search: false,
            online_search_results: None,
            comics: Vec::new(),
            detail_opened: None
        }
    }
}

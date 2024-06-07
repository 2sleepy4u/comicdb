use crate::types::*;
use sqlx::Connection;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqliteConnection;


pub fn insert_comic(comic: &Comic) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut pool = SqliteConnection::connect(comicdb::SQLITE_CONNECTION_STRING)
            .await
            .unwrap();
        sqlx::query("
            INSERT INTO Comics
            (isbn, title, author, genre, price, image, volume, active) VALUES
            (?, ?, ?, ?, ?, ?, ?, ?)
         ")
            .bind(&comic.isbn)
            .bind(&comic.title)
            .bind(&comic.author)
            .bind(&comic.genre)
            .bind(&comic.price)
            .bind(&comic.image)
            .bind(&comic.volume)
            .execute(&mut pool)
            .await
    })
}

pub fn update_comic(comic: &Comic) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut pool = SqliteConnection::connect(comicdb::SQLITE_CONNECTION_STRING)
            .await
            .unwrap();
        let title = if comic.title.is_empty() { None } else { Some(&comic.title)}; 
        let author = if comic.author.is_empty() { None } else { Some(&comic.author)}; 
        let genre = if comic.genre.is_empty() { None } else { Some(&comic.genre)}; 
        sqlx::query("
            UPDATE Comics
            SET
                title = IFNULL(?, title),
                author = IFNULL(?, author),
                genre = IFNULL(?, genre),
                price = IFNULL(?, price),
                volume = IFNULL(?, volume),
                image = IFNULL(?, image)
                active = IFNULL(?, active)
            WHERE
                isbn = ?
         ")
            .bind(&title)
            .bind(&author)
            .bind(&genre)
            .bind(&comic.price)
            .bind(&comic.volume)
            .bind(&comic.image)
            .bind(&comic.active)
            .bind(&comic.isbn)
            .execute(&mut pool)
            .await
    })
}

pub async fn create_db() {
    sqlx::Sqlite::create_database(comicdb::SQLITE_CONNECTION_STRING).await.unwrap();
    let mut pool = SqliteConnection::connect(comicdb::SQLITE_CONNECTION_STRING)
        .await
        .unwrap();

    sqlx::query(include_str!("./../db/schema.sql"))
        .execute(&mut pool)
        .await
        .unwrap();
}

pub fn db_search(comic: &Comic) -> Vec<Comic> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        if !sqlx::Sqlite::database_exists(comicdb::SQLITE_CONNECTION_STRING).await.unwrap() {
            create_db().await;
        }

        let mut pool = SqliteConnection::connect(comicdb::SQLITE_CONNECTION_STRING)
            .await
            .unwrap();


        let isbn = if comic.isbn.is_empty() { None } else { Some(&comic.isbn)}; 
        let row: Vec<Comic> = sqlx::query_as(include_str!("./../db/comic_list.sql"))
            .bind(isbn)
            .bind(&comic.title)
            .bind(&comic.genre)
            .bind(&comic.author)
            .fetch_all(&mut pool)
            .await
            .unwrap();
        row
    })
}


pub fn google_search(comic: &Comic) -> Option<Vec<Comic>> {
    let mut url = "https://www.googleapis.com/books/v1/volumes?q=".to_string();
    if !comic.isbn.is_empty() {
        url = format!("{url}isbn:{}", comic.isbn);
    }
    if !comic.title.is_empty() {
        url = format!("{url}intitle:{}", comic.title);
    }
    if !comic.author.is_empty() {
        url = format!("{url}inauthor:{}", comic.author);
    }
    
    url = format!("{url}&langRestrict=it&maxResults=40");
    let response = reqwest::blocking::get(url)
        .unwrap()
        .json::<serde_json::Value>()
        .unwrap();

    let total_items = response["totalItems"]
        .as_u64()
        .unwrap_or(0);
    //40 max items per page
    let total_pages = total_items / 40;

    let mut items = response["items"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        //.filter(|x| x["saleInfo"]["country"].as_str().unwrap_or("") == "IT")
        //.filter(|x| x["volumeInfo"]["language"].as_str().unwrap_or("") == "it")
        .fold(Vec::new(), |mut acc, x| {
            let isbn: String = x["volumeInfo"]["industryIdentifiers"]
                .as_array()
                .unwrap_or(&Vec::new())
                .iter()
                .filter(|x| x["type"] == "ISBN_13")
                .map(|x| x["identifier"].as_str().unwrap())
                .collect();
            let title = x["volumeInfo"]["title"]
                .as_str()
                .unwrap_or("Non disponibile")
                .to_string();
            let image = x["volumeInfo"]["imageLinks"]["thumbnail"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let author = x["volumeInfo"]["authors"][0]
                .as_str()
                .unwrap_or("Non disponibile")
                .to_string();

            acc.push(Comic {
                title,
                image,
                author,
                isbn,
                ..Default::default()
            });
            acc
        });


    for i in 1..total_pages {
        let mut url = "https://www.googleapis.com/books/v1/volumes?q=".to_string();
        if !comic.isbn.is_empty() {
            url = format!("{url}isbn:{}", comic.isbn);
        }
        if !comic.title.is_empty() {
            url = format!("{url}intitle:{}", comic.title);
        }
        if !comic.author.is_empty() {
            url = format!("{url}inauthor:{}", comic.author);
        }

        let startIndex = i * 40;
        url = format!("{url}&langRestrict=it&maxResults=40&startIndex={startIndex}");
        let response = reqwest::blocking::get(url)
            .unwrap()
            .json::<serde_json::Value>()
            .unwrap();
        let mut result = response["items"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|x| x["saleInfo"]["country"].as_str().unwrap_or("") == "IT")
            .filter(|x| x["volumeInfo"]["language"].as_str().unwrap_or("") == "it")
            .fold(Vec::new(), |mut acc, x| {
                let isbn: String = x["volumeInfo"]["industryIdentifiers"]
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .filter(|x| x["type"] == "ISBN_13")
                    .map(|x| x["identifier"].as_str().unwrap())
                    .collect();
                let title = x["volumeInfo"]["title"]
                    .as_str()
                    .unwrap_or("Non disponibile")
                    .to_string();
                let image = x["volumeInfo"]["imageLinks"]["thumbnail"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                let author = x["volumeInfo"]["authors"][0]
                    .as_str()
                    .unwrap_or("Non disponibile")
                    .to_string();

                acc.push(Comic {
                    title,
                    image,
                    author,
                    isbn,
                    ..Default::default()
                });
                acc
            });
        items.append(&mut result);
    }
    Some(items)
}


pub fn carica_comic(comic: &Comic, quantity: i32, note: Option<String>) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut pool = SqliteConnection::connect(comicdb::SQLITE_CONNECTION_STRING)
            .await
            .unwrap();
        sqlx::query("
                    INSERT INTO MagMov
                    (isbn, quantity_s, quantity_c, mov_date, note) VALUES
                    (?, 0, ?, DATE('now'), ?)
                    ")
            .bind(&comic.isbn)
            .bind(&quantity)
            .bind(note.unwrap_or("".to_string()))
            .execute(&mut pool)
            .await
    })

}
pub fn scarica_comic(comic: &Comic, quantity: i32, note: Option<String>) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut pool = SqliteConnection::connect(comicdb::SQLITE_CONNECTION_STRING)
            .await
            .unwrap();
        sqlx::query("
                    INSERT INTO MagMov
                    (isbn, quantity_s, quantity_c, mov_date) VALUES
                    (?, ?, 0, DATE('now'))
                    ")
            .bind(&comic.isbn)
            .bind(&quantity)
            .bind(note.unwrap_or("".to_string()))
            .execute(&mut pool)
            .await
    })
}

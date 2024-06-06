use sqlx::MySqlPool;
use crate::types::*;

pub fn insert_comic(comic: &Comic) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let pool = MySqlPool::connect(comicdb::DB_CONNECTION_STRING)
            .await
            .unwrap();
        sqlx::query("
            INSERT INTO Comics
            (isbn, title, author, genre, price, image) VALUES
            (?, ?, ?, ?, ?, ?)
         ")
            .bind(&comic.isbn)
            .bind(&comic.title)
            .bind(&comic.author)
            .bind(&comic.genre)
            .bind(&comic.price)
            .bind(&comic.image)
            .execute(&pool)
            .await
            .unwrap();
    });
}

pub fn update_comic(comic: &Comic) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let pool = MySqlPool::connect(comicdb::DB_CONNECTION_STRING)
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
                price = IFNULL(?, price)
            WHERE
                isbn = ?
         ")
            .bind(&title)
            .bind(&author)
            .bind(&genre)
            .bind(&comic.price)
            .bind(&comic.isbn)
            .execute(&pool)
            .await
            .unwrap();
    });
}

pub fn db_search(comic: &Comic) -> Vec<Comic> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let pool = MySqlPool::connect(comicdb::DB_CONNECTION_STRING)
            .await
            .unwrap();
        let isbn = if comic.isbn.is_empty() { None } else { Some(&comic.isbn)}; 
        let row: Vec<Comic> = sqlx::query_as("
                SELECT 
                     C.isbn,
                     C.title,
                     C.author,
                     CAST(SUM(IFNULL(MM.quantity_c, 0) - IFNULL(MM.quantity_s, 0)) AS INT) as quantity,
                     C.image,
                     C.price,
                     C.genre
                 FROM
                    Comics C LEFT JOIN
                    MagMov MM ON C.isbn = MM.isbn 
                WHERE
                    C.isbn = IFNULL(?, C.isbn)
                AND C.title LIKE CONCAT('%', ?, '%')
                AND C.genre LIKE CONCAT('%', ?, '%')
                AND C.author LIKE CONCAT('%', ?, '%')
                GROUP By
                    C.isbn, C.title, C.author, C.image, C.price, C.genre
                 ")
            .bind(isbn)
            .bind(&comic.title)
            .bind(&comic.genre)
            .bind(&comic.author)
            .fetch_all(&pool)
            .await
            //.unwrap();
            .unwrap_or(Vec::new());
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
                quantity: 0,
                price: 0.,
                isbn,
                genre: "".to_string()
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
                    quantity: 0,
                    price: 0.,
                    isbn,
                    genre: "".to_string()
                });
                acc
            });
        items.append(&mut result);
    }
    Some(items)
}


pub fn carica_comic(comic: &Comic, quantity: i32, note: Option<String>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let pool = MySqlPool::connect(comicdb::DB_CONNECTION_STRING)
            .await
            .unwrap();
        sqlx::query("
                    INSERT INTO MagMov
                    (isbn, quantity_s, quantity_c, mov_date, note) VALUES
                    (?, 0, ?, NOW(), ?)
                    ")
            .bind(&comic.isbn)
            .bind(&quantity)
            .bind(note.unwrap_or("".to_string()))
            .execute(&pool)
            .await
            .unwrap();
    });

}
pub fn scarica_comic(comic: &Comic, quantity: i32, note: Option<String>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let pool = MySqlPool::connect(comicdb::DB_CONNECTION_STRING)
            .await
            .unwrap();
        sqlx::query("
                    INSERT INTO MagMov
                    (isbn, quantity_s, quantity_c, mov_date) VALUES
                    (?, ?, 0, NOW())
                    ")
            .bind(&comic.isbn)
            .bind(&quantity)
            .bind(note.unwrap_or("".to_string()))
            .execute(&pool)
            .await
            .unwrap();
    });
}

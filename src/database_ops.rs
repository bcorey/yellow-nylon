
use sqlx::{Column, Sqlite};
use sqlx::sqlite::{SqlitePoolOptions, SqliteArguments};
use sqlx::query::Query;
use sqlx::sqlite::SqliteRow;
use sqlx::Error;

pub async fn query_generic<'a>(path: String, query: Query<'a, Sqlite, SqliteArguments<'a>>) -> Result<u64, Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&path).await?;

    let rows_affected = query
        .execute(&pool)
        .await
        .unwrap();

    pool.close().await;

    Ok(rows_affected.rows_affected())
}

#[derive(Debug, Clone)]
pub struct ContentRow {
    pub content_entry_id: String,
    pub title: String,
    pub tagline: String,
    pub tags: String,
    pub content: String,
    pub is_pinned: bool,
    pub date: String, // date
    pub content_type: String, // enum
}
use crate::components::ContentForm;
impl ContentRow {
    pub fn new(content_form: ContentForm) -> Self {
        ContentRow {
            content_entry_id: content_form.content_entry_id,
            title: content_form.title,
            tagline: content_form.tagline,
            tags: content_form.tags,
            content: content_form.content,
            is_pinned: content_form.is_pinned,
            date: content_form.date,
            content_type: content_form.content_type
        }
    }
}

pub async fn content_table_entry(database_path: String, row: ContentRow) -> Result<u64, Error> {
    let query: Query<Sqlite, SqliteArguments> = sqlx::query!(
        r#"
        INSERT INTO content_table (content_entry_id, name, tagline, tags, content, is_pinned, content_type, date)
        VALUES(?, ?, ?, ?, ?, ?, ?, ?)
        "#, 
        row.content_entry_id, 
        row.title, 
        row.tagline, 
        row.tags, 
        row.content, 
        row.is_pinned, 
        row.content_type,
        row.date,
    );

    let result = query_generic(database_path, query).await;
    result
}

#[derive(Debug, Clone)]
pub struct ImageRow {
    pub image_name: String,
    pub image_caption: String,
    pub is_content_thumbnail: bool,
    pub is_pinned: bool,
    pub content_entry_id: String,
    pub image_original: Vec<u8>,
    pub image_web: Vec<u8>,
    pub image_thumbnail: Vec<u8>,
}
use crate::components::ImageForm;
impl ImageRow {
    pub fn new(image_form: ImageForm) -> Self {
        let path = image_form.image_path.unwrap().clone();
        let image_bytes = std::fs::read(&path).expect("Could not read image");
        ImageRow {
            image_name: path,
            image_caption: image_form.image_caption.unwrap().clone(),
            is_content_thumbnail: image_form.is_content_thumbnail,
            is_pinned: image_form.is_pinned,
            content_entry_id: image_form.content_entry_id.unwrap().clone(),
            image_original: image_bytes.clone(),
            image_web: image_bytes.clone(),
            image_thumbnail: image_bytes.clone(),
        }
    }
}


pub async fn image_table_entry(database_path: String, row: ImageRow) -> Result<u64, Error> {
    let query: Query<Sqlite, SqliteArguments> = sqlx::query!(
        r#"
        INSERT INTO image_table (image_name, image_caption, content_entry_id, is_pinned, image_original, image_web, image_thumbnail)
        VALUES(?, ?, ?, ?, ?, ?, ?)
        "#, 
        row.image_name, 
        row.image_caption, 
        row.content_entry_id, 
        row.is_pinned, 
        row.image_original, 
        row.image_web, 
        row.image_thumbnail
    );

    let result = query_generic(database_path, query).await;
    result
}
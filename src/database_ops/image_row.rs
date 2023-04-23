
use sqlx::Sqlite;
use sqlx::sqlite::{SqlitePoolOptions, SqliteArguments};
use sqlx::query::Query;
use sqlx::Error;

use super::generic_ops::query_generic;

#[derive(Debug, Clone, sqlx::FromRow, PartialEq)]
pub struct ImageRow {
    pub image_name: Option<String>,
    pub image_caption: Option<String>,
    //pub is_content_thumbnail: Option<bool>,
    pub is_pinned: Option<i64>,
    pub content_entry_id: Option<String>,
    pub image_original: Option<Vec<u8>>,
    pub image_web: Option<Vec<u8>>,
    pub image_thumbnail: Option<Vec<u8>>,
}
use crate::components::ImageForm;
impl ImageRow {
    pub fn new(image_form: ImageForm) -> Self {
        let path = image_form.image_path.clone();
        let image_bytes = std::fs::read(&path).expect("Could not read image");

        let name = path.split(r#"\"#).last().unwrap().to_string();
        ImageRow {
            image_name: Some(name),
            image_caption: Some(image_form.image_caption.clone()),
            //is_content_thumbnail: Some(image_form.is_content_thumbnail),
            is_pinned: Some(image_form.is_pinned as i64),
            content_entry_id: Some(image_form.content_entry_id.clone()),
            image_original: Some(image_bytes.clone()),
            image_web: Some(image_bytes.clone()),
            image_thumbnail: Some(image_bytes.clone()),
        }
    }

    pub async fn delete_image_row(&self, database_path: String) -> Result<u64, Error> {
        let query: Query<Sqlite, SqliteArguments> = sqlx::query!(
            r#"
            DELETE FROM image_table WHERE content_entry_id=? AND image_name=?

            "#, 
            self.content_entry_id,
            self.image_name,
        );
        query_generic(database_path, query).await
    }

    pub async fn add_image_row(&self, database_path: String) -> Result<u64, Error> {
        let query: Query<Sqlite, SqliteArguments> = sqlx::query!(
            r#"
            INSERT INTO image_table (image_name, image_caption, content_entry_id, is_pinned, image_original, image_web, image_thumbnail)
            VALUES(?, ?, ?, ?, ?, ?, ?)
            "#, 
            self.image_name, 
            self.image_caption, 
            self.content_entry_id, 
            self.is_pinned, 
            self.image_original, 
            self.image_web, 
            self.image_thumbnail
        );
        query_generic(database_path, query).await
    }

    pub async fn get_content_images(database_path: String, content_entry_id: String) -> Vec<ImageRow> {
        let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_path).await.unwrap();

        let query = sqlx::query_as!(
            ImageRow,
            r#"
            SELECT * FROM image_table WHERE content_entry_id=?
            "#,
            content_entry_id
        );

        let rows = query
            .fetch_all(&pool)
            .await
            .unwrap();

        pool.close().await;
        rows
    }

    pub async fn all_rows(database_path: String) -> Vec<ImageRow> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_path).await.unwrap();
    
        let query = sqlx::query_as!(
            ImageRow,
            r#"
            SELECT * FROM image_table
            "#
        );
    
        let rows = query
            .fetch_all(&pool)
            .await
            .unwrap();
    
        pool.close().await;
        rows
    }

}

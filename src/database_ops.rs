
use sqlx::Sqlite;
use sqlx::sqlite::{SqlitePoolOptions, SqliteArguments};
use sqlx::query::Query;
use sqlx::Error;


pub async fn all_rows(database_path: String) -> Vec<ContentRow> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_path).await.unwrap();

    let query = sqlx::query_as!(
        ContentRow,
        r#"
        SELECT * FROM content_table
        "#
    );

    let rows = query
        .fetch_all(&pool)
        .await
        .unwrap();

    pool.close().await;
    rows
}



pub async fn search_rows(database_path: String, search_term: String) -> Vec<ContentRow> {

    match search_term == "*".to_string() || search_term.len() == 0 {
        true => all_rows(database_path.clone()).await,
        false => {
            let search_term: String = format!("%{}%", search_term);
            let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_path).await.unwrap();
            let query = sqlx::query_as!(
                ContentRow,
                r#"
                SELECT * FROM content_table WHERE title LIKE ? OR tagline LIKE ? OR tags LIKE ? OR content LIKE ? OR date LIKE ?
                "#, 
                search_term,
                search_term,
                search_term,
                search_term,
                search_term
            );
        
            let rows = query
                .fetch_all(&pool)
                .await
                .unwrap();
        
            pool.close().await;
            rows
        }
    }
    
}

async fn query_generic<'a>(path: String, query: Query<'a, Sqlite, SqliteArguments<'a>>) -> Result<u64, Error> {
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

#[derive(Debug, Clone, sqlx::FromRow, PartialEq)]
pub struct ContentRow {
    pub content_entry_id: Option<String>,
    pub title: Option<String>,
    pub tagline: Option<String>,
    pub tags: Option<String>,
    pub content: Option<String>,
    pub is_pinned: Option<i64>,
    pub date: Option<String>, // date
    pub content_type: Option<String>, // enum
}
use crate::components::ContentForm;
use crate::components::form_utils::FormMode;
// this is redundant but lays the foundation for form data vs row data separation
impl ContentRow {
    pub fn new(content_form: ContentForm) -> Self {
        ContentRow {
            content_entry_id: Some(content_form.content_entry_id),
            title: Some(content_form.title),
            tagline: Some(content_form.tagline),
            tags: Some(content_form.tags),
            content: Some(content_form.content),
            is_pinned: Some(content_form.is_pinned as i64),
            date: Some(content_form.date),
            content_type: Some(content_form.content_type)
        }
    }

    /// deletes content row and associated images
    pub async fn delete_content_row(database_path: String, content_entry_id: String) -> Result<u64, Error>{
        let query: Query<Sqlite, SqliteArguments> = sqlx::query!(
            r#"
            DELETE FROM content_table WHERE content_entry_id=?
            "#, 
            content_entry_id
        );
        let images = ImageRow::get_content_images(database_path.clone(), content_entry_id.clone()).await;
        for image in images {
            image.delete_image_row(database_path.clone()).await.expect("Couldn't delete image");
        }
        query_generic(database_path, query).await
    }

    pub async fn enter_content_row(&mut self, database_path: String, form_mode: FormMode) -> Result<u64, Error> {
        if form_mode == FormMode::Create {
            self.next_entry_id(database_path.clone()).await;
            self.content_table_entry(database_path.clone()).await
        } else {
            self.content_table_update(database_path.clone()).await
        }
        
    }

    async fn content_table_update(&self, database_path: String) -> Result<u64, Error> {
        let query: Query<Sqlite, SqliteArguments> = sqlx::query!(
            r#"
            UPDATE content_table 
            SET title = ?, 
                tagline = ?, 
                tags = ?, 
                content = ?, 
                is_pinned = ?, 
                content_type = ?, 
                date = ?
            WHERE
                content_entry_id = ?
            "#, 
            self.title, 
            self.tagline, 
            self.tags, 
            self.content, 
            self.is_pinned, 
            self.content_type,
            self.date,
            self.content_entry_id
        );
    
        query_generic(database_path, query).await
    }

    async fn content_table_entry(&self, database_path: String) -> Result<u64, Error> {
        let query: Query<Sqlite, SqliteArguments> = sqlx::query!(
            r#"
            INSERT INTO content_table (content_entry_id, title, tagline, tags, content, is_pinned, content_type, date)
            VALUES(?, ?, ?, ?, ?, ?, ?, ?)
            "#, 
            self.content_entry_id, 
            self.title, 
            self.tagline, 
            self.tags, 
            self.content, 
            self.is_pinned, 
            self.content_type,
            self.date,
        );
    
        query_generic(database_path, query).await
    }

    async fn next_entry_id(&mut self, database_path: String) {
        let search_term: String = format!("{}-%", self.date.as_ref().unwrap());
        let split_term: String = format!("{}-", self.date.as_ref().unwrap());
    
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_path).await.unwrap();
    
        let query = sqlx::query_as!(
            ContentRow,
            r#"
            SELECT * FROM content_table WHERE content_entry_id LIKE ?
            "#,
            search_term
        );
    
        let content_entry_ids = query
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.content_entry_id.unwrap());
    
        pool.close().await;
    
        let existing_id_endings = content_entry_ids
            .map(|entry_id| 
                entry_id
                    .split(&split_term)
                    .collect::<String>()
                    .parse::<i64>()
                    .expect("couldn't parse entry ID")
            )
            .collect::<Vec<i64>>();
        let id_suffix = match existing_id_endings.iter().max() {
            Some(max) => *max + 1,
            None => 1,
        };
    
        self.content_entry_id = Some(format!("{}-{}", self.date.as_ref().unwrap(), id_suffix));
    }
}



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
}


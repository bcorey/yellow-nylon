use crate::database_ops::ContentRow;

#[derive(Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ContentType {
    Post,
    Project,
    Note
}

use std::fmt;
impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentType::Post => write!(f, "Post"),
            ContentType::Project => write!(f, "Project"),
            ContentType::Note => write!(f, "Note"),
        }
    }
}
use std::str::FromStr;
impl FromStr for ContentType {
    type Err = ();

    fn from_str(input: &str) -> Result<ContentType, Self::Err> {
        match input {
            "Post"  => Ok(ContentType::Post),
            "Note"  => Ok(ContentType::Note),
            "Project"  => Ok(ContentType::Project),
            _      => Err(()),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct PostMetadata {
    pub content_entry_id: String,
    pub title: String,
    pub tagline: String,
    pub date: String,
    pub tags: Vec<String>,
    pub thumbnails: Vec<String>,
    pub is_pinned: bool,
    pub content_type: ContentType,
}

impl PostMetadata {
    pub fn new(row: ContentRow) -> Self  {
        PostMetadata {
            content_entry_id: row.content_entry_id.unwrap(),
            title: row.title.unwrap(),
            tagline: row.tagline.unwrap(),
            date: row.date.unwrap(),
            tags: PostMetadata::tags_from_string(row.tags.unwrap()),
            thumbnails: Vec::new(),
            is_pinned: row.is_pinned.unwrap() != 0,
            content_type: ContentType::from_str(&row.content_type.unwrap()).unwrap(),
        }
    }

    pub fn add_thumbnail(&mut self, thumbnail: String) {
        self.thumbnails.push(thumbnail);
    }

    fn tags_from_string(tags: String) -> Vec<String> {
        tags
            .split(r#","#)
            .map(|item| 
                item.to_string()
                .trim()
                .to_string()
            )
            .filter(|item| item.len() > 0)
            .collect::<Vec<String>>()
    }

    
}
use crate::database_ops::ImageRow;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Highlight {
    image_name: String,
    image_caption: String,
    content_entry_id: String,
}

impl Highlight {
    pub fn new(image: ImageRow) -> Self {
        Highlight {
            image_name: image.image_name.unwrap(),
            image_caption: image.image_caption.unwrap(),
            content_entry_id: image.content_entry_id.unwrap(),
        }
    }
}
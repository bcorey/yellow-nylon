use super::post_metadata::*;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ContentManifest {
    bio: Vec<PostMetadata>,
    all: Vec<PostMetadata>,
}

impl ContentManifest {
    pub fn new() -> Self {
        ContentManifest {
            bio: Vec::new(),
            all: Vec::new(),
        }
    }

    pub fn add(&mut self, content: PostMetadata) {
        /*if self.bio.len() == 0 && content.tags.contains(&"bio".to_string()) {
            self.bio.push(content);
        } else {
            self.all.push(content);
        }*/
        self.all.push(content);

    }
}
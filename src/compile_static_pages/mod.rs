// create vec<> highlights -> highlights.yaml
//
// get all images
// for each image_row in image_table:
// if image_row.is_pinned {
//    add image_row.image_name & image_row.image_caption & image_row.content_entry_id to highlights.yaml
//}
// 
// create Manifest (list of all metadata, for now)
// get all ContentRow from content_table
// for each content_row in content_table 
//      PostMetadata::new()
//          parse content_row.tags to Vec<String>
//      content_row.content TO html
//      prepend server prefix to html img tags
//      
//      create folder in posts/content_row.content_entry_id/
//      save content.html
//      save metadata
//      SELECT image_web, image_thumbnail FROM image_table WHERE content_entry_id=content_row.content_entry_id
//          create img files with name & bytes
//      
//      append metadata to manifest
//
// clean up
// display nonconformant entries 


mod post_metadata;
use post_metadata::*;
mod content_manifest;
use content_manifest::*;
mod content_body;
use content_body::*;
mod highlights_manifest;
use highlights_manifest::*;
mod path_manager;
use path_manager::PathManager;

use crate::database_ops::{ ContentRow, ImageRow };
use std::fs;

async fn compile_highlights(database_path: String, dist_path: String) {
    let mut highlights: Vec<Highlight> = Vec::new();

    let image_rows = ImageRow::all_rows(database_path).await;

    for image_row in image_rows {
        if image_row.is_pinned.unwrap() != 0 {
            highlights.push(Highlight::new(image_row));
        }
    }

    let highlights_write = serde_yaml::to_string::<Vec<Highlight>>(&highlights).unwrap();
    fs::write(format!("{}/highlights_manifest.yaml", dist_path), highlights_write).expect("Could not write manifest");
}


pub async fn compile_content(database_path: String, dist_path: String) {

    compile_highlights(database_path.clone(), dist_path.clone()).await;

    let mut manifest = ContentManifest::new();

    let entries = ContentRow::all_rows(database_path.clone()).await;

    //clears dist/posts/ before proceeding
    fs::remove_dir_all(format!("{}/posts", dist_path.clone())).unwrap();
    fs::DirBuilder::new().create(format!("{}/posts", dist_path.clone())).unwrap();

    for entry in entries {
        let path_manager = PathManager::new(entry.content_entry_id.clone().unwrap(), dist_path.clone());

        fs::DirBuilder::new()
            .recursive(true)
            .create(path_manager.content_dir())
            .unwrap();

        build_html(entry.content.clone().unwrap(), path_manager.clone());

        let content_images = 
            ImageRow::get_content_images(database_path.clone(), entry.clone().content_entry_id.unwrap())
            .await;

        let mut meta = PostMetadata::new(entry);

        for image in content_images {
            let image_name = image.image_name.unwrap();
            meta.add_thumbnail(image_name.clone());
            fs::write(path_manager.make_image_path(image_name), &image.image_web.unwrap()).expect("Could not write image");
        }

        let meta_write = serde_yaml::to_string::<PostMetadata>(&meta).unwrap();
        fs::write(path_manager.yaml_path(), meta_write).expect("Could not write meta");


        manifest.add(meta);
    }


    let manifest_write = serde_yaml::to_string::<ContentManifest>(&manifest).unwrap();
    fs::write(format!("{}/manifest.yaml", dist_path), manifest_write).expect("Could not write manifest");
}
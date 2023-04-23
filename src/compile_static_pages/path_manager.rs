#[derive(Clone, Debug)]
pub struct PathManager {
    content_dir: String,
    html_path: String,
    content_dir_server: String,
    yaml_path: String,
}

static CONTENT_SERVER: &str = "https://orca-app-8uzme.ondigitalocean.app/reflective-panda";

impl PathManager {
    pub fn new(dir_name: String, dist_path: String) -> PathManager {
        let content_dir = format!("{}/posts/{}/", dist_path, dir_name);
        PathManager {
            content_dir: content_dir.clone(),
            html_path: format!("{}/content.html", content_dir),
            content_dir_server: format!("{}/posts/{}/", CONTENT_SERVER, dir_name),
            yaml_path: format!("{}/meta.yaml", content_dir),
        }
    }

    pub fn content_dir(&self) -> String {
        self.content_dir.clone()
    }

    pub fn html_path(&self) -> String {
        self.html_path.clone()
    }

    pub fn yaml_path(&self) -> String {
        self.yaml_path.clone()
    }

    pub fn content_dir_server(&self) -> String {
        self.content_dir_server.clone()
    }

    pub fn make_image_path(&self, img_name: String) -> String {
        format!("{}{}", self.content_dir, img_name)
    }
}
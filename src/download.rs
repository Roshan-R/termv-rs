use std::fs;
use std::path::Path;

use whoami::username;

use ureq;

pub struct Downloader {
    dir_path: String,
    json_path: String,
}

impl Downloader {
    pub fn new() -> Self {
        let username = username().to_string();
        let dir_path = format!("/home/{}/.cache/termv-rs/", username.as_str());
        let file_name = "d.json";
        let json_path = format!("{}/{}", dir_path, file_name);
        Self {
            dir_path,
            json_path,
        }
    }

    pub fn check_file_exists(&self) -> bool {
        Path::new(self.json_path.as_str()).exists()
    }

    pub fn download(&self) {
        let d = Path::new(self.dir_path.as_str());
        fs::create_dir_all(d).unwrap();

        println!("Downloading json file...");

        let resp = ureq::get("https://iptv-org.github.io/iptv/channels.json")
            .call()
            .unwrap();

        let body = resp.into_string().unwrap();

        println!("Done!");

        fs::write(self.json_path.as_str(), body).expect("Unable to write file");
    }
}

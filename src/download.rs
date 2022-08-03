use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

use whoami::username;

use ureq;

pub struct Downloader {
    dir_path: String,
    json_path: String,
    etag_path: String,
}

impl Downloader {
    pub fn new() -> Self {
        let username = username().to_string();
        let dir_path = format!("/home/{}/.cache/termv-rs/", username.as_str());
        let json_path = format!("{}/{}", dir_path, "d.json");
        let etag_path = format!("{}/{}", dir_path, "etag");
        Self {
            dir_path,
            json_path,
            etag_path,
        }
    }

    pub fn check_file_exists(&self) -> bool {
        Path::new(self.json_path.as_str()).exists()
    }

    pub fn first_download(self) {
        let d = Path::new(self.dir_path.as_str());

        fs::create_dir_all(d).unwrap();

        println!("Downloading json file...");

        let resp = ureq::get("https://iptv-org.github.io/iptv/channels.json")
            .set("Accept-Encoding", "gzip")
            .call()
            .unwrap();

        let etag = resp.header("etag").unwrap();
        fs::write(self.etag_path.as_str(), etag).expect("Unable to write file");

        let body = resp.into_string().unwrap();
        fs::write(self.json_path.as_str(), body).expect("Unable to write file");

        println!("Done!");
    }

    fn should_update(&self) -> bool {
        let metadata = fs::metadata(self.etag_path.as_str()).unwrap();

        let last_modified = metadata.modified().unwrap();
        let now = SystemTime::now();

        let one_day = Duration::new(86400, 0);

        let difference = now
            .duration_since(last_modified)
            .expect("Error while calculating difference in time");

        difference > one_day
    }

    pub fn update_if_changed(&self) {
        if !self.should_update() {
            return;
        };
        println!("Checking for updates..");
        let resp = ureq::get("https://iptv-org.github.io/iptv/channels.json")
            .set("Accept-Encoding", "gzip")
            .call()
            .unwrap();

        let r_etag = resp.header("etag").unwrap();
        let old_etag = fs::read_to_string(self.etag_path.as_str()).expect("Unable to read file");

        if r_etag != old_etag {
            println!("Updating json..");
            fs::write(self.etag_path.as_str(), r_etag).expect("Unable to write file");
            let body = resp.into_string().unwrap();
            fs::write(self.json_path.as_str(), body).expect("Unable to write file");
        } else {
            println!("No change detected");
        }
    }
}

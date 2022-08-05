use colored::Colorize;
use platform_dirs::AppDirs;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use std::{fs, io};

use ureq;

pub struct Downloader {
    cache_dir: PathBuf,
    pub json_path: PathBuf,
    etag_path: PathBuf,
}

impl Downloader {
    pub fn new() -> Self {
        let cache_dir = AppDirs::new(Some("termv-rs"), true).expect("Could not get cache dir").cache_dir;

        let mut etag_path = cache_dir.clone();
        etag_path.push("etag");

        let mut json_path = cache_dir.clone();
        json_path.push("d.json");

        Self {
            cache_dir,
            json_path,
            etag_path,
        }
    }

    pub fn check_file_exists(&self) -> bool {
        self.json_path.as_path().exists()
    }

    fn download(&self) {
        let resp = ureq::get("https://iptv-org.github.io/iptv/channels.json")
            .set("Accept-Encoding", "gzip")
            .call()
            .expect("Could not connect to the internet. Check if your net is working");

        let etag = resp.header("etag").unwrap();
        fs::write(self.etag_path.as_path(), etag).expect("Unable to write file");

        let body = resp.into_string().unwrap();
        fs::write(self.json_path.as_path(), body).expect("Unable to write file");
    }

    pub fn first_download(&self) {
        let d = self.cache_dir.as_path();
        println!("Downloading json file...        ");
        fs::create_dir_all(d).unwrap();
        self.download();
        println!("Done!");
    }

    fn should_update(&self) -> bool {
        let metadata = fs::metadata(self.etag_path.as_path()).unwrap();

        let last_modified = metadata.modified().unwrap();
        let now = SystemTime::now();

        let one_day = Duration::new(86400, 0);

        let difference = now
            .duration_since(last_modified)
            .expect("Error while calculating difference in time");

        difference > one_day
    }

    pub fn update(&self) {
        print!("Downloading https://iptv-org.github.io/iptv/channels.json...  ");
        io::stdout().flush().unwrap();
        self.download();
        println!("{}", "Done!".green());
    }

    pub fn update_if_changed(&self) {
        if self.should_update() {
            println!("Checking for updates..");
            self.update();
        };
    }
}

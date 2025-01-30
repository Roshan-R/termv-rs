use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use crate::types::channel::Channel;
use crate::types::stream::Stream;

use crate::Config;

pub struct Downloader {
    config: Config,
}

pub struct DownloadResponse {
    pub json: String,
    pub etag: String,
}

pub trait DownloadTrait: Sized {
    fn download(url: &str) -> DownloadResponse;
    fn save(config: &Config);
    fn load(config: &Config) -> Vec<Self>;
}

impl Downloader {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn file_exists(&self) -> bool {
        self.config.channels_json_path.as_path().exists()
            & self.config.streams_json_path.as_path().exists()
    }

    pub fn download(&self) {
        print!("Downloading {}...", self.config.streams_url);
        Stream::save(&self.config);
        println!("   Done!");

        print!("Downloading {}...", self.config.streams_url);
        Channel::save(&self.config);
        println!("   Done!");
        //self.process();
    }

    // pub fn process(&self) {
    //     let streams: Vec<Stream> = Stream::load(&self.config);
    //     let channels: Vec<Channel> = Channel::load(&self.config);
    //
    //     let mut map: HashMap<String, String> = HashMap::new();
    //     for s in streams {
    //         match s.id {
    //             Some(id) => {
    //                 map.insert(id, s.url.unwrap());
    //             }
    //             None => {}
    //         }
    //     }
    //
    //     let mut ch: Vec<Channel> = channels
    //         .into_iter()
    //         .filter(|x| map.contains_key(x.id.clone().unwrap()))
    //         .collect();
    //
    //     for mut c in &mut ch {
    //         match &c.id {
    //             Some(i) => {
    //                 c.stream = Stream {
    //                     id: Some(i.to_string()),
    //                     url: Some(map.get(i).unwrap().to_string()),
    //                 };
    //             }
    //             None => {}
    //         }
    //     }
    //     let s = serde_json::to_string(&ch).unwrap();
    //     fs::write(&self.json_path, s).expect("Could not write to json file");
    // }

    pub fn first_download(&self) {
        let d = self.config.cache_dir.as_path();
        println!("Downloading json file...        ");
        fs::create_dir_all(d).unwrap();
        self.download();
        println!("Done!");
    }

    fn should_update(&self, path: &PathBuf) -> bool {
        let metadata = fs::metadata(path.as_path()).unwrap();

        let last_modified = metadata.modified().unwrap();
        let now = SystemTime::now();

        let one_day = Duration::new(86400, 0);

        let difference = now
            .duration_since(last_modified)
            .expect("Error while calculating difference in time");

        difference > one_day
    }

    pub fn do_it(&self) {
        if !self.file_exists() {
            self.first_download();
        } else {
            self.update_if_changed();
        }
    }

    pub fn update_if_changed(&self) {
        if self.should_update(&self.config.channels_etag_path)
            && self.should_update(&self.config.streams_etag_path)
        {
            println!("Checking for updates..");
            self.download();
        };
    }
}

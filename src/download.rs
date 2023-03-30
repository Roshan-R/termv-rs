use platform_dirs::AppDirs;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use std::{fs, io};

use crate::types::{Channel, Stream};

use ureq;

pub struct ChannelD {
    pub json_path: PathBuf,
    etag_path: PathBuf,
    url: String,
}

pub struct StreamD {
    pub json_path: PathBuf,
    etag_path: PathBuf,
    url: String,
}

pub struct Downloader {
    pub channel_d: ChannelD,
    pub stream_d: StreamD,
    cache_dir: PathBuf,
    pub json_path: PathBuf,
}

impl Downloader {
    pub fn new(stream_url: String, channel_url: String) -> Self {
        let cache_dir = AppDirs::new(Some("termv-rs"), true)
            .expect("Could not get cache dir")
            .cache_dir;

        let stream_d = StreamD {
            etag_path: cache_dir.clone().join("stream_etag"),
            json_path: cache_dir.clone().join("streams.json"),
            url: stream_url,
        };

        let channel_d = ChannelD {
            etag_path: cache_dir.clone().join("channel_etag"),
            json_path: cache_dir.clone().join("channels.json"),
            url: channel_url,
        };

        Self {
            cache_dir: cache_dir.clone(),
            stream_d,
            channel_d,
            json_path: cache_dir.clone().join("data.json"),
        }
    }

    pub fn check_file_exists(&self) -> bool {
        self.json_path.as_path().exists()
    }

    pub fn download(&self) {
        print!("Downloading {}...", self.stream_d.url);
        io::stdout().flush().unwrap();
        let resp = ureq::get(self.stream_d.url.as_str())
            .set("Accept-Encoding", "gzip")
            .call()
            .expect("Could not connect to the internet. Check if your net is working");

        let etag = resp.header("etag").unwrap();
        fs::write(self.stream_d.etag_path.as_path(), etag).expect("Unable to write file");

        let mut body: String = "".to_string();
        let mut r = resp.into_reader();
        r.read_to_string(&mut body).unwrap();
        fs::write(self.stream_d.json_path.as_path(), body).expect("Unable to write file");

        println!("   Done!");

        print!("Downloading {}...", self.channel_d.url);
        io::stdout().flush().unwrap();
        let resp = ureq::get(self.channel_d.url.as_str())
            .set("Accept-Encoding", "gzip")
            .call()
            .expect("Could not connect to the internet. Check if your net is working");

        let etag = resp.header("etag").unwrap();
        fs::write(self.channel_d.etag_path.as_path(), etag).expect("Unable to write file");

        let mut body: String = "".to_string();
        let mut r = resp.into_reader();
        r.read_to_string(&mut body).unwrap();
        fs::write(self.channel_d.json_path.as_path(), body).expect("Unable to write file");
        println!("   Done!");

        self.process();
    }

    pub fn process(&self) {
        let streams_json = fs::read_to_string(self.stream_d.json_path.to_str().unwrap())
            .expect("Error reading data file");
        let streams: Vec<Stream> = serde_json::from_str(streams_json.as_str()).unwrap();

        let channels_json = fs::read_to_string(self.channel_d.json_path.to_str().unwrap())
            .expect("Error reading data file");
        let channels: Vec<Channel> = serde_json::from_str(channels_json.as_str()).unwrap();

        let mut map: HashMap<String, String> = HashMap::new();
        for s in streams {
            match s.id {
                Some(id) => {
                    map.insert(id, s.url.unwrap());
                }
                None => {}
            }
        }

        let mut ch: Vec<Channel> = channels
            .into_iter()
            .filter(|x| map.contains_key(&x.id.clone().unwrap()))
            .collect();

        for mut c in &mut ch {
            match &c.id {
                Some(i) => {
                    c.stream = Stream {
                        id: Some(i.to_owned()),
                        url: Some(map.get(i).unwrap().to_string()),
                    };
                }
                None => {}
            }
        }
        let s = serde_json::to_string(&ch).unwrap();
        fs::write(&self.json_path, s).expect("Could not write to json file");
    }

    pub fn first_download(&self) {
        let d = self.cache_dir.as_path();
        println!("Downloading json file...        ");
        fs::create_dir_all(d).unwrap();
        self.download();
        println!("Done!");
    }

    fn should_update(&self) -> bool {
        let metadata = fs::metadata(self.channel_d.etag_path.as_path()).unwrap();

        let last_modified = metadata.modified().unwrap();
        let now = SystemTime::now();

        let one_day = Duration::new(86400, 0);

        let difference = now
            .duration_since(last_modified)
            .expect("Error while calculating difference in time");

        difference > one_day
    }

    pub fn update(&self) {
        self.download();
    }

    pub fn update_if_changed(&self) {
        if self.should_update() {
            println!("Checking for updates..");
            self.update();
        };
    }
}

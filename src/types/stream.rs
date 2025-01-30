use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::Write;

use super::Config;

use crate::downloader::{DownloadResponse, DownloadTrait};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct Stream {
    #[serde(rename(serialize = "channel", deserialize = "channel"))]
    pub id: Option<String>,
    pub url: Option<String>,
}

impl DownloadTrait for Stream {
    fn download(url: &str) -> DownloadResponse {
        let resp = ureq::get(url)
            .set("Accept-Encoding", "gzip")
            .call()
            .expect("Could not connect to the internet. Check if your net is working");
        DownloadResponse {
            etag: resp.header("etag").unwrap().to_string(),
            json: resp.into_string().unwrap(),
        }
    }

    fn save(config: &Config) {
        let resp = Stream::download(config.streams_url.as_str());
        io::stdout().flush().unwrap();
        fs::write(config.streams_etag_path.as_path(), resp.etag).expect("Unable to write file");
        fs::write(config.streams_json_path.as_path(), resp.json).expect("Unable to write file");
    }

    fn load(config: &Config) -> Vec<Self> {
        let json = fs::read_to_string(config.streams_json_path.to_str().unwrap())
            .expect("Error reading data file");
        serde_json::from_str(json.as_str()).unwrap()
    }
}

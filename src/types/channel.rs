use std::fmt;
use std::fs;
use std::io;
use std::io::Write;

use super::Config;
use serde::{Deserialize, Serialize};
extern crate skim;

use crate::downloader::DownloadTrait;
use crate::{downloader::DownloadResponse, types::stream::Stream};
use skim::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct Channel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub country: Option<String>,
    pub languages: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub is_nsfw: Option<bool>,
    #[serde(default)]
    pub stream: Stream,
}

impl DownloadTrait for Channel {
    fn download(url: &str) -> DownloadResponse {
        let resp = ureq::get(url)
            .set("Accept-Encoding", "gzip")
            .call()
            .expect("Could not connect to the internet. Check if your net is working");
        DownloadResponse {
            etag: resp.header("etag").unwrap().to_string(),
            json: io::read_to_string(resp.into_reader()).unwrap(),
        }
    }

    fn save(config: &Config) {
        let resp = Channel::download(config.channels_url.as_str());
        io::stdout().flush().unwrap();
        fs::write(config.channels_etag_path.as_path(), resp.etag).expect("Unable to write file");
        fs::write(config.channels_json_path.as_path(), resp.json).expect("Unable to write file");
    }

    fn load(config: &Config) -> Vec<Self> {
        let json = fs::read_to_string(config.channels_json_path.to_str().unwrap())
            .expect("Error reading data file");
        serde_json::from_str(json.as_str()).unwrap()
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let null_string = "Null".to_string();
        let name: &String = self.name.as_ref().unwrap_or(&null_string);
        let category: &String = &self
            .categories
            .as_ref()
            .unwrap()
            .first()
            .unwrap_or(&null_string);
        let language: &String = &self
            .languages
            .as_ref()
            .unwrap()
            .first()
            .unwrap_or(&null_string);
        let country: &String = self.country.as_ref().unwrap_or(&null_string);
        write!(
            f,
            "{:<50}  |{:<15} |{:<10} |{:<10}\n",
            name, category, language, country
        )
    }
}

impl SkimItem for Channel {
    fn text(&self) -> Cow<str> {
        Cow::Owned(self.to_string().clone())
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.id.as_ref().unwrap())
    }
}

use platform_dirs::AppDirs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub channels_json_path: PathBuf,
    pub streams_json_path: PathBuf,
    pub channels_etag_path: PathBuf,
    pub streams_etag_path: PathBuf,
    pub channels_url: String,
    pub streams_url: String,
    pub cache_dir: PathBuf,
}

impl Config {
    pub fn new(channels_url: &str, streams_url: &str) -> Self {
        let app_dirs = AppDirs::new(None, true).unwrap();
        let cache_dir = app_dirs.cache_dir;
        return Self {
            channels_json_path: cache_dir.join("channels.json"),
            streams_json_path: cache_dir.join("streams.json"),
            channels_etag_path: cache_dir.join("channels_etag"),
            streams_etag_path: cache_dir.join("streams_etag"),
            channels_url: channels_url.to_string(),
            streams_url: streams_url.to_string(),
            cache_dir,
        };
    }
}

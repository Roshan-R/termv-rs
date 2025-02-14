use args::Args;
use clap::Parser;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;
use types::{Channel, Config, Stream};
use utils::open_mpv;

mod args;
mod downloader;
mod selector;
mod types;
mod utils;

use crate::downloader::{DownloadTrait, Downloader};

const CHANNELS_URL: &str = "https://iptv-org.github.io/api/channels.json";
const STREAMS_URL: &str = "https://iptv-org.github.io/api/streams.json";

fn main() {
    let args = Args::parse();
    let config = Config::new(CHANNELS_URL, STREAMS_URL);
    let downloader = Downloader::new(config.clone());

    if args.update {
        downloader.update_if_changed();
    } else {
        downloader.do_it();
    }

    let mut channels = Channel::load(&config);
    let streams = Stream::load(&config);

    let mut stream_map: HashMap<&String, &Stream> = HashMap::new();
    for stream in &streams {
        if let Some(id) = &stream.id {
            stream_map.insert(id, &stream);
        }
    }

    for channel in &mut channels {
        if let Some(ref id) = channel.id {
            if let Some(stream) = stream_map.get(id) {
                channel.stream = stream.to_owned().to_owned();
            }
        }
    }

    let filtered_channels: Vec<Channel> = channels
        .into_iter()
        .filter(|c| c.stream.url.is_some())
        .collect();

    let running = Arc::new(AtomicBool::new(true));

    // Handle SIGINT (Ctrl+C)
    let running_clone = Arc::clone(&running);
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    while running.load(Ordering::SeqCst) {
        let channel_id: String =
            match selector::get_user_selection("".to_string(), filtered_channels.clone()) {
                Ok(e) => e,
                Err(_e) => break,
            };

        if let Some(stream_url) = stream_map.get(&channel_id).and_then(|s| s.url.as_ref()) {
            open_mpv(stream_url.to_string(), args.mpv_flags.clone());
        } else {
            eprintln!("Error: No stream found for selected channel.");
        }

        // Sleep briefly to allow signal handling
        thread::sleep(Duration::from_millis(100));
    }
}

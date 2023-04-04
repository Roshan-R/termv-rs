use std::collections::HashMap;
use std::fs;

mod download;
mod selector;
mod types;
mod utils;

use download::Downloader;
use types::Channel;

use utils::open_mpv;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "termv-rs")]
#[clap(version = "0.1")]
#[clap(after_help = "   Improve me on GitHub:\n    https://github.com/Roshan-R/termv-rs")]
struct Cli {
    #[clap(default_value = "")]
    query: String,

    ///Auto update channel list to latest version.
    #[clap(env = "TERMV_AUTO_UPDATE", default_value = "true")]
    auto_update: String,

    ///  Update channel list to latest version
    #[clap(short, long, action)]
    update: bool,

    ///  Open player in fullscreen
    #[clap(short, long)]
    fullscreen: bool,

    /// Always open mpv in fullscreen.
    #[clap(env = "TERMV_FULL_SCREEN", default_value = "false")]
    env_fullscreen: String,

    ///Default arguments which are passed to mpv.
    #[clap(
        env = "TERMV_DEFAULT_MPV_FLAGS",
        default_value = "--no-resume-playback"
    )]
    mpv_flags: String,

    ///URL to the channels list. Any other URL must be in the same format as the default one.
    #[clap(
        env = "TERMV_CHANNELS_URL",
        default_value = "https://iptv-org.github.io/api/channels.json"
    )]
    channels_url: String,

    ///URL to the channel list. Any other URL must be in the same format as the default one.
    #[clap(
        env = "TERMV_STREAMS_URL",
        default_value = "https://iptv-org.github.io/api/streams.json"
    )]
    streams_url: String,
}

pub fn main() {
    let args = Cli::parse();
    utils::has_dependencies();

    let mut flags = args.mpv_flags;

    if args.env_fullscreen.as_str() == "true" || args.fullscreen {
        flags.push_str(" --fs")
    }

    let d = Downloader::new(args.streams_url, args.channels_url);

    if args.update {
        d.update();
        return;
    }

    if !d.check_file_exists() {
        d.first_download();
    } else if args.auto_update.as_str() == "true" {
        d.update_if_changed();
    }

    let channels_json = fs::read_to_string(d.json_path.clone()).unwrap();
    let channels: Vec<Channel> = serde_json::from_str(channels_json.as_str()).unwrap();
    let mut f_input = String::new();

    let mut map: HashMap<String, String> = HashMap::new();

    for x in &channels {
        let name = x.name.clone().unwrap_or("Null".to_string());
        let id = x.id.clone().unwrap_or("Null".to_string());
        map.insert(name.clone(), id);
        let country = x.country.clone().unwrap_or("Null".to_string());
        let language = match x
            .languages
            .clone()
            .unwrap_or(vec!["Null".to_string()])
            .first()
        {
            Some(c) => c.clone(),
            None => "Null".to_string(),
        };
        let category = match x
            .categories
            .clone()
            .unwrap_or(vec!["Null".to_string()])
            .first()
        {
            Some(c) => c.clone(),
            None => "Null".to_string(),
        };
        let a = format!(
            "{:<50}  |{:<15} |{:<10} |{:<10}\n",
            name, category, language, country
        );
        f_input.push_str(a.as_str());
        // let is_nsfw = x.is_nsfw.unwrap_or("Null".to_string());
    }

    loop {
        let s = match selector::get_user_selection(f_input.clone(), args.query.clone()) {
            Ok(e) => e,
            Err(_e) => return,
        };

        let channel_name = s
            .split('|')
            .rev()
            .last()
            .expect("Could not get channel name")
            .trim_end();

        let url = &channels
            .iter()
            .find(|x| x.name.clone().unwrap() == channel_name)
            .unwrap()
            .stream
            .url;

        open_mpv(url.clone().unwrap().to_owned(), flags.clone());
    }
}

use std::collections::HashMap;
use std::fs;

mod channel;
mod download;
mod selector;
mod utils;

use channel::Channel;
use download::Downloader;

use utils::open_mpv;

use clap::Parser;

#[derive(Parser)]
#[clap(name = "termv-rs")]
#[clap(version = "0.1")]
#[clap(after_help = "   Improve me on GitHub:\n    https://github.com/Roshan-R/termv-rs")]
struct Cli {
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

    ///URL to the channel list. Any other URL must be in the same format as the default one.
    #[clap(
        env = "TERMV_API_URL",
        default_value = "https://iptv-org.github.io/iptv/channels.json"
    )]
    api_url: String,
}

pub fn main() {
    let cli = Cli::parse();
    utils::has_dependencies();

    // dbg!(cli.auto_update.clone());
    // dbg!(cli.env_fullscreen.clone());
    // dbg!(cli.mpv_flags.clone());
    // dbg!(cli.api_url.clone());

    let mut flags = cli.mpv_flags;

    if cli.env_fullscreen.as_str() == "true" || cli.fullscreen {
        flags.push_str(" --fs")
    }

    let d = Downloader::new(cli.api_url);

    if cli.update {
        d.update();
        return;
    }

    if !d.check_file_exists() {
        d.first_download();
    } else if cli.auto_update.as_str() == "true" {
        d.update_if_changed();
    }

    let json = fs::read_to_string(d.json_path).expect("Error reading data file");
    let channels: Vec<Channel> = serde_json::from_str(json.as_str()).unwrap();
    let mut map = HashMap::new();

    for channel in channels.iter() {
        map.insert(channel.name.clone(), channel.url.clone());
    }

    let mut f_input = String::new();
    for x in channels.into_iter() {
        let category = match x.categories.first() {
            Some(c) => c.name.clone(),
            None => "Null".to_string(),
        };
        let language = match x.languages.first() {
            Some(c) => c.name.clone(),
            None => "Null".to_string(),
        };
        let country = match x.countries.first() {
            Some(c) => c.name.clone(),
            None => "Null".to_string(),
        };

        let a = format!(
            "{:<50}  |{:<15} |{:<10} |{:<10}\n",
            x.name, category, language, country
        );
        f_input.push_str(a.as_str());
    }

    loop {
        let s = match selector::get_user_selection(f_input.clone()) {
            Ok(e) => e,
            Err(_e) => return,
        };

        let channel_name = s
            .split('|')
            .rev()
            .last()
            .expect("Could not get channel name")
            .trim_end();

        let url = map.get(channel_name).expect("Unknown channel selected");

        open_mpv(url.to_string(), flags.clone());
    }
}

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
    ///  Update channel list to latest version
    #[clap(short, long, action)]
    update: bool,
}

pub fn main() {
    let cli = Cli::parse();
    utils::has_dependencies();

    let d = Downloader::new();

    if cli.update {
        d.update();
        return;
    }

    if !d.check_file_exists() {
        d.first_download();
    } else {
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
        let url = map
            .get(channel_name.to_string().as_str())
            .expect("Unknown channel selected");

        open_mpv(url.to_string());
    }
}

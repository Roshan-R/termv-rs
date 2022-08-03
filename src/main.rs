extern crate skim;
use skim::prelude::*;
use std::fs;
use std::{collections::HashMap, io::Cursor};

mod channel;
mod download;
mod utils;

use channel::Channel;
use download::Downloader;

use utils::open_mpv;

use platform_dirs::AppDirs;

use clap::Parser;

#[derive(Parser)]
#[clap(name = "termv-rs")]
struct Cli {
    #[clap(short, long, action)]
    update: bool,
}

pub fn main() {
    let cli = Cli::parse();

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

    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .layout("reverse")
        .build()
        .unwrap();

    let mut json_path = AppDirs::new(Some("name"), true).unwrap().cache_dir;
    json_path.push("d.json");

    let json = fs::read_to_string(json_path).expect("Error reading data file");

    let channels: Vec<Channel> = serde_json::from_str(json.as_str()).unwrap();

    let mut map = HashMap::new();

    for channel in channels.iter() {
        map.insert(channel.name.clone(), channel.url.clone());
    }

    let mut new_input = String::new();
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
        new_input.push_str(a.as_str());
    }

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(new_input));

    let skim_output = Skim::run_with(&options, Some(items)).unwrap();

    if skim_output.is_abort {
        return;
    }

    let s = skim_output
        .selected_items
        .get(0)
        .unwrap()
        .output()
        .to_string();

    let channel_name = s.split('|').rev().last().unwrap().trim_end();
    let url = map.get(channel_name.to_string().as_str()).unwrap();

    open_mpv(url.to_string());
}

extern crate skim;
use skim::prelude::*;
use std::fs;
use std::process::Command;
use std::{collections::HashMap, io::Cursor};

mod channel;
mod download;

use whoami::username;

use channel::Channel;
use download::Downloader;

pub fn main() {
    let d = Downloader::new();

    if !d.check_file_exists() {
        d.first_download();
    } else {
        d.update_if_changed();
    }

    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(false)
        .build()
        .unwrap();

    let json_path = format!("/home/{}/.cache/termv-rs/d.json", username());

    let json = fs::read_to_string(json_path).expect("Error reading data file");

    let channels: Vec<Channel> = serde_json::from_str(json.as_str()).unwrap();
    // let names: Vec<String> = channels.iter().map(|f| f.name.clone()).collect();

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

    // // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(new_input));

    // // // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let mut url = "https://google.com";

    for item in selected_items.iter() {
        let i = item.output();
        let s = i.split('|').rev().last().unwrap().trim_end();
        url = map.get(s).unwrap();
    }

    println!("Opening Mpv..");

    let mut output = Command::new("mpv")
        .arg(url)
        .spawn()
        .expect("Error spawing mpv");

    output.wait().unwrap();
}

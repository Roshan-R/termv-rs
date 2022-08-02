use std::fs;
use std::path::Path;

use whoami::username;

use ureq;

pub fn check_file_exists() -> bool {
    let username = username();
    let json_path = format!("/home/{}/.cache/termv-rs/d.json", username);
    Path::new(json_path.as_str()).exists()
}

pub fn download() {
    let username = username();
    let dir_path = format!("/home/{}/.cache/termv-rs/", username);
    let json_path = format!("/home/{}/.cache/termv-rs/d.json", username);

    let d = Path::new(dir_path.as_str());
    fs::create_dir_all(d).unwrap();

    println!("Downloading json file...");

    let resp = ureq::get("https://iptv-org.github.io/iptv/channels.json")
        .call()
        .unwrap();

    let body = resp.into_string().unwrap();

    println!("Done!");

    fs::write(json_path, body).expect("Unable to write file");
}

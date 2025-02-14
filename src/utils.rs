// Copied from https://github.com/j0lol/rs-youtube/blob/main/src/backend/utils.rs

use std::process::Command;

#[cfg(target_os = "windows")]
pub fn open_mpv(mut url: String, args: String) {
    url = url.replace("(", r"\(").replace(")", r"\)");
    let command = format!("mpv {} {}", args, url);

    println!("Fetching channel, please wait...");

    let mut output = Command::new("cmd")
        .args(&["/C", command.as_str()])
        .spawn()
        .expect("failed to execute process");

    output.wait().unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn open_mpv(mut url: String, args: String) {
    url = url.replace("(", r"\(").replace(")", r"\)");

    let command = format!("mpv {} {}", args, url);

    println!("Fetching channel, please wait...");

    let mut output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("failed to execute process");
    output.wait().unwrap();
}

pub fn has_dependencies() {
    let mut dependencies = vec!["mpv"];

    if cfg!(windows) {
        dependencies.push("fzf");
    }

    for d in dependencies {
        which::which(d)
            .expect(format!("Cannot find {} in PATH. See if it's installed or not", d).as_str());
    }
}

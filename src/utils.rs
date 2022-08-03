// Copied from https://github.com/j0lol/rs-youtube/blob/main/src/backend/utils.rs

use std::process::Command;

pub fn open_mpv(url: String) {
    let command = format!("mpv {}", url);

    println!("Opening url {}", url);

    let mut output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command.as_str()])
            .spawn()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .spawn()
            .expect("failed to execute process")
    };
    output.wait().unwrap();
}

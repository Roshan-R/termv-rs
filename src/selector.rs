use std::process::Command;
use std::process::Stdio;
use std::io::Write;
//extern crate skim;
//use skim::SkimOptionsBuilder;

//use skim::prelude::*;

#[cfg(target_os = "windows")]
pub fn get_user_selection(buffer: String) -> String {

    let selectors = buffer;


    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("fzf command failed to start");

    let mut stdin = fzf.stdin.take().expect("Failed to take stdin");
    stdin
        .write_all(selectors.to_string().as_bytes())
        .expect("Failed to write to stdin");

    let output = fzf.wait_with_output().expect("Failed to read stdout of fzf");
    String::from_utf8_lossy(&output.stdout).to_string()
    
}


#[cfg(not(target_os = "windows"))]
pub fn setup_skim() -> SkimOptionsBuilder {
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .layout("reverse")
        .header(Some("Select channel (press Escape to exit)"))
        .build()
        .unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn get_user_selection(buffer: String) -> String{
    let options = setup_skim();

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
}



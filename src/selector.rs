#[derive(Debug)]
pub enum UserSelectionResult {
    None,
}

#[cfg(target_os = "windows")]
pub fn get_user_selection(buffer: String) -> Result<String, UserSelectionResult> {
    use std::io::Write;
    use std::process::Command;
    use std::process::Stdio;
    let selectors = buffer;

    let mut fzf = Command::new("fzf")
        .args([
            "--reverse",
            "--header",
            "Select channel (press Escape to exit)",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("fzf command failed to start");

    let mut stdin = fzf.stdin.take().expect("Failed to take stdin");
    stdin
        .write_all(selectors.to_string().as_bytes())
        .expect("Failed to write to stdin");

    let output = fzf
        .wait_with_output()
        .expect("Failed to read stdout of fzf");

    let selection = String::from_utf8_lossy(&output.stdout).to_string();

    match selection.is_empty() {
        false => Ok(selection),
        true => Err(UserSelectionResult::None),
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_user_selection(buffer: String) -> Result<String, UserSelectionResult> {
    extern crate skim;
    use skim::prelude::SkimOptionsBuilder;
    use skim::prelude::*;

    use std::io::Cursor;
    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .layout("reverse")
        .header(Some("Select channel (press Escape to exit)"))
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(buffer));

    let skim_output = Skim::run_with(&options, Some(items)).unwrap();

    if skim_output.is_abort {
        return Err(UserSelectionResult::None);
    }

    let s = skim_output
        .selected_items
        .get(0)
        .unwrap()
        .output()
        .to_string();
    return Ok(s);
}

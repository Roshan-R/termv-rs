use crate::Channel;

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
pub fn get_user_selection(
    query: String,
    channels: Vec<Channel>,
) -> Result<String, UserSelectionResult> {
    extern crate skim;

    use skim::prelude::*;

    let options = SkimOptionsBuilder::default()
        .query(Some(query))
        .height("100%".to_string())
        .layout("reverse".to_string())
        .header(Some("Select channel (press Escape to exit)".to_string()))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for channel in channels {
        let _ = tx_item.send(Arc::new(channel));
    }
    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap();
    let first_item = selected_items.first().unwrap();
    return Ok(first_item.output().to_string());
}

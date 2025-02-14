# termv-rs
Termv-rs is a complete rewrite of [termv](https://github.com/Roshan-R/termv) in rust. 

Along with the speed improvements, Termv-rs now works in windows too provided that fzf is installed. 




https://github.com/user-attachments/assets/51d2dfef-862e-4782-b575-53b6b0120157



## Usage

```
termv-rs 0.1

USAGE:
    termv-rs [OPTIONS] [ARGS]

ARGS:
    <AUTO_UPDATE>       Auto update channel list to latest version [env: TERMV_AUTO_UPDATE=]
                        [default: true]
    <ENV_FULLSCREEN>    Always open mpv in fullscreen [env: TERMV_FULL_SCREEN=] [default: false]
    <MPV_FLAGS>         Default arguments which are passed to mpv [env:
                        TERMV_DEFAULT_MPV_FLAGS=] [default: --no-resume-playback]
    <API_URL>           URL to the channel list. Any other URL must be in the same format as the
                        default one [env: TERMV_API_URL=] [default:
                        https://iptv-org.github.io/iptv/channels.json]

OPTIONS:
    -f, --fullscreen    Open player in fullscreen
    -h, --help          Print help information
    -u, --update        Update channel list to latest version
    -V, --version       Print version information

   Improve me on GitHub:
    https://github.com/Roshan-R/termv-rs
```

use clap::Parser;
#[derive(Parser, Debug)]
#[clap(name = "termv-rs")]
#[clap(version = "0.1")]
#[clap(after_help = "   Improve me on GitHub:\n    https://github.com/Roshan-R/termv-rs")]
pub struct Args {
    #[clap(default_value = "")]
    query: String,

    ///Auto update channel list to latest version.
    #[clap(env = "TERMV_AUTO_UPDATE", default_value = "true")]
    auto_update: String,

    ///  Update channel list to latest version
    #[clap(short, long, action)]
    pub update: bool,

    ///  Open player in fullscreen
    #[clap(short, long)]
    fullscreen: bool,

    /// Always open mpv in fullscreen.
    #[clap(env = "TERMV_FULL_SCREEN", default_value = "false")]
    env_fullscreen: String,

    ///Default arguments which are passed to mpv.
    #[clap(
        env = "TERMV_DEFAULT_MPV_FLAGS",
        default_value = "--no-resume-playback"
    )]
    pub mpv_flags: String,

    ///URL to the channels list. Any other URL must be in the same format as the default one.
    #[clap(
        env = "TERMV_CHANNELS_URL",
        default_value = "https://iptv-org.github.io/api/channels.json"
    )]
    channels_url: String,

    ///URL to the channel list. Any other URL must be in the same format as the default one.
    #[clap(
        env = "TERMV_STREAMS_URL",
        default_value = "https://iptv-org.github.io/api/streams.json"
    )]
    streams_url: String,
}

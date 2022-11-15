use clap::Parser;

/// Config via command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// URL of twitchemotes.com channel page
    url: String,

    /// Destination directory for emote files
    #[arg(short, long)]
    output_directory: Option<String>,
}

impl Config {
    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn output_directory(&self) -> &Option<String> {
        &self.output_directory
    }
}

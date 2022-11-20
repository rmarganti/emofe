use clap::Parser;

use crate::config::Config;
use crate::fetcher::EmoFetcher;
use std::error::Error;
use std::process::ExitCode;

mod config;
mod fetcher;
mod parser;

fn main() -> ExitCode {
    match try_map() {
        Ok(_) => ExitCode::from(0),
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::from(1)
        }
    }
}

fn try_map() -> Result<()> {
    let config = Config::parse();

    let fetcher = EmoFetcher::new();
    let remote_page_urls = fetcher.emote_page_urls_for_index_page(config.url())?;
    let result = fetcher.download_all_emotes(&remote_page_urls, config.output_directory())?;

    if result.has_failures() {
        println!("There were failures:\n");

        for failure in result.failures() {
            println!("{}", failure);
        }
    }

    Ok(())
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

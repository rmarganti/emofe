use crate::fetcher::EmoFetcher;
use std::error::Error;
use std::io;
use std::io::Write;
use std::process::ExitCode;

mod fetcher;
mod parser;

fn main() -> ExitCode {
    match try_map() {
        Ok(_) => ExitCode::from(0),
        Err(e) => {
            println!("{}", e);
            ExitCode::from(1)
        }
    }
}

fn try_map() -> Result<(), Box<dyn Error>> {
    let starting_url = read_starting_url()?;

    let fetcher = EmoFetcher::new();
    let remote_page_urls = fetcher.emote_page_urls_for_index_page(starting_url)?;
    let result = fetcher.download_all_emotes(remote_page_urls);

    if result.has_failures() {
        println!("There were failures:\n");

        for failure in result.failures() {
            println!("{}", failure.name());
        }
    }

    Ok(())
}

fn read_starting_url() -> Result<String, Box<dyn Error>> {
    print!("Enter starting URL: ");
    io::stdout().flush()?;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    Ok(buf.trim().to_string())
}

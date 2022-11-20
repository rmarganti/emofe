use crate::parser;
use scraper::Html;
use std::fmt;
use std::io::Write;
use std::{io::stdout, time::Duration};
use ureq::{Agent, AgentBuilder};

pub struct EmoFetcher {
    client: Agent,
}

impl EmoFetcher {
    pub fn new() -> Self {
        let agent: Agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();

        EmoFetcher { client: agent }
    }

    /// Given a channel index page, return a list of URLs for emotes.
    pub fn emote_page_urls_for_index_page(&self, index_url: &String) -> crate::Result<Vec<String>> {
        println!("Fetching index");
        let document = self.document_for_url(index_url)?;
        let urls = parser::parse_index_document(&document);

        Ok(urls)
    }

    /// Download the emote associated with each of the given emote URLs.
    pub fn download_all_emotes(
        &self,
        emote_urls: &Vec<String>,
        output_dir: &Option<String>,
    ) -> crate::Result<DownloadAllEmotesResult> {
        println!("Fetching info for {} emotes", emote_urls.len());

        let mut result = DownloadAllEmotesResult::new();

        let emotes_info: Vec<parser::ImageInfo> = emote_urls
            .iter()
            .filter_map(|emote_url| match self.fetch_emote_info(emote_url) {
                Ok(emote) => Some(emote),
                Err(e) => {
                    result.add_failure("unknown".to_string(), e.to_string());
                    None
                }
            })
            .collect();

        println!("");

        println!("Downloading {} emotes", emotes_info.len());

        for emote_info in &emotes_info {
            match self.download_emote(emote_info, output_dir) {
                Ok(_) => {}
                Err(e) => {
                    result.add_failure(emote_info.name().to_string(), e.to_string());
                }
            };
        }

        println!("");

        Ok(result)
    }

    /// Get the name and image URL for the emote page at the given URL.
    fn fetch_emote_info(&self, url: &String) -> crate::Result<parser::ImageInfo> {
        let mut absolute_url = String::from("https://twitchemotes.com");
        absolute_url.push_str(url);

        print!(".");
        stdout().flush()?;

        let document = self.document_for_url(&absolute_url)?;
        Ok(parser::parse_emote_document(&document))
    }

    /// Get a parsed Document for the given URL.
    fn document_for_url(&self, url: &String) -> crate::Result<Html> {
        let body = self.client.get(&url).call()?.into_string()?;
        let document = Html::parse_document(body.as_str());

        Ok(document)
    }

    // Download a single emote.
    fn download_emote(
        &self,
        emote_info: &parser::ImageInfo,
        output_dir: &Option<String>,
    ) -> crate::Result<()> {
        print!(".");
        stdout().flush()?;

        let response = self.client.get(emote_info.url()).call()?;

        let file_path = emote_info
            .file_path(&response, output_dir)
            .expect("Unable to get destination path");

        let mut file = std::fs::File::create(file_path)?;
        let mut reader = response.into_reader();
        std::io::copy(&mut reader, &mut file)?;

        Ok(())
    }
}

pub struct DownloadAllEmotesResult {
    failures: Vec<EmoteFailure>,
}

pub struct EmoteFailure {
    name: String,
    message: String,
}

impl fmt::Display for EmoteFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.message)
    }
}

impl DownloadAllEmotesResult {
    pub fn new() -> Self {
        DownloadAllEmotesResult {
            failures: Vec::new(),
        }
    }

    pub fn add_failure(&mut self, name: String, message: String) {
        self.failures.push(EmoteFailure { name, message });
    }

    pub fn has_failures(&self) -> bool {
        !self.failures.is_empty()
    }

    pub fn failures(&self) -> &Vec<EmoteFailure> {
        &self.failures
    }
}

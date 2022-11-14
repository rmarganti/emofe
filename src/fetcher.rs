use crate::parser;
use reqwest::blocking::Client;
use reqwest::Error as ReqwestError;
use scraper::Html;
use std::error::Error;
use std::io::Cursor;

pub struct EmoFetcher {
    client: Client,
}

impl EmoFetcher {
    pub fn new() -> Self {
        EmoFetcher {
            client: Client::new(),
        }
    }

    /// Given a channel index page, return a list of URLs for emotes.
    pub fn emote_page_urls_for_index_page(
        &self,
        index_url: String,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        println!("Fetching index");
        let document = self.document_for_url(index_url)?;
        let urls = parser::parse_index_document(&document);

        Ok(urls)
    }

    /// Download the emote associated with each of the given emote URLs.
    pub fn download_all_emotes(&self, emote_urls: Vec<String>) -> DownloadAllEmotesResult {
        let emotes_info: Vec<parser::ImageInfo> = emote_urls
            .iter()
            .map(|emote_url| self.fetch_emote_info(emote_url))
            .collect();

        let mut result = DownloadAllEmotesResult::new();

        for emote_info in &emotes_info {
            match self.download_emote(emote_info) {
                Ok(_) => result.add_success(emote_info.clone()),
                Err(_) => result.add_failure(emote_info.clone()),
            };
        }

        result
    }

    /// Get the name and image URL for the emote page at the given URL.
    fn fetch_emote_info(&self, url: &String) -> parser::ImageInfo {
        let mut absolute_url = String::from("https://twitchemotes.com");
        absolute_url.push_str(url);

        println!("Fetching {url}");
        let document = self.document_for_url(absolute_url).unwrap();
        parser::parse_emote_document(&document)
    }

    /// Get a parsed Document for the given URL.
    fn document_for_url(&self, url: String) -> Result<Html, ReqwestError> {
        let body = self.client.get(url).send()?.text()?;
        let document = Html::parse_document(body.as_str());

        Ok(document)
    }

    // Download the an emote to the desktop
    fn download_emote(&self, emote_info: &parser::ImageInfo) -> Result<(), Box<dyn Error>> {
        println!("Downloading {}", emote_info.name());

        let response = self.client.get(emote_info.url()).send()?;

        let file_path = emote_info
            .file_path(&response)
            .expect("Unable to get destination path");

        let mut file = std::fs::File::create(file_path)?;
        let bytes = response.bytes()?;
        let mut content = Cursor::new(bytes);
        std::io::copy(&mut content, &mut file)?;

        Ok(())
    }
}

pub struct DownloadAllEmotesResult {
    successful: Vec<parser::ImageInfo>,
    failed: Vec<parser::ImageInfo>,
}

impl DownloadAllEmotesResult {
    pub fn new() -> Self {
        DownloadAllEmotesResult {
            successful: Vec::new(),
            failed: Vec::new(),
        }
    }

    pub fn add_success(&mut self, image_info: parser::ImageInfo) {
        self.successful.push(image_info);
    }

    pub fn add_failure(&mut self, image_info: parser::ImageInfo) {
        self.failed.push(image_info);
    }

    pub fn has_failures(&self) -> bool {
        !self.failed.is_empty()
    }
}

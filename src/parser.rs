use regex::Regex;
use scraper::{Html, Selector};
use ureq::Response;

/// Parse all the emote URLs out of the given index HTML document.
pub fn parse_index_document(document: &Html) -> Vec<String> {
    // We are certain this will parse.
    let emote_url_regex = Regex::new(r"/channels/\d+/emotes/[a-z0-9_]+").unwrap();
    let link_selector = Selector::parse("a").expect("Unable to parse selector");

    document
        .select(&link_selector)
        .filter_map(|a| a.value().attr("href"))
        .filter(|val| emote_url_regex.is_match(&val))
        .map(|val| val.to_string())
        .collect()
}

/// Parse the name and image URL out of the given emote HTML document.
pub fn parse_emote_document(document: &Html) -> ImageInfo {
    let name_selector = Selector::parse(".card-header h2").unwrap();
    let name = document.select(&name_selector).next().unwrap().inner_html();

    let img_selector = Selector::parse("img").unwrap();
    let emote_regex = Regex::new(
        r"https://static-cdn.jtvnw.net/emoticons/v2/[a-z0-9_]+/(animated|static)/light/3\.0",
    )
    .unwrap();

    let url = document
        .select(&img_selector)
        .filter_map(|el| el.value().attr("src"))
        .filter(|src| emote_regex.is_match(src))
        .next()
        .unwrap()
        .to_string();

    ImageInfo { name, url }
}

#[derive(Debug, Clone)]
pub struct ImageInfo {
    name: String,

    url: String,
}

impl ImageInfo {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    /// Determine the destination file path for
    /// an emote given its file name and content-type.
    pub fn file_path(&self, response: &Response) -> Option<std::path::PathBuf> {
        let mut file_path = dirs::desktop_dir()?;
        file_path.push("emotes");

        let mut file_name = self.name().to_lowercase();
        let content_type = response.header("content-type");

        let extension = match content_type {
            Some(header) => match header {
                "image/png" => Some(".png"),
                "image/gif" => Some(".gif"),
                _ => None,
            },
            None => None,
        };

        if let Some(ext) = extension {
            file_name.push_str(ext)
        }

        file_path.push(file_name);

        Some(file_path)
    }
}

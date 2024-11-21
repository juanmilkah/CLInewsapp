use serde::Deserialize;
use serde_json;

use ureq;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("failed fetching data")]
    RequestFailed(ureq::Error),
    #[error("failed translating response to string")]
    FailResponsetoString(std::io::Error),
    #[error("failed parsing JSON to string")]
    FailedParsingToString(serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}
#[derive(Deserialize, Debug)]

pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| NewsApiError::RequestFailed(e))?
        .into_string()
        .map_err(|e| NewsApiError::FailResponsetoString(e))?;
    let articles: Articles =
        serde_json::from_str(&response).map_err(|e| NewsApiError::FailedParsingToString(e))?;
    Ok(articles)
}

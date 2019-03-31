#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate reqwest;
#[allow(unused_imports)]
#[macro_use]
extern crate failure;

/// Error handling used in this library
pub type Result<T> = std::result::Result<T, failure::Error>;

use reqwest::Url;


pub struct TriblerRestApi {
    c : reqwest::Client,
    baseurl: String,
}

impl TriblerRestApi {
    pub fn new(baseurl: String) -> Self {
        TriblerRestApi {
            c : reqwest::Client::new(),
            baseurl,
        }
    }

    /// Query Tribler and return current downloads list
    pub fn get_downloads(&self) -> Result<Vec<Download>>  {
        #[derive(Deserialize)]
        struct Reply {
            downloads: Vec<Download>,
        }

        let url = Url::parse(&format!("{}/downloads?get_pieces=0", &self.baseurl))?;

        let reply : Reply =
            self.c
            .get(url)
            .send()?
            .json()?;

        Ok(reply.downloads)
    }


    pub fn get_search_completions(&self, prefix: &str) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct Reply {
            completions: Vec<String>,
        }


        let url = Url::parse(&format!("{}/search/completions", &self.baseurl))?;

        let reply : Reply = self.c
            .get(url)
            .query(&[("q", prefix)])
            .send()?
            .json()?;

        Ok(reply.completions)
    }
}

/// Information about specific Tribler downloads
#[derive(Debug,Deserialize)]
#[serde(rename_all="snake_case")]
pub struct Download {
    pub name: String,
    pub size: u64,
    pub progress: f64,
}



#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate reqwest;
#[allow(unused_imports)]
#[macro_use]
extern crate failure;

/// Error handling using in this library
pub type Result<T> = std::result::Result<T, failure::Error>;


/// Information about specific Tribler downloads
#[derive(Debug,Deserialize)]
#[serde(rename_all="snake_case")]
pub struct Download {
    pub name: String,
    pub size: u64,
    pub progress: f64,
}

/// Query Tribler and return current downloads list
pub fn get_downloads() -> Result<Vec<Download>> {
    #[derive(Debug,Deserialize)]
    #[serde(rename_all="snake_case")]
    struct Reply {
        downloads: Vec<Download>,
    }

    let reply : Reply =
        reqwest::get("http://localhost:8085/downloads?get_pieces=0")?
        .json()?;

    Ok(reply.downloads)
}

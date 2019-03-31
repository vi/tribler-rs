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

    pub fn begin_search(&self, q: SearchQuery) -> Result<Vec<SearchResult>> {
        #[derive(Deserialize)]
        struct Reply {
            //last: u64,
            uuid: String,
            results: Vec<SearchResult>,
        }


        let url = Url::parse(&format!("{}/search", &self.baseurl))?;

        let reply : Reply = self.c
            .get(url)
            .query(&q)
            .send()?
            .json()?;

        ensure!(reply.uuid == q.uuid, "UUID mismatch in search reply");
        
        Ok(reply.results)
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

/// Information about one search result
#[derive(Debug,Deserialize)]
#[serde(rename_all="snake_case")]
pub struct SearchResult {
    /// Main name of the torrent
    pub name: String,
    /// like "video" or "Audio".
    pub category : String,
    /// Status like LEGACY_ENTRY or TODELETE. To be converted into an enum.
    pub status: u64,
    
    /// Number of known downloaders of this torrent
    pub num_leechers: u64,
    /// Number of known uploaders of this torrent
    pub num_seeders: u64,

    /// Probably UNIX time of something
    pub last_tracker_check: u64,

    /// Identifier of this torrent
    pub infohash: String,

    /// Can it be other than "torrent"?
    pub r#type: String,
    
    /// Some ID. I don't know if it is within the search 
    pub id: u64,
    
    /// Total file sizes in bytes?
    pub size: u64,
}

#[derive(Debug,Serialize)]
pub struct SearchQuery {
    /// Search query line
    pub filter: String,

    /// Probably number of results to include
    pub last: u32,
    /// ID of this search
    pub uuid: String,
    /// Sorting mode. "category"?
    pub sort_by: String,

    /// I don't know what it does mean, defaults to "torrent"
    pub metadata_type: String,

    /// Probably enables family filter
    #[serde(serialize_with = "bool2int")]
    pub hide_xxx: bool,

    /// Ascending sorting for results
    #[serde(serialize_with = "bool2int")]
    pub sort_asc: bool,

    /// Probably set on first search within this `uuid`
    #[serde(serialize_with = "bool2int")]
    pub first: bool,
}

impl SearchQuery {
    /// Create search query as Tribler GUI does for the first search
    pub fn new(uuid:String, filter: String) -> Self {
        SearchQuery {
            filter,
            first: true,
            last: 50,
            uuid,
            
            sort_by: "category".to_string(),
            metadata_type: "torrent".to_string(),
            hide_xxx: true,
            sort_asc: true,
        }
    }
}


fn bool2int<S:serde::Serializer>(b:&bool,s:S) -> std::result::Result<S::Ok, S::Error> 
{
    s.serialize_u8(
        if *b {
            1
        } else {
            0
        }
    )
}

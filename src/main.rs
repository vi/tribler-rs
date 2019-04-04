extern crate tribler;
extern crate structopt;
extern crate byte_unit;

use structopt::StructOpt;

use byte_unit::Byte;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Opt {
    /// Show short information about each download
    ListDownloads,
    /// Start searching for some query and give initial results
    Search {
        /// Text to search in the torrent database for
        query: String,

        #[structopt(long="uuid",default_value="e09774f87c32414cad817e6d643a7235")]
        uuid: String,
    },
    /// Give list of search completions, one per line
    SearchComplete {
        /// Prefix to complete
        prefix: String,
    },
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let opt = Opt::from_args();

    let tribler_default_urlbase = "http://localhost:8085";
    let tribler_urlbase = std::env::var("TRIBLER_REST")
            .unwrap_or(tribler_default_urlbase.to_string());

    let trib = tribler::TriblerRestApi::new(tribler_urlbase);
    
    match opt {
        Opt::ListDownloads => {
            let ds = trib.get_downloads()?;
            for d in ds {
                let s = Byte::from_bytes(d.size.into());
                println!(
                    "{:>11} {:>6}%  {}",
                    s.get_appropriate_unit(true).to_string(),
                    d.progress * 100.0,
                    d.name,
                );
            }
        },
        Opt::SearchComplete {prefix} => {
            let cs = trib.get_search_completions(&prefix)?;
            for i in cs {
                println!("{}", i);
            }
        },
        Opt::Search { query, uuid } => {
            let q = tribler::SearchQuery::new(
                uuid,
                query,
            );
            let srs = trib.begin_search(q)?;
            for sr in srs {
                let sz = Byte::from_bytes(sr.size.into());
                println!(
                    "{:>11} {}   magnet:?xt=urn:btih:{}", 
                    sz.get_appropriate_unit(true).to_string(),
                    sr.name,
                    sr.infohash,
                );
            }
        },
    }

    Ok(())
}

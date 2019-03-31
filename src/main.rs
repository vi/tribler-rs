extern crate tribler;
extern crate structopt;
extern crate byte_unit;

use structopt::StructOpt;

use byte_unit::Byte;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Opt {
    /// Operate on downloads
    Downloads(Downloads),
    /// Deal with searches
    Search(Search)
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Downloads {
    /// List downloads
    List{},
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Search {
    /// Search completions
    Completions{
        /// Prefix to complete
        prefix: String,
    },
    /// Start new search
    Begin {
        /// Text to search in the torrent database for
        query: String,

        #[structopt(long="uuid",default_value="e09774f87c32414cad817e6d643a7235")]
        uuid: String,
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let opt = Opt::from_args();

    let trib = tribler::TriblerRestApi::new("http://localhost:8085".to_string());
    
    match opt {
        Opt::Downloads(cmd) => match cmd {
            Downloads::List{} => {
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
        },
        Opt::Search(cmd) => match cmd {
            Search::Completions { prefix } => {
                let cs = trib.get_search_completions(&prefix)?;
                for i in cs {
                    println!("{}", i);
                }
            },
            Search::Begin { query, uuid } => {
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
            }
        }
    }

    Ok(())
}

extern crate tribler;
extern crate structopt;
extern crate byte_unit;

use structopt::StructOpt;


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
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let opt = Opt::from_args();

    let trib = tribler::TriblerRestApi::new("http://localhost:8085".to_string());
    
    match opt {
        Opt::Downloads(cmd) => match cmd {
            Downloads::List{} => {
                let ds = trib.get_downloads()?;
                for d in ds {
                    let s = byte_unit::Byte::from_bytes(d.size.into());
                    println!("{:>11} {:>6}%  {}",
                        s.get_appropriate_unit(true).to_string(),
                        d.progress * 100.0,
                        d.name,
                    );
                }
            }
        },
        Opt::Search(cmd) => match cmd {
            Search::Completions { prefix } => {
                let cs = trib.get_search_completions(&prefix)?;
                for i in cs {
                    println!("{}", i);
                }
            }
        }
    }

    Ok(())
}

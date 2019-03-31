extern crate tribler;
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Downloads {
    /// List downloads
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Opt {
    Downloads(Downloads),
}



#[derive(Debug,Deserialize)]
#[serde(rename_all="snake_case")]
struct Download {
    name: String,
    size: u64,
    progress: f64,
}

#[derive(Debug,Deserialize)]
#[serde(rename_all="snake_case")]
struct Reply {
    downloads: Vec<Download>,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let downloads : Reply = reqwest::get("http://localhost:8085/downloads?get_pieces=0")?.json()?;

    println!("{:#?}", downloads);
    Ok(())
}

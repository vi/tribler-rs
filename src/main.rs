extern crate tribler;
extern crate structopt;
extern crate byte_unit;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Downloads {
    /// List downloads
    List{},
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Opt {
    /// Operate on downloads
    Downloads(Downloads),
}



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let opt = Opt::from_args();
    
    match opt {
        Opt::Downloads(cmd) => match cmd {
            Downloads::List{} => {
                let ds = tribler::get_downloads()?;
                for d in ds {
                    let s = byte_unit::Byte::from_bytes(d.size.into());
                    println!("{:>11} {:>6}%  {}",
                        s.get_appropriate_unit(true).to_string(),
                        d.progress * 100.0,
                        d.name,
                    );
                }
            }
        }
    }

    Ok(())
}

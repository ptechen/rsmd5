use exitfailure::ExitFailure;
use serde_json::json;
use structopt::StructOpt;
pub mod create_md5;
use create_md5::create_md5::{create_md5,Opt};

fn main() -> Result<(), ExitFailure> {
    let args = Opt::from_args();
    let val = create_md5(args).unwrap();
    println!("{}", json!({"result": val}));
    Ok(())
}


#![allow(dead_code, unused_variables, unused_imports)]

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Result};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// asset
    #[clap(short, long, value_parser)]
    insert: String,
}

// most basic asset format, name, id (from DB), version
#[derive(Debug, Serialize, Deserialize)]
struct Asset {
    name: Option<String>,
    id: Option<u32>,
    version: Option<u32>,
}

fn main() {
    let args = Args::parse();

    // get the insert args ("the asset to insert into the DB")
    let asset_str = args.insert.to_string();

    let asset_result: serde_json::Result<Asset> = serde_json::from_str(&asset_str);

    let asset: Asset;
    match asset_result {
        Ok(a) => asset = a,
        Err(r) => {
            print!("Err: bad json format: {}", asset_str);
            panic!();
        }
    }

    if asset.id.is_none() && asset.name.is_none() {
        print!("Err: 'id' and 'name' are None: nothing to do");
        panic!();
    }

    if asset.id.is_none() {
        //create new asset, set version to 1
        //return asset

        // we checked above that is asset.name isn't none, OK to unwrap it.
        let new_asset = Asset {
            name: Some(asset.name.unwrap()),
            id: Some(123),
            version: Some(1),
        };
        print!("{:?}", new_asset);
    } else {
        // check in the database for that asset

        // asset ID not found

        // print!("Asset ID: {} not found", asset.id);
        // panic!();

        // asset ID found,
        // find the latest version.
    }
}

#![allow(dead_code, unused_variables, unused_imports)]

use bson::{bson, Bson};
use mongodb::bson::{to_document, Document};
use mongodb::{bson::doc, options::FindOptions};
use serde::{Deserialize, Serialize};

fn main() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Asset {
        name: String,
        latest_approved: u32,
        latest_version: u32,
        versions: Vec<Bson>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct AssetVersion {
        version: u32,
        source: String,
        approved: bool,
        status: u32,
    }

    // create 2 versions manually
    let bson_version: Bson = bson!({
        "version": 12,
        "source":"my_file_12.hip",
        "approved":false,
        "status":0,
    });

    let bson_version2: Bson = bson!({
        "version": 13,
        "source":"my_file_13.hip",
        "approved":false,
        "status":0,
    });

    // create a vector of Bson and push the 2 versions to it
    let mut v: Vec<Bson> = Vec::new();
    v.push(bson_version);
    v.push(bson_version2);
    // convert to an bson array
    let array = bson::to_bson(&v).unwrap();

    let bson_asset: Bson = bson!({
        "name":"my asset name",
        "latest_approved":10,
        "latest_version":12,
        "versions":&array,
    });

    let ass: Asset = bson::from_bson(bson_asset).unwrap();

    println!(
        "{} {} {} {:?}",
        ass.name, ass.latest_version, ass.latest_approved, ass.versions
    );
}

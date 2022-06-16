#![allow(dead_code, unused_variables, unused_imports)]

use clap::Parser;
use futures::stream::TryStreamExt;
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use mongodb::{bson::doc, options::FindOptions};
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
    source: Option<String>,
}



// TO DO.
// - connect to database
// - insert new document
// - check latest version
// - increment version
// - add dictionaries for versions instead of u32




// for nowm, returns Collection as an option (should be a result)
async fn db_connect() -> Option<mongodb::Collection<Document>>{

    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await;

    match client{
        Ok(c) =>{
            let database = c.database("gusfring");
            let collection: mongodb::Collection<Document> = database.collection("chicken");
            println!(">> {:?}", collection);
            Some(collection)
        },
        Err(e) => None
    }
}



// async fn main() {

#[tokio::main]
async fn main() {//-> Result<(), Box<dyn std::error::Error>> {
// fn main() {
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

    let db = db_connect().await;
    println!("DB connected: {:?}" , db.unwrap());

    if asset.id.is_none() {
        //create new asset, set version to 1
        //return asset

        // we checked above that is asset.name isn't none, OK to unwrap it.
        let new_asset = Asset {
            name: Some(asset.name.unwrap()),
            id: Some(123),
            version: Some(1),
            source: None,
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

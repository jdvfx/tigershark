#![allow(dead_code, unused_variables, unused_imports)]


use bson::{bson, Bson};
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use mongodb::bson::{doc, Document};
use mongodb::Client;
use mongodb:: options::FindOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::io::Cursor;



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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("gusfring");
    let collection = database.collection::<Document>("balls");



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
    let bytes = bson::to_raw_document_buf(&ass).unwrap();
    let doc = bytes.to_document().unwrap();

    let b = doc!{
        "name":"my asset name",
        "latest_approved":10,
        "latest_version":12,
        "versions":&array,
    };

    // println!("{:?}", bytes);
    // let doc =  Document::from_reader(&mut bytes.to_document()).unwrap();

    // let _b = collection.insert_one(doc, None).await;
    let _b = collection.insert_one(b, None).await;


    // println!(
    //     "{} {} {} {:?}",
    //     ass.name, ass.latest_version, ass.latest_approved, ass.versions
    // );

    Ok(())
}

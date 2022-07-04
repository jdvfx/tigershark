// use mongodb::{
// options::{ClientOptions, UpdateOptions},
//     Client,
// };
// use futures::stream::TryStreamExt;
// use serde_json::{Error, Result};
// use mongodb::{bson::doc, bson::Document, options::FindOptions};
// use serde::{Deserialize, Serialize, Deserializer};
// use mongodb::results::InsertOneResult;

#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use bson::{bson, Bson};
use clap::Parser;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, bson::Document};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::path::Display as pathDisplay;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// asset
    #[clap(short, long, value_parser)]
    insert: String,
}

// for CLAP de-serialize
#[derive(Debug, Serialize, Deserialize)]
struct Insert {
    name: Option<String>,
    id: Option<String>,
    version: Option<u32>,
}

// for bson
#[derive(Debug, Serialize, Deserialize)]
struct AssetVersion {
    version: u32,
    source: String,
    approved: bool,
    status: u32,
}

// use bson::Document;

#[derive(Debug, Serialize, Deserialize)]
struct Asset {
    name: String,
    color: String,
    price: u32,
    food: String,
    version: AssetVersion,
}

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name:{} color:{} price:{} food:{} \n versions:{:?}",
            self.name, self.color, self.price, self.food, self.version
        )
    }
}
// TO DO.
// - connect to database .. ok but failure is not an Option, it's a Result!
// for now, returns Collection as an option (should be a result)
async fn db_connect() -> Option<mongodb::Collection<Asset>> {
    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await;

    match client {
        Ok(c) => {
            let database = c.database("gusfring");
            let collection: mongodb::Collection<Asset> = database.collection("food");
            Some(collection)
        }
        Err(_e) => None,
    }
}

#[tokio::main]
async fn main() {
    //-> Result<(), Box<dyn std::error::Error>> {

    // --- check args --- START
    let args = Args::parse();

    // get the insert args ("the asset to insert into the DB")
    let insert_str = args.insert.to_string();
    let insert_result: serde_json::Result<Insert> = serde_json::from_str(&insert_str);

    // let asset: Asset;
    let insert: Insert = match insert_result {
        Ok(a) => a,
        Err(r) => {
            print!("Err: bad json format: {} : {:?}", insert_str, r);
            panic!();
        }
    };

    if insert.id.is_none() && insert.name.is_none() {
        print!("Err: 'id' and 'name' are None: nothing to do");
        panic!();
    }
    // --- check args --- END

    // --- connect to DB --- START
    let collection = db_connect().await;

    // check collection is not empty
    let coll: Collection<Asset> = if collection.is_some() {
        collection.unwrap()
    } else {
        println!("Err: collection is None, nothing to do here.");
        panic!();
    };
    // --- connect to DB --- END

    // --- Create Asset --- START
    if insert.id.is_none() {
        // eg: tigershark -i
        // '{
        // "name":"my_3d_asset",
        // "source":"my_source_file.hip",
        // "datapath":"my/data/path"
        // }'

        let first_version: Bson = bson!({
            "version": 1,
            // "source":&insert.source,
            // "datapath:":&insert.datapath,
            "approved":false,
            "status":0,
        });
        // create Bson vector and convert to array
        let versions_array = Bson::Array(vec![first_version]);

        // let mut versions_vec: Vec<Bson> = Vec::new();
        // versions_vec.push(first_version);
        // let versions_array = bson::to_bson(&versions_vec).unwrap();

        // let new_asset = doc! {
        //     "name":insert.name.unwrap(),
        //     "latest_approved":0,
        //     "latest_version":1,
        //     "versions":versions_array,
        // };
        // let new_asset = Asset {
        //     "name":insert.name.unwrap(),
        //     "color":"red".to_owned(),
        //     "price":123,
        //     "food":"potato".to_owned(),
        // };
        //

        let ver = AssetVersion {
            version: 1 as u32,
            source: String::from("my_source_file"),
            approved: false,
            status: 0 as u32,
        };

        //
        //

        let new_asset = Asset {
            color: "red".to_owned(),
            name: insert.name.unwrap(),
            price: 123,
            food: "tomato".to_owned(),
            version: ver,
        };

        let insert_result = coll.insert_one(new_asset, None).await;

        match insert_result {
            Ok(i) => {
                // get id as a string
                let id: String = i.inserted_id.as_object_id().unwrap().to_hex();
                println!("{}", &id);
                exit(0); // exit nicely!
            }
            Err(e) => println!("Err: {:?}", e),
        }
    // --- Create Asset --- END
    // --- Update Asset --- START
    } else {
        // eg: tigershark -i '{"id":"6278a87db06a9874bfa44660"}'
        // check ID in the database for that asset

        let objid = ObjectId::parse_str(&insert.id.unwrap());
        if objid.is_err() {
            println!("id invalid: {:?}", &objid);
            panic!();
        }

        // find doc from ID
        let cursor = coll
            .find_one(Some(doc! { "_id": &objid.as_ref().unwrap() }), None)
            .await;

        match cursor {
            Ok(c) => {
                // println!("{:?}", c.unwrap());
                let asset: Asset = c.unwrap();
                println!("{}", asset);
                // println!("{} {} {} {}", a.name, a.color, a.price, a.food);
                // println!("{:?}", asset);
            }
            Err(c) => {
                println!("ID not found: {:?}", c);
                panic!();
            }
        }
    }
    // --- Update Asset --- END
}

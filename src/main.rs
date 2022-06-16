#![allow(dead_code, unused_variables, unused_imports)]

use clap::Parser;
use futures::stream::TryStreamExt;
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use mongodb::{Client, Collection};
use mongodb::{bson::doc, options::FindOptions};
use serde::{Deserialize, Serialize, Deserializer};
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
    id: Option<String>,
    version: Option<u32>,
    source: Option<String>,
}

// TO DO.
// - connect to database .. sortof done > failure is not an Option, it's a Result!
// - insert new document .. works, but using doc! macro, should be using a Struct instead?
// - check latest version .. can find documents from ID, need to create a version and increment
// before using a dict instead
// - increment version
// - add dictionaries for versions instead of u32


// for now, returns Collection as an option (should be a result)
async fn db_connect() -> Option<mongodb::Collection<Document>>{

    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await;

    match client{
        Ok(c) =>{
            let database = c.database("gusfring");
            let collection: mongodb::Collection<Document> = database.collection("chicken");
            Some(collection)
        },
        Err(e) => None
    }
}


#[tokio::main]
async fn main() {//-> Result<(), Box<dyn std::error::Error>> {
// fn main() {
    let args = Args::parse();

    // get the insert args ("the asset to insert into the DB")
    let asset_str = args.insert.to_string();

    let asset_result: serde_json::Result<Asset> = serde_json::from_str(&asset_str);

    // let asset: Asset;
    let asset:Asset = match asset_result {
        Ok(a) =>  a,
        Err(r) => {
            print!("Err: bad json format: {}", asset_str);
            panic!();
        }
    };

    if asset.id.is_none() && asset.name.is_none() {
        print!("Err: 'id' and 'name' are None: nothing to do");
        panic!();
    }

    let collection = db_connect().await;

    // check collection is not empty
    let coll:Collection<Document> = if collection.is_some(){
        collection.unwrap()
    }else{
        println!("Err: collection is None, nothing to do here.");
        panic!();
    };


    if asset.id.is_none() {

        //eg: tigershark -i '{"name":"jessie pinkman"}'

        let version:u32 = 1;
        let new_asset = doc!{
            "name":asset.name.unwrap(),
            "version":version,
        };

         let insert_result = coll.insert_one(new_asset, None).await;

         match insert_result {
             Ok(i) => {
                 // get id as a string
                 let id :String = i.inserted_id.as_object_id().unwrap().to_hex();
                 println!("{}",&id);
                 panic!();
                 // return ID and leave.
             },
             Err(e) => println!("Err: {:?}",e),
         }

    } else {
         // eg: tigershark -i '{"id":"6278a87db06a9874bfa44660"}'
         // check in the database for that asset
         //
         let objid = ObjectId::parse_str(&asset.id.unwrap());
         let objid_:ObjectId;
         if objid.is_ok(){

             let cursor = coll.find_one(Some(doc! { "_id": &objid.unwrap() }), None).await;
             match cursor{
                 Ok(c) => {
                     println!("document found: {:?}",c);
                     // TODO: need to find the latest version now ...
                 } ,
                 Err(c) => println!("id not found")
             }
         }else{
             print!("Obj ID not valid");
         }

    }
}

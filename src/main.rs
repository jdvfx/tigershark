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

#[derive(Debug, Serialize, Deserialize)]
struct DbInsert {
    name: String,
    version: u32,
}
// TO DO.
// - connect to database .. sortof done > failure is not an Option, it's a Result!
// - insert new document
// - check latest version .. can find documents from ID
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

    let collection = db_connect().await;

    // match collection{
    //     Some(c) => let collection = collection.unwrap(),
    //     None() => {
    //         print!("Err: collection issue");
    //         panic!();
    //     }
    // }
    // ugly TEMP thing...

    let coll:Collection<Document>;

    if collection.is_some(){
        coll = collection.unwrap();
    }else{
        println!("Err: collection is None");
        panic!();
    }



    // if collection.is_none(){
    //     print!("Err: collection Empty:nothing to do");
    // }else{
    //     let collection = collection.unwrap();
    // }
    // println!("DB connected: {:?}" , db.unwrap());

    if asset.id.is_none() {

        //eg: tigershark -i '{"name":"jessie pinkman"}'

        let version:u32 = 1;
        let new_asset = doc!{
            "name":asset.name.unwrap(),
            "version":version,
        };

         let insert_result = coll.insert_one(new_asset, None).await;
         let version = 0;

         match insert_result {
             Ok(i) => {
                 // of course, it could be easy but it ain't
                 let id :String = i.inserted_id.as_object_id().unwrap().to_hex();
                 println!("insert result: {:?}",&i);
                 println!("ID: {:?} ",&id);
             },
             Err(e) => println!("Err: {:?}",e),
         }

    } else {
       // check in the database for that asset
       //
         let objid = ObjectId::parse_str(&asset.id.unwrap());
         let objid_:ObjectId;
         if objid.is_ok(){

             let cursor = coll.find_one(Some(doc! { "_id": &objid.unwrap() }), None).await;
             match cursor{
                 Ok(c) => println!("{:?}",c),
                 Err(c) => println!("id not found")
             }

         }else{
             print!("Obj ID not valid");
         }


         // println!("objid : {:?}" , objid);
         // println!("{:?}",cursor);
       // asset ID not found

        // print!("Asset ID: {} not found", asset.id);
        // panic!();

        // asset ID found,
        // find the latest version.
    }
}

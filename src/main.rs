
// use mongodb::{
    // options::{ClientOptions, UpdateOptions},
//     Client,
// };
// use futures::stream::TryStreamExt;
// use serde_json::{Error, Result};
// use mongodb::{bson::doc, bson::Document, options::FindOptions};
// use serde::{Deserialize, Serialize, Deserializer};
// use mongodb::results::InsertOneResult;

use bson::{bson, Bson};
use std::process::exit;
use clap::Parser;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};
use mongodb::{bson::doc, bson::Document};
use serde::{Deserialize, Serialize};

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
    source: String,
}

// TO DO.
// - connect to database .. ok but failure is not an Option, it's a Result!


// for bson
#[derive(Debug, Serialize, Deserialize)]
struct AssetVersion {
    version: u32,
    source: String,
    approved: bool,
    status: u32,
}

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
        Err(_e) => None
    }
}


#[tokio::main]
async fn main() {//-> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    // get the insert args ("the asset to insert into the DB")
    let insert_str = args.insert.to_string();
    let insert_result: serde_json::Result<Insert> = serde_json::from_str(&insert_str);

    // let asset: Asset;
    let insert:Insert = match insert_result {
        Ok(a) =>  a,
        Err(r) => {
            print!("Err: bad json format: {} : {:?}", insert_str, r);
            panic!();
        }
    };


    if insert.id.is_none() && insert.name.is_none() {
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

    // let src = insert.source;

    if insert.id.is_none() {

        // eg: tigershark -i '{"name":"my_3d_asset","source":"my_source_file.hip"}'
        // source file is REQUIRED.

        let first_version: Bson = bson!({
            "version": 1,
            "source":&insert.source,
            "approved":false,
            "status":0,
        });
        // create Bson vector and convert to array
        let versions_array = Bson::Array(vec![first_version]);

        // let mut versions_vec: Vec<Bson> = Vec::new();
        // versions_vec.push(first_version);
        // let versions_array = bson::to_bson(&versions_vec).unwrap();

        let new_asset = doc!{
            "name":insert.name.unwrap(),
            "latest_approved":0,
            "latest_version":1,
            "versions":versions_array,
        };

         let insert_result = coll.insert_one(new_asset, None).await;

         match insert_result {
             Ok(i) => {
                 // get id as a string
                 let id :String = i.inserted_id.as_object_id().unwrap().to_hex();
                 println!("{}",&id);
                 exit(0); // exit nicely!
             },
             Err(e) => println!("Err: {:?}",e),
         }

    } else {
         // eg: tigershark -i '{"id":"6278a87db06a9874bfa44660"}'
         // check ID in the database for that asset

        let objid = ObjectId::parse_str(&insert.id.unwrap());
        if objid.is_err(){
            println!("id invalid: {:?}",&objid);
            panic!();
        }

        // find doc from ID
        let cursor = coll.find_one(Some(doc! { "_id": &objid.as_ref().unwrap() }), None).await;

        match cursor{
             Ok(c) => {

                let latest_version = c.as_ref().unwrap().get_i32("latest_version");
                if latest_version.is_err(){
                    println!("fail to parse 'latest_version'");
                    panic!();
                }

                let new_version_num = latest_version.unwrap() + 1;

                let new_version: Bson = bson!({
                    "version": new_version_num,
                    "source":&insert.source,
                    "approved":false,
                    "status":0,
                });

                let db_update_result = coll.update_one(
                  doc! { "_id": objid.as_ref().unwrap()},
                  doc! { "$push": { "versions": &new_version }, "$set": {"latest_version": &new_version_num  } },
                      None,
                  )
                  .await;
                    //
                match db_update_result{
                    Ok(r) => {
                        match r.matched_count{
                            0 => println!("ID not found, how is that even possible?! : {}", objid.as_ref().unwrap()),
                            _ => println!("updated {:?}",r),
                        }
                    },
                    Err(e) => println!("Error updating, something is really messed up: {:?}",e)
                }
             } ,
             Err(c) => {
                 println!("ID not found: {:?}",c);
                 panic!();
                }
        }

    }
}

#![allow(dead_code, unused_variables, unused_imports)]

use std::hash;

use mongodb::{
    options::{ClientOptions, UpdateOptions},
    Client,
};


use bson::{bson, Bson};
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::results::InsertOneResult;
use mongodb::bson::{doc, Document};
use mongodb:: options::FindOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;



#[derive(Debug, Serialize, Deserialize)]
struct AssetVersion {
    version: u32,
    source: String,
    approved: bool,
    status: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // create a version manually
    let bson_version: Bson = bson!({
        "version": 20,
        "source":"my_file_20.hip",
        "approved":false,
        "status":0,
    });


    // connect to DB and get collection (of balls?)
    let uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(uri).await?;
    let database = client.database("gusfring");
    let collection = database.collection::<Document>("balls");

    // create ObjectID from string
    let id_search= "62afef82ff5a83a76bf0c802";
    let id = ObjectId::parse_str(&id_search);

    // find doc from ID
    let cursor = collection.find_one(Some(doc! { "_id": &id.as_ref().unwrap() }), None).await;

    match cursor{
         Ok(c) => {
             // get versions arrat and push new version in
             let versions = c.as_ref().unwrap().get_array("versions").unwrap();
             let mut new_versions = versions.to_owned();
             new_versions.push(bson_version);

             // update the versions. --- there's probably a cleaner way of doing that no?
             // like adding a new element to an array directly, without copying everything back in
             // for now this is "working"
             // ...
            let db_update_result = collection.update_one(
              doc! { "_id": id.as_ref().unwrap()},
              doc! { "$set": { "versions": &new_versions } },
                  None,
              )
              .await;
            // did this stuff even work?
            match db_update_result{
                Ok(r) => {
                    // well that's dumb, I already know this ID exists from find_one
                    match r.matched_count{
                        0 => println!("ID not found, how is that even possible?! : {}", id.as_ref().unwrap()),
                        _ => println!("updated {:?}",r),
                    }
                },
                Err(e) => println!("Error updating, something is really messed up: {:?}",e)
            }
         } ,
         Err(c) => println!("ID not found")
    }

    Ok(())
}

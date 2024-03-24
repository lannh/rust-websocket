use futures_util::StreamExt;
use mongodb::{ 
    bson::{ doc,  Bson, Document},//Bson
    Client,
    Collection };

use futures::TryStreamExt;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterviewItem {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    _id: bson::oid::ObjectId,
    userId: String,
    interviewType: String,
    date: String,
    duration: u32,
    transcript: String
}

pub async fn getInterviewsDB(coll: Collection<InterviewItem>, userId: String) -> Vec<InterviewItem> {

    let filter = doc! {"userId": userId};

    let cursor = match coll.find(Some(filter), None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            println!("{e}");
            return vec![];
        },
    };

    return cursor.try_collect().await.unwrap();
    // cursor.try_collect().await.unwrap_or_else(|_| vec![])

}

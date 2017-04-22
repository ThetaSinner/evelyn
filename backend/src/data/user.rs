use bson;
use mongodb::db::ThreadedDatabase;

use model::user::{UserModel};
use core::error_messages::EvelynDatabaseError;
use mongodb::{Client, ThreadedClient};


pub fn insert_user(client : &Client, user_model: &UserModel) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("user");

    let bson_user_model = bson::to_bson(&user_model).unwrap();

    if let bson::Bson::Document(document) = bson_user_model {
      match collection.insert_one(document, None) {
          Ok(_) => None,
          Err(e) => Some(EvelynDatabaseError::InsertUser(e))
      }
    }
    else {
      Some(EvelynDatabaseError::SerialisationFailed)
    }
}

pub fn find_user(client : &Client, email_address: &String) -> Result<Option<UserModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("user");

    let query = doc!{"emailAddress" => email_address};
    let result = collection.find_one(Some(query), None);

    match result {
        Ok(r) => {
            if r.is_some() {
                Ok(bson::from_bson(bson::Bson::Document(r.unwrap())).unwrap())
            }
            else {
                Ok(None)
            }
        },
        Err(e) => {
            Err(EvelynDatabaseError::LookupUser(e))
        }
    }
}

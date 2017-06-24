// Evelyn: Your personal assistant, project manager and calendar
// Copyright (C) 2017 Gregory Jensen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use bson;
use bson::{Bson, Document};
use core::error_messages::{EvelynBaseError, EvelynDatabaseError};
use model::user::{UserModel, SearchResult};
use mongodb::{Client, ThreadedClient};
use mongodb::coll::options::FindOptions;
use mongodb::db::ThreadedDatabase;

pub fn insert_user(
    client: &Client,
    user_model: &UserModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("user");

    let bson_user_model = bson::to_bson(&user_model);

    match bson_user_model {
        Ok(x) =>  {
            if let bson::Bson::Document(document) = x {
                match collection.insert_one(document, None) {
                    Ok(_) => None,
                    Err(e) => Some(EvelynDatabaseError::InsertUser(e)),
                }
            } else {
                Some(EvelynDatabaseError::SerialisationFailed(EvelynBaseError::NothingElse))
            }
        },
        Err(e) => Some(EvelynDatabaseError::BSONEncodeFailed(e))
    }

}

pub fn find_user(
    client: &Client,
    email_address: &String,
) -> Result<Option<UserModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("user");

    let query = doc!{"emailAddress" => email_address};
    let result = collection.find_one(Some(query), None);

    match result {
        Ok(result_unwrap) => {
            match result_unwrap {
                Some(item) => {
                    match bson::from_bson(bson::Bson::Document(item)) {
                        Ok(bson_obj) => Ok(Some(bson_obj)),
                        Err(e) => Err(EvelynDatabaseError::BSONDecodeFailed(e))
                    }
                },
                // Todo: fix again
                None => Ok(None)
            }
        },
        Err(e) => Err(EvelynDatabaseError::LookupUser(e)),
    }
}

pub fn find_user_by_id(
    client: &Client,
    user_id: &String,
) -> Result<Option<UserModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("user");

    let query = doc!{"userId" => user_id};
    let result = collection.find_one(Some(query), None);

    match result {
        Ok(result_unwrap) => {
            match result_unwrap {
                Some(item) => {
                    match bson::from_bson(bson::Bson::Document(item)) {
                        Ok(bson_obj) => Ok(Some(bson_obj)),
                        Err(e) => Err(EvelynDatabaseError::BSONDecodeFailed(e))
                    }
                },
                // Todo: fix again
                None => Ok(None)
            }
        },
        Err(e) => Err(EvelynDatabaseError::LookupUser(e)),
    }
}

pub fn search_for_users(
    client: &Client,
    query: String,
) -> Result<Vec<SearchResult>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("user");

    let regex = doc!{"$regex" => query};
    let filter = doc!{"userName" => regex};

    let mut find_options = FindOptions::new();

    let mut projection = Document::new();
    projection.insert("userId", Bson::I32(1));
    projection.insert("userName", Bson::I32(1));
    projection.insert("_id", Bson::I32(0));
    find_options.projection = Some(projection);

    let cursor = collection.find(Some(filter), Some(find_options));

    match cursor {
        Ok(c) => {
            let results: Vec<SearchResult> = c.map(|x| match x {
               Ok(x) => bson::from_bson(bson::Bson::Document(x)).unwrap(),
               Err(e) => {
                   println!("Database error in search for users {}", e);
                   panic!()
               },
           }).collect();

           Ok(results)
        },
        Err(e) => Err(EvelynDatabaseError::SearchForUsers(e)),
    }
}

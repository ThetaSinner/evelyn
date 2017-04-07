/// Evelyn: Your personal assistant, project manager and calendar
/// Copyright (C) 2017 Gregory Jensen
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <http://www.gnu.org/licenses/>.

pub mod conf;

use bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use model;
use model::user::{UserModel};
use core::error_messages::EvelynDatabaseError;

pub struct MongoClient {
    client: Client
}

impl MongoClient {
    pub fn new<'a>(conf: &conf::Conf) -> Result<Self, &'a str> {
        let uri = conf.get_db_connnection_string();
        let client_result = Client::with_uri(uri.as_str());

        // Note that this will not fail if MongoDB is not available. There will only be an error
        // if the client cannot be initialised properly. Expect failure when sending commands to
        // the database.
        match client_result {
            Ok(client) => Ok(MongoClient{client : client}),
            Err(_) => Err("MongoDB driver failure")
        }
    }

    pub fn insert_user(&mut self, user_model: &UserModel) -> Option<EvelynDatabaseError> {
        let collection = self.client.db("evelyn").collection("user");

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

    pub fn find_user(&mut self, email_address: &String) -> Result<Option<UserModel>, EvelynDatabaseError> {
        let collection = self.client.db("evelyn").collection("user");

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

    pub fn insert_simple_task(&mut self, simple_task_model: &model::simple_task::SimpleTaskModel) -> Option<String> {
        let collection = self.client.db("evelyn").collection("simpletask");

        let bson_simple_task_model = bson::to_bson(&simple_task_model).unwrap();

        if let bson::Bson::Document(document) = bson_simple_task_model {
          match collection.insert_one(document, None) {
              Ok(_) => {None},
              Err(e) => {
                  println!("Database Error: Insert error {}", e);
                  Some(String::from("Failed to insert simple task"))
              }
          }
        } else {
          println!("Error converting the BSON object into a MongoDB document");
          Some(String::from("Error converting the BSON object into a MongoDB document"))
        }
    }

    pub fn lookup_simple_tasks(&mut self, simple_task_lookup_model: &model::simple_task::SimpleTaskLookupModel) -> Option<Vec<model::simple_task::SimpleTaskModel>> {
        let collection = self.client.db("evelyn").collection("simpletask");

        let ref user_id = simple_task_lookup_model.user_id;
        let query = doc!{"userId" => user_id};
        let cursor = collection.find(Some(query), None);

        match cursor {
            Ok(c) => {
                let docs: Vec<model::simple_task::SimpleTaskModel> = c
                    .map(|x| {
                        match x {
                            Ok(x) => {
                                bson::from_bson(bson::Bson::Document(x)).unwrap()
                            },
                            Err(e) => {
                                println!("Database error in lookup simple tasks {}", e);
                                panic!()
                            }
                        }
                    })
                    .collect();
                Some(docs)
            },
            Err(e) => {
                println!("Failed to lookup simple tasks {}", e);
                None
            }
        }
    }
}

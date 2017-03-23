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

use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use bson;

use model::UserModel;
use model;

pub struct MongoClient {
    client: Client
}

impl MongoClient {
    pub fn new<'a>() -> Result<Self, &'a str> {
        let client_result = Client::with_uri("mongodb://localhost:27017");

        match client_result {
            Ok(client) => Ok(MongoClient{client : client}),
            Err(_) => Err("Unable to connect to mongo")
        }
    }

    pub fn insert_user(&mut self, user_model: &UserModel) {
        let collection = self.client.db("evelyn").collection("user");

        let bson_user_model = bson::to_bson(&user_model).unwrap();

        if let bson::Bson::Document(document) = bson_user_model {
          match collection.insert_one(document, None) {
              Ok(_) => {},
              Err(e) => println!("Database Error: Insert error {}", e)
          }
        } else {
          println!("Error converting the BSON object into a MongoDB document");
        }
    }

    pub fn find_user(&mut self, email_address: &String) -> Option<UserModel> {
        let collection = self.client.db("evelyn").collection("user");

        let query = doc!{"emailAddress" => email_address};
        let result = collection.find_one(Some(query), None);

        match result {
            Ok(r) => {
                if r.is_some() {
                    Some(bson::from_bson(bson::Bson::Document(r.unwrap())).unwrap())
                }
                else {
                    None
                }
            },
            Err(e) => {
                println!("Failed to find user {}", e);
                None
            }
        }
    }

    pub fn insert_simple_task(&mut self, simple_task_model: &model::SimpleTaskModel) -> Option<String> {
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

    pub fn lookup_simple_tasks(&mut self, simple_task_lookup_model: &model::SimpleTaskLookupModel) -> Option<Vec<model::SimpleTaskModel>> {
        let collection = self.client.db("evelyn").collection("simpletask");

        let ref user_id = simple_task_lookup_model.user_id;
        let query = doc!{"userId" => user_id};
        let cursor = collection.find(Some(query), None);

        match cursor {
            Ok(c) => {
                let docs: Vec<model::SimpleTaskModel> = c
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

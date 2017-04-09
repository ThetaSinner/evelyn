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
use mongodb;
use bson::{Bson, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use model;
use model::user::{UserModel};
use core::error_messages::EvelynDatabaseError;

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

pub fn insert_simple_task(client : &Client, simple_task_model: &model::simple_task::SimpleTaskModel) -> Option<String> {
    let collection = client.db("evelyn").collection("simpletask");

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

pub fn lookup_simple_tasks(client : &Client, simple_task_lookup_model: &model::simple_task::SimpleTaskLookupModel) -> Option<Vec<model::simple_task::SimpleTaskModel>> {
    let collection = client.db("evelyn").collection("simpletask");

    let ref user_id = simple_task_lookup_model.user_id;
    let query = doc!{"userId" => user_id};
    let mut options = mongodb::coll::options::FindOptions::new();

    if simple_task_lookup_model.limit > 0 {
        options.limit = Some(simple_task_lookup_model.limit as i64);
    }

    let cursor = collection.find(Some(query), Some(options));

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

pub fn update_simple_task(client : &Client, simple_task_update_model: model::simple_task::SimpleTaskUpdateModel) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("simpletask");

    let ref user_id = simple_task_update_model.user_id;
    let ref task_id = simple_task_update_model.task_id;
    let filter = doc!("userId" => user_id, "taskId" => task_id);

    let mut update_query = Document::new();

    if simple_task_update_model.title.is_some() {
        update_query.insert("title", Bson::String(simple_task_update_model.title.unwrap()));
    }
    if simple_task_update_model.description.is_some() {
        update_query.insert("description", Bson::String(simple_task_update_model.description.unwrap()));
    }
    if simple_task_update_model.due_date.is_some() {
        update_query.insert("due_date", Bson::String(simple_task_update_model.due_date.unwrap()));
    }

    let mut set_update_query = Document::new();
    set_update_query.insert("$set", update_query);

    match collection.update_one(filter, set_update_query, None) {
        Ok(_) => {None},
        Err(e) => {
            println!("{}", e);
            Some(EvelynDatabaseError::UpdateSimpleTask(e))
        }
    }
}

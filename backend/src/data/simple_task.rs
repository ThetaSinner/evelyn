use bson;
use bson::{Bson, Document};
use mongodb::db::ThreadedDatabase;

use model;
use core::error_messages::EvelynDatabaseError;
use mongodb::{Client, ThreadedClient};


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
            Some(EvelynDatabaseError::UpdateSimpleTask(e))
        }
    }
}

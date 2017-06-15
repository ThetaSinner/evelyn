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
use model;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

pub fn insert_simple_task(
    client: &Client,
    simple_task_model: &model::simple_task::SimpleTaskModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("simpletask");

    let bson_simple_task_model = bson::to_bson(&simple_task_model).unwrap();

    if let bson::Bson::Document(document) = bson_simple_task_model {
        match collection.insert_one(document, None) {
            Ok(_) => None,
            Err(e) => Some(EvelynDatabaseError::InsertSimpleTask(e)),
        }
    } else {
        Some(EvelynDatabaseError::SerialisationFailed(EvelynBaseError::NothingElse))
    }
}

pub fn lookup_simple_tasks(
    client: &Client,
    simple_task_lookup_model: &model::simple_task::SimpleTaskLookupModel,
) -> Result<Vec<model::simple_task::SimpleTaskModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("simpletask");

    let ref user_id = simple_task_lookup_model.user_id;
    let query = doc!{"userId" => user_id};

    let cursor = collection.find(Some(query), None);

    match cursor {
        Ok(c) => {
            let docs: Vec<model::simple_task::SimpleTaskModel> = c.map(|x| match x {
                                                                           Ok(x) => bson::from_bson(bson::Bson::Document(x)).unwrap(),
                                                                           Err(e) => {
                                                                               println!("Database error in lookup simple tasks {}", e);
                                                                               panic!()
                                                                           },
                                                                       })
                .collect();
            Ok(docs)
        },
        Err(e) => Err(EvelynDatabaseError::LookupSimpleTask(e)),
    }
}

pub fn update_simple_task(
    client: &Client,
    simple_task_update_model: model::simple_task::SimpleTaskUpdateModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("simpletask");

    let ref user_id = simple_task_update_model.user_id;
    let ref task_id = simple_task_update_model.task_id;
    let filter = doc!("userId" => user_id, "taskId" => task_id);

    let mut update_query = Document::new();

    if simple_task_update_model.title.is_some() {
        update_query.insert("title",
                            Bson::String(simple_task_update_model.title.unwrap()));
    }
    if simple_task_update_model.description.is_some() {
        update_query.insert("description",
                            Bson::String(simple_task_update_model.description.unwrap()));
    }
    if simple_task_update_model.due_date.is_some() {
        update_query.insert("dueDate",
                            Bson::String(simple_task_update_model.due_date.unwrap()));
    }
    if simple_task_update_model.completed.is_some() {
        update_query.insert("completed",
                            Bson::Boolean(simple_task_update_model.completed.unwrap()));
    }

    let mut set_update_query = Document::new();
    set_update_query.insert("$set", update_query);

    match collection.update_one(filter, set_update_query, None) {
        Ok(_) => None,
        Err(e) => Some(EvelynDatabaseError::UpdateSimpleTask(e)),
    }
}

pub fn remove(
    client: &Client,
    task_id: String,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("simpletask");

    let filter = doc!{"taskId" => task_id};

    match collection.delete_one(filter, None) {
        Ok(_) => None,
        Err(e) => Some(EvelynDatabaseError::RemoveSimpleTask(e)),
    }
}

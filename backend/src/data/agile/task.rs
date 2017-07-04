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
use model::agile::task as task_model;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

pub fn insert_task(
    client: &Client,
    task_model: &task_model::TaskModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_task");

    insert_model!(
        collection,
        task_model,
        EvelynDatabaseError::InsertAgileTask
    )
}

pub fn find_task_by_id(
    client: &Client,
    task_id: &String,
) -> Result<Option<task_model::TaskModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_task");

    let query = doc!{"taskId" => task_id};
    let result = collection.find_one(Some(query), None);

    match result {
        Ok(r) => {
            if r.is_some() {
                Ok(bson::from_bson(bson::Bson::Document(r.unwrap())).unwrap())
            } else {
                // TODO fix me.
                Ok(None)
            }
        },
        Err(e) => Err(EvelynDatabaseError::LookupAgileTaskById(e)),
    }
}

pub fn lookup_backlog(
    client: &Client,
    project_id: &String,
    exclude_task_ids: &Vec<String>
) -> Result<Vec<task_model::TaskModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_task");

    let mut bson_exclude_task_ids = bson::Array::new();
    for id in exclude_task_ids {
        bson_exclude_task_ids.push(Bson::String(id.to_owned()));
    }

    let not_in_exclude_task_ids = doc!{"$nin" => bson_exclude_task_ids};

    let query = doc!{"projectId" => project_id, "taskId" => not_in_exclude_task_ids};

    match collection.find(Some(query), None) {
        Ok(cursor) => {
            Ok(cursor.map(|x| match x {
                Ok(x) => bson::from_bson(bson::Bson::Document(x)).unwrap(),
                Err(e) => {
                    println!("Database error in lookup backlog agile tasks {}", e);
                    panic!()
                },
            }).collect())
        },
        Err(e) => Err(EvelynDatabaseError::LookupBacklogAgileTasks(e)),
    }
}

pub fn update(
    client: &Client,
    update_model: task_model::UpdateTaskModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_task");

    let ref task_id = update_model.task_id;
    let filter = doc!("taskId" => task_id);

    let mut update_query = Document::new();

    update_query.insert("dateModified", Bson::I64(update_model.date_modified));
    update_query.insert("modifiedByUserId", Bson::String(update_model.modified_by_user_id));

    if update_model.title.is_some() {
        update_query.insert("title", Bson::String(update_model.title.unwrap()));
    }
    if update_model.description.is_some() {
        update_query.insert("description", Bson::String(update_model.description.unwrap()));
    }
    if update_model.original_estimate.is_some() {
        update_query.insert("originalEstimate", Bson::String(update_model.original_estimate.unwrap()));
    }
    if let Some(assignment) = update_model.assignment {
        let mut assignment_update_model = Document::new();
        assignment_update_model.insert("assignedToUserId", Bson::String(assignment.assigned_to_user_id));
        assignment_update_model.insert("assignedByUserId", Bson::String(assignment.assigned_by_user_id));

        update_query.insert("assignment", assignment_update_model);
    }

    let mut set_update_query = Document::new();
    set_update_query.insert("$set", update_query);

    match collection.update_one(filter, set_update_query, None) {
        Ok(_) => None,
        Err(e) => Some(EvelynDatabaseError::UpdateAgileTask(e)),
    }
}

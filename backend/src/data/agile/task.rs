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
    let collection = client.db("evelyn").collection("user");

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

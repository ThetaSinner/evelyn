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
use model::agile::sprint as sprint_model;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use chrono::prelude::*;



pub fn insert_sprint(
    client: &Client,
    sprint_model: &sprint_model::SprintModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_sprint");

    insert_model!(
        collection,
        sprint_model,
        EvelynDatabaseError::InsertAgileSprint
    )
}

pub fn find_active(
    client: &Client,
    project_ids: &Vec<String>,
) -> Result<Vec<sprint_model::SprintModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_sprint");

    let mut projects = bson::Array::new();
    for project in project_ids {
        projects.push(Bson::String(project.to_owned()));
    }

    let projects_in_query = doc!{"$in" => projects};

    let mut query = Document::new();
    query.insert("projectId", projects_in_query);

    let current_time = bson::Bson::I64(Utc::now().timestamp());
    let after_start_time = current_time.clone();
    query.insert("startDate", doc!{"$lte" => after_start_time});
    let before_end_time = current_time.clone();
    query.insert("endDate", doc!{"$gte" => before_end_time});

    debug!("query: {}", query);

    let cursor = collection.find(Some(query), None);

    match cursor {
        Ok(c) => {
            Ok(c.map(|x| match x {
                Ok(x) => bson::from_bson(bson::Bson::Document(x)).unwrap(),
                Err(e) => {
                    println!("Database error in lookup agile sprints {}", e);
                    panic!()
                },
            }).collect())
        },
        Err(e) => Err(EvelynDatabaseError::LookupAgileStories(e)),
    }
}

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
use bson::{Document};
use core::error_messages::{EvelynBaseError, EvelynDatabaseError};
use model::agile::project as project_model;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

pub fn insert_project(
    client: &Client,
    project_model: &project_model::ProjectModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_project");

    insert_model!(
        collection,
        project_model,
        EvelynDatabaseError::InsertAgileProject
    )
}

pub fn push_contributor(
    client: &Client,
    contributor_model: project_model::CreateContributorModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_project");

    let ref project_id = contributor_model.project_id;
    let filter = doc!("projectId" => project_id);

    let mut update_query = Document::new();
    let bson_contributor_model = bson::to_bson(&contributor_model.contributor).unwrap();
    if let bson::Bson::Document(document) = bson_contributor_model {
        update_query.insert("contributors", document);

        let mut push_update_query = Document::new();
        push_update_query.insert("$addToSet", update_query);

        match collection.update_one(filter, push_update_query, None) {
            Ok(_) => None,
            Err(e) => Some(EvelynDatabaseError::AddContributorToAgileProject(e)),
        }
    } else {
        Some(EvelynDatabaseError::SerialisationFailed(EvelynBaseError::NothingElse))
    }
}

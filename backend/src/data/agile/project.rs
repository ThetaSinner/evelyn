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
use model::agile::project as project_model;
use mongodb::{Client, ThreadedClient};
use mongodb::coll::options::FindOptions;
use mongodb::db::ThreadedDatabase;

fn build_project_lookup_filter(
    user_id: &String,
    user_groups: Vec<model::user_group::UserGroupsExternalModel>,
) -> Document {
    let ref _user_id = user_id;

    let mut created_by_filter = Document::new();
    created_by_filter.insert("createdByUserId", *_user_id);

    let mut user_id_filter = Document::new();
    user_id_filter.insert("userContributors.userId", *_user_id);

    let mut group_ids = bson::Array::new();
    for group in user_groups {
        group_ids.push(Bson::String(group.user_group_id));
    }

    let mut user_group_in_filter = Document::new();
    user_group_in_filter.insert("$in", group_ids);

    let mut user_group_filter = Document::new();
    user_group_filter.insert("userGroupContributors.userGroupId", user_group_in_filter);

    let mut arr = bson::Array::new();
    arr.push(bson::to_bson(&created_by_filter).unwrap());
    arr.push(bson::to_bson(&user_id_filter).unwrap());
    arr.push(bson::to_bson(&user_group_filter).unwrap());

    let mut filter = Document::new();
    filter.insert("$or", Bson::Array(arr));

    filter
}

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

pub fn add_user_contributor(
    client: &Client,
    user_contributor_model: project_model::AddUserContributorModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_project");

    let ref project_id = user_contributor_model.project_id;
    let filter = doc!("projectId" => project_id);

    let mut update_query = Document::new();
    let bson_user_contributor_model = bson::to_bson(&user_contributor_model.user_contributor).unwrap();
    if let bson::Bson::Document(document) = bson_user_contributor_model {
        update_query.insert("userContributors", document);

        let mut push_update_query = Document::new();
        push_update_query.insert("$addToSet", update_query);

        match collection.update_one(filter, push_update_query, None) {
            Ok(_) => None,
            Err(e) => Some(EvelynDatabaseError::AddUserContributorToAgileProject(e)),
        }
    } else {
        Some(EvelynDatabaseError::SerialisationFailed(EvelynBaseError::NothingElse))
    }
}

pub fn add_user_group_contributor(
    client: &Client,
    user_group_contributor_model: project_model::AddUserGroupContributorModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_project");

    let ref project_id = user_group_contributor_model.project_id;
    let filter = doc!("projectId" => project_id);

    let mut update_query = Document::new();
    let bson_user_group_contributor_model = bson::to_bson(&user_group_contributor_model.user_group_contributor).unwrap();
    if let bson::Bson::Document(document) = bson_user_group_contributor_model {
        update_query.insert("userGroupContributors", document);

        let mut push_update_query = Document::new();
        push_update_query.insert("$addToSet", update_query);

        match collection.update_one(filter, push_update_query, None) {
            Ok(_) => None,
            Err(e) => Some(EvelynDatabaseError::AddUserGroupContributorToAgileProject(e)),
        }
    } else {
        Some(EvelynDatabaseError::SerialisationFailed(EvelynBaseError::NothingElse))
    }
}

pub fn lookup_contributing_to(
    client: &Client,
    user_id: &String,
    user_groups: Vec<model::user_group::UserGroupsExternalModel>,
) -> Result<Vec<project_model::ProjectPreviewModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_project");

    let filter = build_project_lookup_filter(user_id, user_groups);

    let mut find_options = FindOptions::new();

    let mut projection = Document::new();
    projection.insert("projectId", Bson::I32(1));
    projection.insert("name", Bson::I32(1));
    projection.insert("shortName", Bson::I32(1));
    projection.insert("description", Bson::I32(1));
    projection.insert("_id", Bson::I32(0));
    find_options.projection = Some(projection);

    let cursor = collection.find(Some(filter), Some(find_options));

    match cursor {
        Ok(cursor) => {
            Ok(cursor
                   .map(|x| {
                match x {
                    Ok(x) => {
                        debug!("{}", x);
                        bson::from_bson(bson::Bson::Document(x)).unwrap()
                    },
                    Err(e) => {
                        error!("Database error in lookup agile projects {}", e);
                        panic!() // need a better way to handle this ideally.
                    },
                }
            }).collect())
        },
        Err(e) => Err(EvelynDatabaseError::LookupContributingToAgileProjects(e)),
    }
}

pub fn lookup(
    client: &Client,
    project_id: &String,
    user_id: &String,
    user_groups: Vec<model::user_group::UserGroupsExternalModel>,
) -> Result<project_model::ProjectModel, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("agile_project");

    let mut filter = build_project_lookup_filter(user_id, user_groups);
    filter.insert("projectId", project_id);

    match collection.find_one(Some(filter), None) {
        Ok(result) => {
            if let Some(result) = result {
                Ok(bson::from_bson(bson::Bson::Document(result)).unwrap())
            }
            else {
                Err(EvelynDatabaseError::AgileProjectNotFound(EvelynBaseError::NothingElse))
            }
        },
        Err(e) => Err(EvelynDatabaseError::LookupAgileProject(e)),
    }
}

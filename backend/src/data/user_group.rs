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
use model::user_group as user_group_model;
use mongodb::{Client, ThreadedClient};
use mongodb::coll::options::FindOptions;
use mongodb::db::ThreadedDatabase;

fn build_user_group_lookup_filter(user_id: &String) -> Document {
      let mut created_by_filter = Document::new();
    created_by_filter.insert("createdByUserId", user_id);

    let mut member_filter = Document::new();
    member_filter.insert("members.userId", user_id);

    let mut arr = bson::Array::new();
    arr.push(bson::to_bson(&created_by_filter).unwrap());
    arr.push(bson::to_bson(&member_filter).unwrap());

    let mut filter = Document::new();
    filter.insert("$or", Bson::Array(arr));

    filter
}

pub fn insert_user_group(
    client: &Client,
    user_group_model: &user_group_model::UserGroupModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("usergroup");

    insert_model!(collection,
                  user_group_model,
                  EvelynDatabaseError::InsertUserGroup)
}

pub fn lookup_user_groups(
    user_id: &String,
    client: &Client,
) -> Result<Vec<user_group_model::UserGroupsModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("usergroup");

    let filter = build_user_group_lookup_filter(user_id);

    let mut find_options = FindOptions::new();

    let mut projection = Document::new();
    projection.insert("userGroupId", Bson::I32(1));
    projection.insert("name", Bson::I32(1));
    projection.insert("description", Bson::I32(1));
    projection.insert("_id", Bson::I32(0));
    find_options.projection = Some(projection);

    let cursor = collection.find(Some(filter), Some(find_options));

    match cursor {
        Ok(cursor) => {
            Ok(cursor
                   .map(|x| {
                match x {
                    Ok(x) => bson::from_bson(bson::Bson::Document(x)).unwrap(),
                    Err(e) => {
                        println!("Database error in lookup user groups {}", e);
                        panic!() // need a better way to handle this ideally.
                    },
                }
            })
                   .collect())
        },
        Err(e) => Err(EvelynDatabaseError::LookupUserGroups(e)),
    }
}

pub fn lookup_user_group(
    client: &Client,
    user_id: &String,
    user_group_id: &String,
) -> Result<user_group_model::UserGroupModel, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("usergroup");

    let mut filter = build_user_group_lookup_filter(user_id);
    filter.insert("userGroupId", user_group_id);

    match collection.find_one(Some(filter), None) {
        Ok(result) => {
            if let Some(result) = result {
                Ok(bson::from_bson(bson::Bson::Document(result)).unwrap())
            } else {
                Err(EvelynDatabaseError::UserGroupNotFound(EvelynBaseError::NothingElse))
            }
        },
        Err(e) => Err(EvelynDatabaseError::LookupUserGroup(e)),
    }
}

pub fn add_member(
    client: &Client,
    add_member_model: user_group_model::member::AddMemberModel,
) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("usergroup");

    let ref user_group_id = add_member_model.user_group_id;
    let filter = doc!("userGroupId" => user_group_id);

    let mut update_query = Document::new();
    let bson_member_model = bson::to_bson(&add_member_model.user_group_member_model).unwrap();
    if let bson::Bson::Document(document) = bson_member_model {
        update_query.insert("members", document);

        let mut push_update_query = Document::new();
        push_update_query.insert("$addToSet", update_query);

        match collection.update_one(filter, push_update_query, None) {
            Ok(_) => None,
            Err(e) => Some(EvelynDatabaseError::AddMemberToUserGroup(e)),
        }
    } else {
        Some(EvelynDatabaseError::SerialisationFailed(EvelynBaseError::NothingElse))
    }
}

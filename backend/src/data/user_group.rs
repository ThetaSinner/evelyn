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

use bson;
use bson::{Bson, Document};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::FindOptions;

use model::user_group as user_group_model;
use core::error_messages::{EvelynDatabaseError, EvelynBaseError};
use mongodb::{Client, ThreadedClient};

pub fn insert_user_group(client : &Client, user_group_model: &user_group_model::UserGroupModel) -> Option<EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("usergroup");

    insert_model!(collection, user_group_model, EvelynDatabaseError::InsertUserGroup)
}

pub fn lookup_user_groups(client : &Client) -> Result<Vec<user_group_model::UserGroupsModel>, EvelynDatabaseError> {
    let collection = client.db("evelyn").collection("usergroup");

    let mut find_options = FindOptions::new();

    let mut projection = Document::new();
    projection.insert("userGroupId", Bson::I32(1));
    projection.insert("name", Bson::I32(1));
    projection.insert("description", Bson::I32(1));
    projection.insert("_id", Bson::I32(0));
    find_options.projection = Some(projection);

    let cursor = collection.find(Some(Document::new()), Some(find_options));

    match cursor {
        Ok(cursor) => {
            Ok(cursor.map(|x| {
                match x {
                    Ok(x) => {
                        bson::from_bson(bson::Bson::Document(x)).unwrap()
                    },
                    Err(e) => {
                        println!("Database error in lookup user groups {}", e);
                        panic!() // need a better way to handle this ideally.
                    }
                }
            }).collect())
        },
        Err(e) => {
            Err(EvelynDatabaseError::LookupUserGroups(e))
        }
    }
}

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
use mongodb::db::ThreadedDatabase;

use model::user::{UserModel};
use core::error_messages::EvelynDatabaseError;
use mongodb::{Client, ThreadedClient};

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
                // TODO fix me.
                Ok(None)
            }
        },
        Err(e) => {
            Err(EvelynDatabaseError::LookupUser(e))
        }
    }
}

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

use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use bson;

use model::UserModel;

pub struct MongoClient {
    client: Client
}

impl MongoClient {
    pub fn new<'a>() -> Result<Self, &'a str> {
        let client_result = Client::with_uri("mongodb://localhost:27017");

        match client_result {
            Ok(client) => Ok(MongoClient{client : client}),
            Err(_) => Err("Unable to connect to mongo")
        }
    }

    pub fn insert_user(&mut self, user_model: &UserModel) {
        let collection = self.client.db("evelyn").collection("user");

        let bson_user_model = bson::to_bson(&user_model).unwrap();

        if let bson::Bson::Document(document) = bson_user_model {
          match collection.insert_one(document, None) {
              Ok(_) => {},
              Err(e) => println!("Database Error: Insert error {}", e)
          }
        } else {
          println!("Error converting the BSON object into a MongoDB document");
        }
    }

    pub fn find_user(&mut self, email_address: &String) -> Option<UserModel> {
        let collection = self.client.db("evelyn").collection("user");

        let query = doc!{"emailAddress" => email_address};
        let result = collection.find_one(Some(query), None).unwrap().unwrap();

        Some(bson::from_bson(bson::Bson::Document(result)).unwrap())
    }
}

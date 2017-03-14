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

// use mongodb::{Client, ThreadedClient};

use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

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

    pub fn test(&mut self) {
        let collection = self.client.db("test").collection("testc");

        collection.insert_one(doc!{"test key" => "test value"}, None).unwrap();
    }
}

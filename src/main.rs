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

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

#[macro_use]
extern crate serde_json;

mod server;
mod data;

use serde_json::Value;

use server::router;

fn test_processor() {

}

fn main() {
  println!("Hello, World!");

  let mut client = data::MongoClient::new().unwrap();
  // client.test();

  let mut router = router::Router::new();
  router.post("hello/world", test_processor);

  let data = r#"{"name":"John Doe", "age": 43}"#;
  let v: Value = serde_json::from_str(data).unwrap();
  // let () = v["name"].as_str().unwrap();
  println!("Hello, I'm {}, and I'm {} years old {}", v["name"].as_str().unwrap(), v["age"], "some data");

  server::start();
}

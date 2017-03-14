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

#[macro_use]
extern crate serde_derive;

mod server;
mod data;

use serde_json::Value;

use server::http::HttpServer;
use server::routing::{Router, RouterInput, RouterOutput};

#[derive(Serialize, Deserialize)]
struct TestModel {
    name: Option<String>,
    hello: String,
}

fn test_processor(router_input: RouterInput) -> RouterOutput {
  println!("Test processor running");

  let request_model_de: Result<TestModel,_> = serde_json::from_str(&router_input.request_body);
  match request_model_de {
    Ok(request_model) => {
      println!("Test processor got : {}, {:?}", request_model.hello, request_model.name);
    },
    Err(e) => {
      println!("Bad payload");
    }
  }

  RouterOutput{response_body: "not implemented".to_string()}
}

fn main() {
  println!("Hello, World!");

  let mut client = data::MongoClient::new().unwrap();
  // the above doesn't handle errors, but the code below prevents a lot of warnings!! :)
  // client.test();

  let data = r#"{"name":"John Doe", "age": 43}"#;
  let v: Value = serde_json::from_str(data).unwrap();
  // let () = v["name"].as_str().unwrap();
  println!("Hello, I'm {}, and I'm {} years old {}", v["name"].as_str().unwrap(), v["age"], "some data");

  let mut router = Router::new();
  router.add_rule("/hello/world", test_processor);

  let http_server = HttpServer::new(router);
  http_server.start();
}

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

use serde_json;
use std::sync::Arc;
use std::sync::Mutex;

use server::routing::{Router, RouterInput, RouterOutput};
use model;
use data::MongoClient;
use core::user;

pub struct ProcessorData {
  pub data_store: Arc<Mutex<MongoClient>>,
}

pub fn load_processors(router: &mut Router) {
  router.add_rule("/hello/world", test_processor);
  router.add_rule("/user/create", create_user_processor);
}

fn test_processor(router_input: RouterInput, processor_data: Arc<ProcessorData>) -> RouterOutput {
  println!("Test processor running");

  let request_model_de: Result<model::TestModel,_> = serde_json::from_str(&router_input.request_body);
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

fn create_user_processor(router_input: RouterInput, processor_data: Arc<ProcessorData>) -> RouterOutput {
  let request_model_de: Result<model::CreateUserModel,_> = serde_json::from_str(&router_input.request_body);
  let result = "Processor error";
  match request_model_de {
    Ok(request_model) => {
      user::create_user(request_model, processor_data);
    },
    Err(e) => {
      println!("Bad payload");
    }
  }

  RouterOutput{response_body: "not implemented".to_string()}
}

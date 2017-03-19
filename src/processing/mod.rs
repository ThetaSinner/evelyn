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
use core::token_service::TokenService;

pub struct ProcessorData {
  pub data_store: Arc<Mutex<MongoClient>>,
  pub token_service: TokenService,
}

pub fn load_processors(router: &mut Router) {
  router.add_rule("/user/create", create_user_processor);
  router.add_rule("/user/logon", logon_user_processor);
}

fn create_user_processor(router_input: RouterInput, processor_data: Arc<ProcessorData>) -> RouterOutput {
  let request_model_de: Result<model::CreateUserModel,_> = serde_json::from_str(&router_input.request_body);
  let mut result: model::CreateUserResponseModel = model::CreateUserResponseModel{
      error: Some(model::ErrorModel{
          error_code: "_".to_owned(),
          error_message: "Create user processor failed".to_owned()
      })
  };
  match request_model_de {
    Ok(request_model) => {
      result = user::create_user(request_model, processor_data);
    },
    Err(e) => {
      println!("Bad payload, {}", e);
    }
  }

  RouterOutput{response_body: serde_json::to_string(&result).unwrap()}
}

fn logon_user_processor(router_input: RouterInput, processor_data: Arc<ProcessorData>) -> RouterOutput {
    let request_model_de: Result<model::LogonUserModel,_> = serde_json::from_str(&router_input.request_body);

    match request_model_de {
      Ok(request_model) => {
        let response = user::logon_user(request_model, processor_data);
        RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
      },
      Err(e) => {
        println!("Bad payload, {}", e);

        let response = model::LogonUserResponseModel {
            token: None,
            error: Some(model::ErrorModel {
                error_code: "101002".to_owned(),
                error_message: "Failed to process user logon".to_owned()
            })
        };

        RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
      }
    }
}

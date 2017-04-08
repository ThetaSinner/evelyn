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

use std::sync::Arc;
use std::sync::Mutex;

use serde_json;

use server::routing::{Router, RouterInput, RouterOutput};
use model;
use data::MongoClient;
use data::conf;
use core::{user, simple_task};
use core::token_service::TokenService;
use core::error_messages::{EvelynServiceError, EvelynCoreError};

pub struct ProcessorData {
  pub data_store: Arc<Mutex<MongoClient>>,
  pub token_service: TokenService,
  pub conf: conf::Conf,
}

pub fn load_processors(router: &mut Router) {
  router.add_rule("/user/create", create_user_processor);
  router.add_rule("/user/logon", logon_user_processor);

  router.add_rule("/simpletask/create", create_simple_task_processor);
  router.add_rule("/simpletask/lookup", lookup_simple_task_processor);
}

fn create_user_processor(router_input: RouterInput, processor_data: Arc<ProcessorData>) -> RouterOutput {
  let request_model_decoded: Result<model::user::CreateUserModel,_> = serde_json::from_str(&router_input.request_body);

  match request_model_decoded {
    Ok(request_model) => {
        match user::create_user(request_model, processor_data) {
            None => {
                RouterOutput{response_body: serde_json::to_string(&model::user::CreateUserResponseModel {error: None}).unwrap()}
            },
            Some(e) => {
                match e {
                    EvelynCoreError::WillNotCreateUserBecauseUserAlreadyExists => {
                        RouterOutput{response_body: serde_json::to_string(&model::user::CreateUserResponseModel {
                            error: Some(From::from(EvelynServiceError::UserAlreadyExists(e)))
                        }).unwrap()}
                    },
                    _ => {
                        RouterOutput{response_body: serde_json::to_string(&model::user::CreateUserResponseModel {
                            error: Some(From::from(EvelynServiceError::CreateUser(e)))
                        }).unwrap()}
                    },
                }
            },
        }
    },
    Err(e) => {
        let model: model::ErrorModel = From::from(EvelynServiceError::CouldNotDecodeTheRequestPayload(e));
        RouterOutput {
            response_body: serde_json::to_string(&model::user::CreateUserResponseModel {
                error: Some(model),
            }).unwrap()
        }
    }
  }
}

fn logon_user_processor(router_input: RouterInput, processor_data: Arc<ProcessorData>) -> RouterOutput {
    let request_model_de: Result<model::user::LogonUserModel,_> = serde_json::from_str(&router_input.request_body);

    match request_model_de {
      Ok(request_model) => {
        match user::logon_user(request_model, processor_data) {
            Ok(response) => {
                RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
            },
            Err(e) => {
                match e {
                    EvelynCoreError::InvalidLogon => {
                        RouterOutput{response_body: serde_json::to_string(&model::user::LogonUserResponseModel {
                            token: None,
                            error: Some(From::from(EvelynServiceError::LogonUser(e)))
                        }).unwrap()}
                    },
                    _ => {
                        RouterOutput{response_body: serde_json::to_string(&model::user::LogonUserResponseModel {
                            token: None,
                            error: Some(From::from(EvelynServiceError::FailedToLogonUser(e)))
                        }).unwrap()}
                    },
                }
            },
        }
      },
      Err(e) => {
        RouterOutput{response_body: serde_json::to_string(&model::user::LogonUserResponseModel {
            token: None,
            error: Some(From::from(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)))
        }).unwrap()}
      }
    }
}

fn create_simple_task_processor(router_input: RouterInput, processor_data: Arc<ProcessorData>) -> RouterOutput {
    let request_model_de: Result<model::simple_task::CreateSimpleTaskModel,_> = serde_json::from_str(&router_input.request_body);

    match request_model_de {
      Ok(request_model) => {
        let response = simple_task::create_simple_task(request_model, processor_data);
        RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
      },
      Err(e) => {
        println!("Bad payload, {}", e);

        let response = model::simple_task::CreateSimpleTaskResponseModel {
            error: Some(model::ErrorModel {
                error_code: "102001".to_owned(),
                error_message: "Failed to process create simple task".to_owned()
            })
        };

        RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
      }
    }
}

fn lookup_simple_task_processor(router_input: RouterInput, processor_data: Arc<ProcessorData>) -> RouterOutput {
    let request_model_de: Result<model::simple_task::LookupSimpleTaskRequestModel,_> = serde_json::from_str(&router_input.request_body);

    match request_model_de {
      Ok(request_model) => {
        let response = simple_task::lookup_simple_tasks(request_model, processor_data);
        RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
      },
      Err(e) => {
        println!("Bad payload, {}", e);

        let response = model::simple_task::CreateSimpleTaskResponseModel {
            error: Some(model::ErrorModel {
                error_code: "102001".to_owned(),
                error_message: "Failed to process create simple task".to_owned()
            })
        };

        RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
      }
    }
}
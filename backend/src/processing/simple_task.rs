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

use serde_json;

use server::routing::{RouterInput, RouterOutput};
use model;
use processing;
use core::simple_task;
use core::error_messages::EvelynServiceError;


pub fn create_simple_task_processor(router_input: RouterInput, processor_data: Arc<processing::ProcessorData>) -> RouterOutput {
    let request_model_de: Result<model::simple_task::CreateSimpleTaskModel,_> = serde_json::from_str(&router_input.request_body);

    match request_model_de {
      Ok(request_model) => {
        match simple_task::create_simple_task(request_model, processor_data) {
            Ok(response) => {
                RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
            }
            Err(e) => {
                RouterOutput{
                    response_body: serde_json::to_string(&model::simple_task::CreateSimpleTaskResponseModel {
                        tasks: None,
                        error: Some(From::from(EvelynServiceError::FailedToCreateSimpleTask(e))),
                    }).unwrap()
                }
            }
        }
      },
      Err(e) => {
        let response = model::simple_task::LookupSimpleTaskResponseModel {
            tasks: None,
            error: Some(From::from(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)))
        };

        RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
      }
    }
}

pub fn lookup_simple_task_processor(router_input: RouterInput, processor_data: Arc<processing::ProcessorData>) -> RouterOutput {
    let request_model_de: Result<model::simple_task::LookupSimpleTaskRequestModel,_> = serde_json::from_str(&router_input.request_body);

    match request_model_de {
      Ok(request_model) => {
        match simple_task::lookup_simple_tasks(request_model, processor_data) {
            Ok(response) => {
                RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
            }
            Err(e) => {
                RouterOutput{
                    response_body: serde_json::to_string(&model::simple_task::LookupSimpleTaskResponseModel {
                        tasks: None,
                        error: Some(From::from(EvelynServiceError::FailedToLookupSimpleTask(e))),
                    }).unwrap()
                }
            }
        }
      },
      Err(e) => {
        let response = model::simple_task::LookupSimpleTaskResponseModel {
            tasks: None,
            error: Some(From::from(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)))
        };

        RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
      }
    }
}

pub fn update_simple_task_processor(router_input: RouterInput, processor_data: Arc<processing::ProcessorData>) -> RouterOutput {
    let request_model_de: Result<model::simple_task::UpdateSimpleTaskRequestModel,_> = serde_json::from_str(&router_input.request_body);

    match request_model_de {
      Ok(request_model) => {
        match simple_task::update_simple_task(request_model, processor_data) {
            None => {
                RouterOutput{response_body: serde_json::to_string(&model::simple_task::UpdateSimpleTaskResponseModel {
                    error: None,
                }).unwrap()}
            },
            Some(e) => {
                let model: model::ErrorModel = From::from(EvelynServiceError::FailedToUpdateSimpleTask(e));
                RouterOutput {
                    response_body: serde_json::to_string(&model::simple_task::UpdateSimpleTaskResponseModel {
                        error: Some(model),
                    }).unwrap()
                }
            }
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

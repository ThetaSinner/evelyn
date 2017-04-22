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
use core::todo_list;
use core::error_messages::EvelynServiceError;

pub fn create_todo_list_processor(router_input: RouterInput, processor_data: Arc<processing::ProcessorData>) -> RouterOutput {
  let request_model_decoded: Result<model::todo_list::CreateTodoListRequestModel,_> = serde_json::from_str(&router_input.request_body);

  match request_model_decoded {
    Ok(request_model) => {
        match todo_list::create_todo_list(request_model, processor_data) {
            Ok(response) => {
                RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
            },
            Err(e) => {
                RouterOutput{
                    response_body: serde_json::to_string(&model::todo_list::CreateTodoListResponseModel {
                        todo_list_id: None,
                        error: Some(From::from(EvelynServiceError::CreateTodoList(e))),
                    }).unwrap()
                }
            },
        }
    },
    Err(e) => {
        let model: model::ErrorModel = From::from(EvelynServiceError::CouldNotDecodeTheRequestPayload(e));
        RouterOutput {
            response_body: serde_json::to_string(&model::todo_list::CreateTodoListResponseModel {
                todo_list_id: None,
                error: Some(model),
            }).unwrap()
        }
    }
  }
}

pub fn add_item_todo_list_processor(router_input: RouterInput, processor_data: Arc<processing::ProcessorData>) -> RouterOutput {
  let request_model_decoded: Result<model::todo_list::AddItemTodoListRequestModel,_> = serde_json::from_str(&router_input.request_body);

  match request_model_decoded {
    Ok(request_model) => {
        match todo_list::add_item_to_todo_list(request_model, processor_data) {
            Some(e) => {
                RouterOutput{
                    response_body: serde_json::to_string(&model::todo_list::AddItemTodoListResponseModel {
                        error: Some(From::from(EvelynServiceError::AddItemToTodoList(e))),
                    }).unwrap()
                }
            },
            None => {
                RouterOutput{
                    response_body: serde_json::to_string(&model::todo_list::AddItemTodoListResponseModel {
                        error: None,
                    }).unwrap()
                }
            },
        }
    },
    Err(e) => {
        let model: model::ErrorModel = From::from(EvelynServiceError::CouldNotDecodeTheRequestPayload(e));
        RouterOutput {
            response_body: serde_json::to_string(&model::todo_list::AddItemTodoListResponseModel {
                error: Some(model),
            }).unwrap()
        }
    }
  }
}

pub fn lookup_todo_lists_processor(router_input: RouterInput, processor_data: Arc<processing::ProcessorData>) -> RouterOutput {
  let request_model_decoded: Result<model::todo_list::LookupTodoListsRequestModel,_> = serde_json::from_str(&router_input.request_body);

  match request_model_decoded {
    Ok(request_model) => {
        match todo_list::lookup_todo_lists(request_model, processor_data) {
            Ok(result) => {
                RouterOutput{response_body: serde_json::to_string(&result).unwrap()}
            },
            Err(e) => {
                RouterOutput{
                    response_body: serde_json::to_string(&model::todo_list::LookupTodoListsResponseModel {
                        todo_lists: None,
                        error: Some(From::from(EvelynServiceError::LookupTodoLists(e))),
                    }).unwrap()
                }
            },
        }
    },
    Err(e) => {
        let model: model::ErrorModel = From::from(EvelynServiceError::CouldNotDecodeTheRequestPayload(e));
        RouterOutput {
            response_body: serde_json::to_string(&model::todo_list::AddItemTodoListResponseModel {
                error: Some(model),
            }).unwrap()
        }
    }
  }
}

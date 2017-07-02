// Evelyn: Your personal assistant, project manager and calendar
// Copyright (C) 2017 Gregory Jensen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use core::error_messages::{EvelynServiceError, EvelynBaseError};
use model;
use model::agile::task as task_model;
use core::agile::task;
use processing;
use serde_json;
use server::routing::{RouterInput, RouterOutput};
use std::sync::Arc;

pub fn create_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(task_model::CreateTaskRequestModel, router_input) {
        Ok(request_model) => {
            let session_token_model = validate_session!(processor_data, request_model);

            match task::create(request_model, session_token_model, processor_data) {
                Ok(response) => {
                    model_to_router_output!(response)
                },
                Err(e) => {
                    model_to_router_output!(task_model::CreateTaskResponseModel {
                        task_id: None,
                        error: service_error_to_model!(EvelynServiceError::CreateAgileTask(e)),
                    })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(task_model::CreateTaskResponseModel {
                task_id: None,
                error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
            })
        },
    }
}

pub fn lookup_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(task_model::LookupTaskRequestModel, router_input) {
        Ok(request_model) => {
            validate_session!(processor_data, request_model);

            match task::lookup(request_model, processor_data) {
                Ok(response) => {
                    model_to_router_output!(response)
                },
                Err(e) => {
                    model_to_router_output!(task_model::LookupTaskResponseModel {
                        task: None,
                        error: service_error_to_model!(EvelynServiceError::LookupAgileTask(e)),
                    })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(task_model::LookupTaskResponseModel {
                task: None,
                error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
            })
        },
    }
}

pub fn update_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    match decode_router_input_to_model!(task_model::UpdateTaskRequestModel, router_input) {
        Ok(request_model) => {
            let session_token_model = validate_session!(processor_data, request_model);

            match task::update(request_model, session_token_model, processor_data) {
                None => {
                    model_to_router_output!(task_model::UpdateTaskResponseModel {
                        error: None,    
                    })
                },
                Some(e) => {
                    model_to_router_output!(task_model::UpdateTaskResponseModel {
                        error: service_error_to_model!(EvelynServiceError::UpdateAgileTask(e)),
                    })
                },
            }
        },
        Err(e) => {
            model_to_router_output!(task_model::UpdateTaskResponseModel {
                error: service_error_to_model!(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)),
            })
        },
    }
}

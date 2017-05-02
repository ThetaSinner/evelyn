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
use core::server_admin;
use core::error_messages::{EvelynBaseError, EvelynCoreError, EvelynServiceError};

pub fn purge_processor(router_input: RouterInput, processor_data: Arc<processing::ProcessorData>) -> RouterOutput {
    let request_model_de: Result<model::server_admin::PurgeRequestModel,_> = serde_json::from_str(&router_input.request_body);

    match request_model_de {
        Ok(request_model) => {
            let result = match request_model.target.as_str() {
                "all" => server_admin::purge_all(processor_data),
                "simpletask" => server_admin::purge_simple_task(processor_data),
                "todolist" => server_admin::purge_todo_list(processor_data),
                "calendar" => server_admin::purge_calendar(processor_data),
                _ =>
                    Some(EvelynCoreError::FailedToAcquirePurgeTarget(EvelynBaseError::NothingElse))
            };

            let model : model::server_admin::PurgeResponseModel = match result {
                None => model::server_admin::PurgeResponseModel { error : None },
                Some(error) =>
                model::server_admin::PurgeResponseModel {
                    error : Some(model::ErrorModel::from(EvelynServiceError::FailedToPurge(error)))
                }
            };

            RouterOutput { response_body : serde_json::to_string( &model).unwrap() }
        },
        Err(e) => {
            let response = model::server_admin::PurgeResponseModel {
                error: Some(From::from(EvelynServiceError::CouldNotDecodeTheRequestPayload(e)))
            };

            RouterOutput{response_body: serde_json::to_string(&response).unwrap()}
        }
    }
}

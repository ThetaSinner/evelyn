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

use core::error_messages::{EvelynBaseError, EvelynServiceError};
use core::server_admin;
use model;
use processing;
use serde_json;
use server::routing::{RouterInput, RouterOutput};
use std::sync::Arc;

pub fn purge_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    let request_model_de: Result<model::server_admin::PurgeRequestModel, _> = serde_json::from_str(&router_input.request_body);

    // TODO check token, but need some way to mark users as privileged.

    match request_model_de {
        Ok(request_model) => {
            let error = match request_model.target_type.as_str() {
                "database" => {
                    match server_admin::purge_database(processor_data) {
                        None => None,
                        Some(e) => Some(EvelynServiceError::FailedToPurge(e)),
                    }
                },
                "database_area" => {
                    match server_admin::purge_database_area(&request_model.target, processor_data) {
                        None => None,
                        Some(e) => Some(EvelynServiceError::FailedToPurge(e)),
                    }
                },
                _ => Some(EvelynServiceError::InvalidPurgeTargetType(EvelynBaseError::NothingElse)),
            };

            match error {
                None => {
                    RouterOutput {
                        response_body: serde_json::to_string(&model::server_admin::PurgeResponseModel {
                                                                 error: None,
                                                             })
                                .unwrap(),
                    }
                },
                Some(e) => {
                    RouterOutput {
                        response_body: serde_json::to_string(&model::server_admin::PurgeResponseModel {
                                                                 error: Some(From::from(e)),
                                                             })
                                .unwrap(),
                    }
                },
            }

        },
        Err(e) => {
            let response = model::server_admin::PurgeResponseModel {
                error: Some(From::from(EvelynServiceError::CouldNotDecodeTheRequestPayload(e))),
            };

            RouterOutput {
                response_body: serde_json::to_string(&response).unwrap(),
            }
        },
    }
}

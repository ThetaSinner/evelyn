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

use core::calendar;
use core::error_messages::{EvelynBaseError, EvelynServiceError};
use model;
use model::calendar as calendar_model;
use processing;
use serde_json;
use server::routing::{RouterInput, RouterOutput};
use std::sync::Arc;

pub fn calendar_add_event_processor(
    router_input: RouterInput,
    processor_data: Arc<processing::ProcessorData>,
) -> RouterOutput {
    let request_model_de: Result<calendar_model::CalendarAddEventRequestModel, _> = serde_json::from_str(&router_input.request_body);

    match request_model_de {
        Ok(request_model) => {
            let session_token_model = validate_session!(processor_data, request_model);

            match calendar::calendar_add_event(request_model, session_token_model, processor_data) {
                None => {
                    RouterOutput {
                        response_body: serde_json::to_string(&calendar_model::CalendarAddEventResponseModel {
                                                                 error: None,
                                                             })
                                .unwrap(),
                    }
                },
                Some(e) => {
                    RouterOutput {
                        response_body: serde_json::to_string(&calendar_model::CalendarAddEventResponseModel {
                                                                 error: Some(From::from(EvelynServiceError::AddCalendarEvent(e))),
                                                             })
                                .unwrap(),
                    }
                },
            }
        },
        Err(e) => {
            trace!("{}", e);

            let response = calendar_model::CalendarAddEventResponseModel {
                error: Some(From::from(EvelynServiceError::CouldNotDecodeTheRequestPayload(e))),
            };

            RouterOutput {
                response_body: serde_json::to_string(&response).unwrap(),
            }
        },
    }
}

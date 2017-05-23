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

#[macro_export]
macro_rules! validate_session {
    ($processor_data:expr, $model:expr) => {{
        let session_token_model = $processor_data.token_service.extract_session_token(&$model.token);

        if session_token_model.server_session_token != $processor_data.server_session_token {
            return RouterOutput{
                response_body: serde_json::to_string(&model::ErrorResponseModel {
                    error: From::from(EvelynServiceError::ForeignSessionToken(EvelynBaseError::NothingElse)),
                }).unwrap()
            }
        }

        session_token_model
    }};
}

#[macro_export]
macro_rules! service_error_to_model {
    ($service_error:expr) => {{
        Some(From::from($service_error))
    }};
}

#[macro_export]
macro_rules! model_to_router_output {
    ($model:expr) => {{
        RouterOutput {
            response_body: serde_json::to_string(&$model).unwrap()
        }
    }};
}

#[macro_export]
macro_rules! decode_router_input_to_model {
    ($target_model:path, $router_input:expr) => {{
        let request_model_decoded: Result<$target_model,_> = serde_json::from_str(&$router_input.request_body);
        request_model_decoded
    }};
}

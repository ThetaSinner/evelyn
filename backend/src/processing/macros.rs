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

#[macro_export]
macro_rules! validate_session {
    ($processor_data:expr, $model:expr) => {{
        let session_token_model = $processor_data.token_service.extract_session_token(&$model.token);

        if session_token_model.server_session_token != $processor_data.server_session_token {
            return RouterOutput{
                response_body: serde_json::to_string(&model::ErrorResponseModel {
                    error: From::from(EvelynServiceError::ForeignSessionToken),
                }).unwrap()
            }
        }

        session_token_model
    }};
}

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

pub mod user;
pub mod user_group;
pub mod simple_task;
pub mod todo_list;
pub mod calendar;
pub mod server_admin;
pub mod agile;

use core::error_messages;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorResponseModel {
    pub error: ErrorModel,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorModel {
    pub error_code: String,
    pub error_message: String,
}

impl From<error_messages::EvelynServiceError> for ErrorModel {
    fn from(error: error_messages::EvelynServiceError) -> Self {
        ErrorModel {
            error_code: format!("{}", error),
            error_message: String::from(error.description()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionTokenModel {
    pub user_id: String,

    pub server_session_token: String,
}

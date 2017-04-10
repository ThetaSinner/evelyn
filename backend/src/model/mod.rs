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

pub mod user;
pub mod simple_task;

use std::error::Error;

use core::error_messages;

#[derive(Serialize, Deserialize)]
pub struct ErrorModel {
    #[serde(rename="ErrorCode")]
    pub error_code: String,

    #[serde(rename="ErrorMessage")]
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

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SessionTokenModel {
    pub user_id: String,
}

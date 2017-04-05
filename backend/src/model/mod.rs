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

use std::error::Error;

use core::error_messages;

#[derive(Serialize, Deserialize)]
pub struct TestModel {
    pub name: Option<String>,
    pub hello: String,
}

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

#[derive(Serialize, Deserialize)]
pub struct CreateUserModel {
    #[serde(rename="UserName")]
    pub user_name: String,

    #[serde(rename="EmailAddress")]
    pub email_address: String,

    #[serde(rename="Password")]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponseModel {
    #[serde(rename="Error")]
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
pub struct LogonUserModel {
    #[serde(rename="EmailAddress")]
    pub email_address: String,

    #[serde(rename="Password")]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogonUserResponseModel {
    #[serde(rename="Token")]
    pub token: Option<String>,

    #[serde(rename="Error")]
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
pub struct UserModel {
  #[serde(rename="userName")]
  pub user_name: String,

  #[serde(rename="emailAddress")]
  pub email_address: String,

  #[serde(rename="password")]
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateSimpleTaskModel {
    #[serde(rename="Token")]
    pub token: String,

    #[serde(rename="Title")]
    pub title: String,

    #[serde(rename="Description")]
    pub description: String,

    #[serde(rename="DueDate")]
    pub due_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateSimpleTaskResponseModel {
    #[serde(rename="Error")]
    pub error: Option<ErrorModel>,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleTaskModel {
    #[serde(rename="userId")]
    pub user_id: String,

    #[serde(rename="title")]
    pub title: String,

    #[serde(rename="description")]
    pub description: String,

    #[serde(rename="dueDate")]
    pub due_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct LookupSimpleTaskRequestModel {
    #[serde(rename="Token")]
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct LookupSimpleTaskResponseModel {
    #[serde(rename="Error")]
    pub error: Option<ErrorModel>,

    #[serde(rename="SimpleTasks")]
    pub tasks: Vec<SimpleTaskModel>,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleTaskLookupModel {
    #[serde(rename="UserId")]
    pub user_id: String,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct SessionTokenModel {
    pub user_id: String,
}

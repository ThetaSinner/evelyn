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

#[derive(Serialize, Deserialize)]
pub struct TestModel {
    pub name: Option<String>,
    pub hello: String,
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
pub struct LogonUserModel {
    #[serde(rename="EmailAddress")]
    pub email_address: String,

    #[serde(rename="Password")]
    pub password: String,
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

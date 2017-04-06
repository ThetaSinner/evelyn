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

use model::{CreateUserModel, LogonUserModel, UserModel};
use processing::ProcessorData;
use model;

pub fn create_user(model: CreateUserModel, processor_data: Arc<ProcessorData>) -> model::CreateUserResponseModel {
  let user_model = UserModel{user_name: model.user_name, email_address: model.email_address, password: model.password};

  let ds = processor_data.data_store.clone();
  let mut data_store = ds.lock().unwrap();

  let user = data_store.find_user(&user_model.email_address);
  if user.is_some() {
    model::CreateUserResponseModel{
        error: Some(model::ErrorModel{
            error_code: "100001".to_owned(),
            error_message: "User already exists".to_owned()
        })
    }
  }
  else {
    let error = data_store.insert_user(&user_model);
    if error.is_some() {
        println!("Failed to insert user {}", error.unwrap());
        model::CreateUserResponseModel{error:None} // TODO temp
    }
    else {
        model::CreateUserResponseModel{error:None}
    }
  }
}

pub fn logon_user(model: LogonUserModel, processor_data: Arc<ProcessorData>) -> model::LogonUserResponseModel {
  let user: Option<UserModel>;
  {
      let ds = processor_data.data_store.clone();
      let mut data_store = ds.lock().unwrap();
      user = data_store.find_user(&model.email_address);
  }

  let mut response = None;
  if user.is_some() {
      let user = user.unwrap();
      if user.password == model.password {
          response = Some(model::LogonUserResponseModel {
              token: Some(processor_data.token_service.create_session_token(&user)),
              error: None
          });
      }
  }

  if !response.is_some() {
      response = Some(model::LogonUserResponseModel {
          token: None,
          error: Some(model::ErrorModel {
              error_code: "101001".to_owned(),
              error_message: "Invalid logon".to_owned()
          })
      });
  }

  response.unwrap()
}
